
advent_of_code::solution!(4);

#[derive(Debug, Clone, Copy)]
struct Word {
    start: (u32, u32),
    direction: (i8, i8),
}

impl Word {
    const fn candidates_from(x: u32, y: u32) -> [Word; 8] {
        [
            Word {
                start: (x, y),
                direction: (1, 0),
            }, // r
            Word {
                start: (x, y),
                direction: (-1, 0),
            }, // l
            Word {
                start: (x, y),
                direction: (0, 1),
            }, // d
            Word {
                start: (x, y),
                direction: (0, -1),
            }, // u
            Word {
                start: (x, y),
                direction: (1, 1),
            }, // dr
            Word {
                start: (x, y),
                direction: (1, -1),
            }, // dl
            Word {
                start: (x, y),
                direction: (-1, 1),
            }, // ur
            Word {
                start: (x, y),
                direction: (-1, -1),
            },
        ] // ul
    }
    fn valid(&self, mat: &[&[u8]]) -> bool {
        let (mut x, mut y) = self.start;
        for letter in b"MAS" {
            x = {
                let Some(v) = x.checked_add_signed(self.direction.0 as i32) else {
                    return false;
                };
                v
            };
            y = {
                let Some(v) = y.checked_add_signed(self.direction.1 as i32) else {
                    return false;
                };
                v
            };
            let Some(l) = mat.get(y as usize).and_then(|line| line.get(x as usize)) else {
                return false;
            };
            if l != letter {
                return false;
            }
        }
        true
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut words = vec![];
    let mat: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    for (y, l) in mat.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c == b'X' {
                let mut new_words = Word::candidates_from(x as u32, y as u32)
                    .into_iter()
                    .filter(|w| w.valid(&mat))
                    .collect();
                words.append(&mut new_words);
            };
        }
    }
    Some(words.len() as u32)
}

struct Cross(u32, u32);

impl Cross {
    fn from_mat(x: u32, y: u32, mat: &[&[u8]]) -> Option<Cross> {
        let get = |xoff, yoff| {
            mat.get(y.checked_add_signed(yoff)? as usize)?
                .get(x.checked_add_signed(xoff)? as usize)
        };
        if mat[y as usize][x as usize] != b'A' {
            return None;
        };
        match get(-1, -1)? {
            b'M' => {
                if *get(1, 1)? != b'S' {
                    return None;
                }
            }
            b'S' => {
                if *get(1, 1)? != b'M' {
                    return None;
                }
            }
            _ => return None,
        };
        match get(1, -1)? {
            b'M' => {
                if *get(-1, 1)? != b'S' {
                    return None;
                }
            }
            b'S' => {
                if *get(-1, 1)? != b'M' {
                    return None;
                }
            }
            _ => return None,
        };
        return Some(Cross(x, y));
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut crosses = vec![];
    let mat: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    for (y, line) in input.lines().enumerate() {
        for (x, _) in line.as_bytes().into_iter().enumerate() {
            if let Some(cross) = Cross::from_mat(x as u32, y as u32, &mat) {
                crosses.push(cross)
            }
        }
    }
    Some(crosses.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
