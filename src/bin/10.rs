advent_of_code::solution!(10);

use std::collections::{HashMap, HashSet};

use rayon::prelude::*;

fn get_trailheads(map: &[&[u8]]) -> Vec<(usize, usize)> {
    map.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, h)| **h == b'0')
                .map(move |(x, _)| (x, y))
        })
        .collect()
}

const fn wanted_neighbors(coord: (usize, usize), level: u8) -> [(usize, usize, u8); 4] {
    [
        (coord.0.saturating_sub(1), coord.1, level + 1),
        (coord.0, coord.1.saturating_sub(1), level + 1),
        (coord.0 + 1, coord.1, level + 1),
        (coord.0, coord.1 + 1, level + 1),
    ]
}

fn calculate_trail_score(trailhead: (usize, usize), map: &[&[u8]]) -> u32 {
    let mut need_to_check = Vec::from(wanted_neighbors(trailhead, b'0'));
    let mut checked = HashSet::new();
    let mut trails_discovered = 0;
    while !need_to_check.is_empty() {
        let next_round: Vec<_> = need_to_check
            .drain(..)
            .filter_map(|(x, y, h)| {
                if checked.contains(&(x, y)) {
                    return None;
                }
                let height = map.get(y)?.get(x)?;
                if *height == h {
                    checked.insert((x, y));
                    if h == b'9' {
                        trails_discovered += 1;
                        None
                    } else {
                        Some(wanted_neighbors((x, y), h))
                    }
                } else {
                    None
                }
            })
            .flatten()
            .collect();
        need_to_check.extend(next_round);
    }

    trails_discovered
}

fn calculate_trail_rating(trailhead: (usize, usize), map: &[&[u8]]) -> u32 {
    let mut need_to_check = Vec::from(wanted_neighbors(trailhead, b'0'));
    let mut trails_discovered = 0;
    while !need_to_check.is_empty() {
        let next_round: Vec<_> = need_to_check
            .drain(..)
            .filter_map(|(x, y, h)| {
                let height = map.get(y)?.get(x)?;
                if *height == h {
                    if h == b'9' {
                        trails_discovered += 1;
                        None
                    } else {
                        Some(wanted_neighbors((x, y), h))
                    }
                } else {
                    None
                }
            })
            .flatten()
            .collect();
        need_to_check.extend(next_round);
    }

    trails_discovered
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let trailheads = get_trailheads(&map);
    let trail_score_sum = trailheads
        .into_par_iter()
        .map(|coord| calculate_trail_score(coord, &map))
        .sum();
    Some(trail_score_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let trailheads = get_trailheads(&map);
    let trail_score_sum = trailheads
        .into_par_iter()
        .map(|coord| calculate_trail_rating(coord, &map))
        .sum();
    Some(trail_score_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
