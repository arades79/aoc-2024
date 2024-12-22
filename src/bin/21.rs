use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(21);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KeyPadKey {
    A,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirKey {
    Up,
    Down,
    Left,
    Right,
    Push,
}

impl DirKey {
    const fn reverse(self) -> Self {
        match self {
            DirKey::Up => DirKey::Down,
            DirKey::Down => DirKey::Up,
            DirKey::Left => DirKey::Right,
            DirKey::Right => DirKey::Left,
            DirKey::Push => self,
        }
    }

    fn navigate(self, other: Self) -> Vec<DirKey> {
        use DirKey::*;
        let mut keys = match (self, other) {
            (Up, Down) => vec![Down],
            (Up, Left) => vec![Down, Left],
            (Up, Right) => vec![Right, Down],
            (Up, Push) => vec![Right],
            (Down, Left) => vec![Left],
            (Down, Right) => vec![Right],
            (Down, Push) => vec![Right, Up],
            (Left, Right) => vec![Right, Right],
            (Left, Push) => vec![Right, Right, Up],
            (Right, Push) => vec![Up],
            (a, b) if a == b => vec![],
            (a, b) => {
                let mut unroute: Vec<DirKey> =
                    b.navigate(a).into_iter().map(DirKey::reverse).collect();
                unroute.pop();
                unroute.reverse();
                unroute
            }
        };
        keys.push(Push);
        keys
    }
}

impl KeyPadKey {
    fn navigate(self, other: Self) -> Vec<DirKey> {
        use DirKey::*;
        use KeyPadKey::*;
        let mut directions = match (self, other) {
            (A, Zero) => vec![Left],
            (A, One) => vec![Up, Left, Left],
            (A, Two) => vec![Left, Up],
            (A, Three) => vec![Up],
            (A, Four) => vec![Up, Up, Left, Left],
            (A, Five) => vec![Left, Up, Up],
            (A, Six) => vec![Up, Up],
            (A, Seven) => vec![Up, Up, Up, Left, Left],
            (A, Eight) => vec![Left, Up, Up, Up],
            (A, Nine) => vec![Up, Up, Up],
            (Zero, One) => vec![Up, Left],
            (Zero, Two) => vec![Up],
            (Zero, Three) => vec![Up, Right],
            (Zero, Four) => vec![Up, Up, Left],
            (Zero, Five) => vec![Up, Up],
            (Zero, Six) => vec![Up, Up, Right],
            (Zero, Seven) => vec![Up, Up, Up, Left],
            (Zero, Eight) => vec![Up, Up, Up],
            (Zero, Nine) => vec![Up, Up, Up, Right],
            (One, Two) => vec![Right],
            (One, Three) => vec![Right, Right],
            (One, Four) => vec![Up],
            (One, Five) => vec![Up, Right],
            (One, Six) => vec![Up, Right, Right],
            (One, Seven) => vec![Up, Up],
            (One, Eight) => vec![Up, Up, Right],
            (One, Nine) => vec![Up, Up, Right, Right],
            (Two, Three) => vec![Right],
            (Two, Four) => vec![Left, Up],
            (Two, Five) => vec![Up],
            (Two, Six) => vec![Up, Right],
            (Two, Seven) => vec![Left, Up, Up],
            (Two, Eight) => vec![Up, Up],
            (Two, Nine) => vec![Up, Up, Right],
            (Three, Four) => vec![Left, Left, Up],
            (Three, Five) => vec![Left, Up],
            (Three, Six) => vec![Up],
            (Three, Seven) => vec![Left, Left, Up, Up],
            (Three, Eight) => vec![Left, Up, Up],
            (Three, Nine) => vec![Up, Up],
            (Four, Five) => vec![Right],
            (Four, Six) => vec![Right, Right],
            (Four, Seven) => vec![Up],
            (Four, Eight) => vec![Up, Right],
            (Four, Nine) => vec![Up, Right, Right],
            (Five, Six) => vec![Right],
            (Five, Seven) => vec![Left, Up],
            (Five, Eight) => vec![Up],
            (Five, Nine) => vec![Right, Up],
            (Six, Seven) => vec![Left, Left, Up],
            (Six, Eight) => vec![Left, Up],
            (Six, Nine) => vec![Up],
            (Seven, Eight) => vec![Right],
            (Seven, Nine) => vec![Right, Right],
            (Eight, Nine) => vec![Right],
            (a, b) if a == b => vec![],
            (a, b) => {
                let mut unroute: Vec<DirKey> =
                    b.navigate(a).into_iter().map(DirKey::reverse).collect();
                unroute.pop();
                unroute.reverse();
                unroute
            }
        };
        directions.push(Push);
        directions
    }
}

fn combo_from_string(input: &str) -> (Vec<KeyPadKey>, usize) {
    let mut keys = vec![KeyPadKey::A];
    let mut combonum = 0;
    for c in input.trim().chars() {
        if let Some(num) = c.to_digit(10) {
            combonum = (combonum * 10) + num;
            keys.push(match num {
                0 => KeyPadKey::Zero,
                1 => KeyPadKey::One,
                2 => KeyPadKey::Two,
                3 => KeyPadKey::Three,
                4 => KeyPadKey::Four,
                5 => KeyPadKey::Five,
                6 => KeyPadKey::Six,
                7 => KeyPadKey::Seven,
                8 => KeyPadKey::Eight,
                9 => KeyPadKey::Nine,
                _ => unreachable!(),
            });
        } else {
            keys.push(KeyPadKey::A);
        }
    }
    (keys, combonum as usize)
}

fn print_keycode(keycode: &str, keys: &[DirKey]) {
    print!("{keycode}: ");
    for key in keys.iter() {
        let c = match key {
            DirKey::Up => '^',
            DirKey::Down => 'v',
            DirKey::Left => '<',
            DirKey::Right => '>',
            DirKey::Push => 'A',
        };
        print!("{c}");
    }
    println!("");
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut complexity_scores = 0;
    for line in input.lines() {
        let (keys, num) = combo_from_string(line);
        // robots start pointing at A, it's not pressed, but used for navigation
        let mut robot_1_keys = vec![DirKey::Push];
        robot_1_keys.extend(keys.windows(2).flat_map(|key| key[0].navigate(key[1])));
        let mut robot_2_keys = vec![DirKey::Push];
        robot_2_keys.extend(
            robot_1_keys
                .windows(2)
                .flat_map(|key| key[0].navigate(key[1])),
        );
        let human_keys: Vec<_> = robot_2_keys
            .windows(2)
            .flat_map(|key| key[0].navigate(key[1]))
            .collect();
        print_keycode(line, &robot_1_keys);
        print_keycode(line, &robot_2_keys);
        print_keycode(line, &human_keys);
        let complexity = human_keys.len() * num;
        complexity_scores += complexity;
    }
    Some(complexity_scores)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut complexity_scores = 0;
    for line in input.lines() {
        let (keys, num) = combo_from_string(line);
        // robots start pointing at A, it's not pressed, but used for navigation
        let mut robot_1_keys = vec![DirKey::Push];
        robot_1_keys.extend(keys.windows(2).flat_map(|key| key[0].navigate(key[1])));
        for _ in 0..24 {
            let mut robot_2_keys = vec![DirKey::Push];
            robot_2_keys.extend(
                robot_1_keys
                    .windows(2)
                    .flat_map(|key| key[0].navigate(key[1])),
            );
            robot_1_keys = robot_2_keys;
        }
        let keypresses = robot_1_keys
            .into_iter()
            .tuple_windows()
            .flat_map(|(key1, key2)| key1.navigate(key2))
            .count();
        let complexity = keypresses * num;
        complexity_scores += complexity;
    }
    Some(complexity_scores)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
