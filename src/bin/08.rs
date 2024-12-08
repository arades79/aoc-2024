use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Div, Sub},
    sync::OnceLock,
};

use gcd::Gcd;
use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Point(i32, i32);

impl Point {
    fn fits(self) -> bool {
        let max = MAX.get().unwrap();
        self.0 >= 0 && self.1 >= 0 && self.0 < max.0 && self.1 < max.1
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Div for Point {
    type Output = Point;

    fn div(self, rhs: Self) -> Self::Output {
        Point(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl Div<i32> for Point {
    type Output = Point;

    fn div(self, rhs: i32) -> Self::Output {
        Point(self.0 / rhs, self.1 / rhs)
    }
}

static MAX: OnceLock<(i32, i32)> = OnceLock::new();

fn get_antennas(input: &str) -> HashMap<char, Vec<Point>> {
    MAX.set(get_max(input)).ok();
    let mut antennas = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            if c != '.' {
                let p = Point(x as i32, y as i32);
                antennas
                    .entry(c)
                    .and_modify(|v: &mut Vec<Point>| v.push(p))
                    .or_insert(vec![p]);
            }
        }
    }
    antennas
}

fn to_antinodes(p1: Point, p2: Point) -> [Point; 2] {
    let dist = p2 - p1;
    [p1 - dist, p2 + dist]
}

fn to_antinodes_part2(p1: Point, p2: Point) -> Vec<Point> {
    let simple_dist = p2 - p1;
    //let simple_dist = dist / Gcd::gcd(dist.0 as u32, dist.1 as u32) as i32;
    let mut temp_p = p1;
    let mut antinodes = Vec::new();
    while (temp_p - simple_dist).fits() {
        temp_p = temp_p - simple_dist;
        antinodes.push(temp_p);
    }
    temp_p = p1;
    antinodes.push(temp_p);
    while (temp_p + simple_dist).fits() {
        temp_p = temp_p + simple_dist;

        antinodes.push(temp_p);
    }
    antinodes
}

fn get_max(input: &str) -> (i32, i32) {
    let mut lines = input.lines();
    let x = lines.next().unwrap().len();
    let y = lines.count() + 1;

    (x as i32, y as i32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut unique_antinodes = HashSet::new();
    let antennas = get_antennas(input);
    dbg!(&antennas);
    for nodes in antennas.into_values() {
        for (p1, p2) in nodes.into_iter().tuple_combinations() {
            let [an1, an2] = to_antinodes(p1, p2);
            if an1.fits() {
                unique_antinodes.insert(an1);
            }
            if an2.fits() {
                unique_antinodes.insert(an2);
            }
        }
    }
    dbg!(&unique_antinodes);
    Some(unique_antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut unique_antinodes = HashSet::new();
    let antennas = get_antennas(input);
    for nodes in antennas.into_values() {
        for (p1, p2) in nodes.into_iter().tuple_combinations() {
            let antinodes = to_antinodes_part2(p1, p2);
            unique_antinodes.extend(antinodes.into_iter());
        }
    }
    Some(unique_antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
