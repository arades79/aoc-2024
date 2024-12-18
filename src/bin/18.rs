advent_of_code::solution!(18);

use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use winnow::{ascii::dec_uint, combinator::separated_pair, prelude::*};

type Point = (u32, u32);

const START: Point = (0, 0);
#[cfg(test)]
const EXIT: Point = (6, 6);
#[cfg(not(test))]
const EXIT: Point = (70, 70);

fn point_parser(input: &mut &str) -> PResult<Point> {
    separated_pair(dec_uint, ",", dec_uint).parse_next(input)
}

const fn neighbors(coord: Point) -> [Point; 4] {
    [
        (coord.0.saturating_sub(1), coord.1),
        (coord.0, coord.1.saturating_sub(1)),
        (coord.0 + 1, coord.1),
        (coord.0, coord.1 + 1),
    ]
}

fn find_exit(corrupted: &HashSet<Point>) -> Option<u32> {
    let mut score = 0;
    let mut traversed: HashMap<Point, u32> = HashMap::from([(START, score)]);
    let mut to_check = vec![START];
    while !traversed.contains_key(&EXIT) && !to_check.is_empty() {
        score += 1;
        for next in to_check.split_off(0).into_iter().flat_map(neighbors) {
            if !(START.0..=EXIT.0).contains(&next.0)
                || !(START.1..=EXIT.1).contains(&next.1)
                || corrupted.contains(&next)
                || traversed.contains_key(&next)
            {
                continue;
            }
            traversed.insert(next, score);
            to_check.push(next);
        }
    }
    traversed.remove(&EXIT)
}

pub fn part_one(input: &str) -> Option<u32> {
    #[cfg(test)]
    const STEPS: usize = 12;
    #[cfg(not(test))]
    const STEPS: usize = 1024;
    let corrupted: HashSet<Point> = input
        .lines()
        .filter_map(|line| point_parser.parse(line).ok())
        .take(STEPS)
        .collect();
    find_exit(&corrupted)
}

pub fn part_two(input: &str) -> Option<String> {
    #[cfg(test)]
    const STEPS: usize = 12;
    #[cfg(not(test))]
    const STEPS: usize = 1024;
    let all_corrupted: Vec<Point> = input
        .lines()
        .filter_map(|line| point_parser.parse(line).ok())
        .collect();
    let mut corrupted: HashSet<Point> = HashSet::from_iter(all_corrupted[0..STEPS].iter().copied());
    let mut next_corrupt = STEPS;
    while find_exit(&corrupted).is_some() {
        corrupted.insert(all_corrupted[next_corrupt]);
        next_corrupt += 1;
    }
    let (x, y) = all_corrupted[next_corrupt - 1];
    Some(format!("{x},{y}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
