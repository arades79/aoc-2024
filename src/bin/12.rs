use std::collections::HashSet;

advent_of_code::solution!(12);

const fn neighbors(coord: (usize, usize)) -> [(usize, usize); 4] {
    [
        (coord.0.wrapping_sub(1), coord.1), // L
        (coord.0, coord.1.wrapping_sub(1)), // U
        (coord.0 + 1, coord.1),             // R
        (coord.0, coord.1 + 1),             // D
    ]
}

fn plot(
    (x, y): (usize, usize),
    plant: u8,
    map: &[&[u8]],
    visited: &mut HashSet<(usize, usize)>,
) -> Option<(u32, u32)> {
    if visited.contains(&(x, y)) {
        return None;
    }
    visited.insert((x, y));
    let mut area = 1;
    let mut perimiter = 0;
    for (nx, ny) in neighbors((x, y)) {
        if let Some(other_plant) = map.get(ny).and_then(|row| row.get(nx)) {
            if *other_plant == plant {
                if let Some((narea, nperim)) = plot((nx, ny), plant, map, visited) {
                    area += narea;
                    perimiter += nperim;
                }
            } else {
                perimiter += 1;
            }
        } else {
            perimiter += 1;
        }
    }
    Some((area, perimiter))
}

fn plot_corners(
    (x, y): (usize, usize),
    plant: u8,
    map: &[&[u8]],
    visited: &mut HashSet<(usize, usize)>,
) -> Option<(u32, u32)> {
    if visited.contains(&(x, y)) {
        return None;
    }
    visited.insert((x, y));
    let check = |mx, my| {
        map.get(my)
            .is_some_and(|row: &&[u8]| row.get(mx).is_some_and(|other_plant| *other_plant == plant))
    };
    let shape = neighbors((x, y)).map(|(nx, ny)| check(nx, ny));
    let mut area = 1;
    let lu = check(x.wrapping_sub(1), y.wrapping_sub(1));
    let ld = check(x.wrapping_sub(1), y + 1);
    let ur = check(x + 1, y.wrapping_sub(1));
    let dr = check(x + 1, y + 1);
    let mut corners = match shape {
        // left, up, right, down
        [true, true, false, false] => 1 + !lu as u32,
        [true, false, false, true] => 1 + !ld as u32,
        [true, false, false, false] => 2,
        [false, true, true, false] => 1 + !ur as u32,
        [false, true, false, false] => 2,
        [false, false, true, true] => 1 + !dr as u32,
        [false, false, true, false] => 2,
        [false, false, false, true] => 2,
        [false, false, false, false] => 4,
        [l, u, r, d] => {
            (l && u && !lu) as u32
                + (l && d && !ld) as u32
                + (u && r && !ur) as u32
                + (d && r && !dr) as u32
        }
    };
    // println!("location ({x},{y}) has {corners} corners");
    for (nx, ny) in neighbors((x, y)) {
        if let Some(other_plant) = map.get(ny).and_then(|row| row.get(nx)) {
            if *other_plant == plant {
                if let Some((narea, ncorners)) = plot_corners((nx, ny), plant, map, visited) {
                    area += narea;
                    corners += ncorners;
                }
            }
        }
    }
    Some((area, corners))
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let mut plotted = HashSet::new();
    let mut fence_price = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, plant) in line.iter().copied().enumerate() {
            let Some((area, perimiter)) = plot((x, y), plant, &map, &mut plotted) else {
                continue;
            };
            println!("found region of {plant} starting at ({x},{y}) with area {area} and perimiter {perimiter}");
            fence_price += area * perimiter
        }
    }
    Some(fence_price)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let mut plotted = HashSet::new();
    let mut fence_price = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, plant) in line.iter().copied().enumerate() {
            let Some((area, sides)) = plot_corners((x, y), plant, &map, &mut plotted) else {
                continue;
            };
            println!(
                "found region of {plant} starting at ({x},{y}) with area {area} and {sides} sides"
            );
            fence_price += area * sides
        }
    }
    Some(fence_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
