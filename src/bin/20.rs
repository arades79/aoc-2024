use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use pathfinding::prelude::*;

advent_of_code::solution!(20);

type Map<'a> = &'a [&'a [u8]];
type Point = (usize, usize);

fn find(maze: &[&[u8]], crumb: u8) -> Option<Point> {
    maze.iter()
        .enumerate()
        .filter_map(|(j, row)| {
            row.iter()
                .find_position(|tile| **tile == crumb)
                .map(|(i, _)| (i, j))
        })
        .next()
}

fn find_start(maze: &[&[u8]]) -> Option<Point> {
    find(maze, b'S')
}

fn find_end(maze: &[&[u8]]) -> Option<Point> {
    find(maze, b'E')
}

const fn n_away(coord: Point, n: usize) -> [Point; 4] {
    [
        (coord.0.wrapping_sub(n), coord.1),
        (coord.0, coord.1.wrapping_sub(n)),
        (coord.0.saturating_add(n), coord.1),
        (coord.0, coord.1.saturating_add(n)),
    ]
}

const fn neighbors(coord: Point) -> [Point; 4] {
    n_away(coord, 1)
}

fn successors(maze: Map, point: &Point) -> Vec<(Point, u32)> {
    let mut options = Vec::new();
    let empty = |(i, j): (usize, usize)| maze[j][i] == b'.' || maze[j][i] == b'E';
    for point in neighbors(*point) {
        if empty(point) {
            options.push((point, 1));
        }
    }
    options
}

#[cfg(test)]
const THRESHOLD: u32 = 50;
#[cfg(not(test))]
const THRESHOLD: u32 = 100;

pub fn part_one(input: &str) -> Option<u32> {
    let maze: Vec<_> = input.lines().map(str::as_bytes).collect();
    let start = find_start(&maze)?;
    let mut distance_map: HashMap<_, _> = dijkstra_all(&start, |nd| successors(&maze, nd))
        .into_iter()
        .map(|(node, (_parent, score))| (node, score))
        .collect();
    distance_map.insert(start, 0);
    let mut shortcuts = 0;
    for (point, score) in distance_map.iter() {
        for cheat in n_away(*point, 2) {
            if let Some(score2) = distance_map.get(&cheat) {
                let diff = score2.saturating_sub(*score + 2);
                if diff >= THRESHOLD {
                    shortcuts += 1;
                }
            }
        }
    }
    Some(shortcuts)
}

pub fn part_two(input: &str) -> Option<u64> {
    let maze: Vec<_> = input.lines().map(str::as_bytes).collect();
    let start = find_start(&maze)?;
    let mut distance_map: HashMap<_, _> = dijkstra_all(&start, |nd| successors(&maze, nd))
        .into_iter()
        .map(|(node, (_parent, score))| (node, score))
        .collect();
    distance_map.insert(start, 0);
    let mut shortcuts = 0;
    let mut shortcuts_scores = HashMap::new();
    for (point, score) in distance_map.iter() {
        let mut checked: HashSet<(usize, usize)> = HashSet::new();
        checked.insert(*point);
        for n in 0..20 {
            for neighbor in checked.clone() {
                for cheat in neighbors(neighbor) {
                    if !checked.insert(cheat) {
                        continue;
                    }
                    if let Some(score2) = distance_map.get(&cheat) {
                        let diff = score2.saturating_sub(*score + n);
                        if diff >= THRESHOLD {
                            shortcuts += 1;
                            shortcuts_scores
                                .entry(diff)
                                .and_modify(|v| *v += 1)
                                .or_insert(1);
                        }
                    }
                }
            }
        }
    }
    shortcuts_scores
        .into_iter()
        .sorted()
        .for_each(|v| println!("{v:?}"));
    Some(shortcuts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(285));
    }
}
