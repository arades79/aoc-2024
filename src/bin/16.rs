use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use pathfinding::directed::{astar::astar_bag_collect, dijkstra::dijkstra};

advent_of_code::solution!(16);

type Coord = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
enum Direction {
    #[default]
    East,
    South,
    West,
    North,
}

impl Direction {
    const fn rotate(self, turn: Turn) -> Self {
        match (self, turn) {
            (Direction::East, Turn::Left) => Direction::North,
            (Direction::East, Turn::Right) => Direction::South,
            (Direction::South, Turn::Left) => Direction::East,
            (Direction::South, Turn::Right) => Direction::West,
            (Direction::West, Turn::Left) => Direction::South,
            (Direction::West, Turn::Right) => Direction::North,
            (Direction::North, Turn::Left) => Direction::West,
            (Direction::North, Turn::Right) => Direction::East,
            (_, Turn::Forward) => self,
        }
    }
    const fn apply_to_coord(self, (i, j): Coord) -> Coord {
        match self {
            Direction::East => (i + 1, j),
            Direction::South => (i, j + 1),
            Direction::West => (i - 1, j),
            Direction::North => (i, j - 1),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Deer {
    position: Coord,
    direction: Direction,
}

impl Deer {
    fn start(coord: Coord) -> Self {
        Self {
            position: coord,
            direction: Direction::East,
        }
    }
    fn apply_turn(&mut self, turn: Turn) {
        if turn != Turn::Forward {
            self.direction = self.direction.rotate(turn);
        }
        self.position = self.direction.apply_to_coord(self.position)
    }
    fn front(&self) -> Coord {
        self.direction.apply_to_coord(self.position)
    }
    fn left(&self) -> Coord {
        self.direction
            .rotate(Turn::Left)
            .apply_to_coord(self.position)
    }
    fn right(&self) -> Coord {
        self.direction
            .rotate(Turn::Right)
            .apply_to_coord(self.position)
    }
    fn dist_to(&self, coord: Coord) -> u32 {
        (self.position.0.abs_diff(coord.0) + self.position.1.abs_diff(coord.1))
            .try_into()
            .unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Turn {
    Left,
    Right,
    Forward,
}

impl Turn {
    const fn weight(self) -> u32 {
        match self {
            Turn::Left | Turn::Right => 1001,
            Turn::Forward => 1,
        }
    }
}

fn find_start(maze: &[&[u8]]) -> Option<Deer> {
    maze.iter()
        .enumerate()
        .filter_map(|(j, row)| {
            row.iter()
                .find_position(|tile| **tile == b'S')
                .map(|(i, _)| (i, j))
        })
        .next()
        .map(Deer::start)
}

fn find_end(maze: &[&[u8]]) -> Option<Coord> {
    maze.iter()
        .enumerate()
        .filter_map(|(j, row)| {
            row.iter()
                .find_position(|tile| **tile == b'E')
                .map(|(i, _)| (i, j))
        })
        .next()
}

fn options(maze: &[&[u8]], deer: &Deer) -> Vec<(Deer, u32)> {
    let mut options = Vec::new();
    let empty = |(i, j): (usize, usize)| maze[j][i] == b'.' || maze[j][i] == b'E';
    if empty(deer.front()) {
        let mut nd = deer.clone();
        nd.apply_turn(Turn::Forward);
        options.push((nd, 1));
    }
    if empty(deer.left()) {
        let mut nd = deer.clone();
        nd.apply_turn(Turn::Left);
        options.push((nd, 1001));
    }
    if empty(deer.right()) {
        let mut nd = deer.clone();
        nd.apply_turn(Turn::Right);
        options.push((nd, 1001))
    }
    options
}

fn finished(maze: &[&[u8]], deer: &Deer) -> bool {
    let (i, j) = deer.position;
    maze[j][i] == b'E'
}

// fn check(maze: &[&[u8]], deer: Deer, route: &mut HashMap<Coord, u32>, full_paths: &mut Vec<u32>) {
//     for option in options(maze, &deer) {
//         let mut next_deer = deer.clone();
//         next_deer.apply_turn(option);
//         let score = route[&deer.position] + option.weight();
//         if let Some(old_score) = route.get(&next_deer.position) {
//             if *old_score < score {
//                 continue;
//             }
//         }
//         route.insert(next_deer.position, score);
//         if finished(maze, &next_deer) {
//             full_paths.push(route[&next_deer.position]);
//         } else {
//             check(maze, next_deer, route, full_paths);
//         }
//     }
// }

pub fn part_one(input: &str) -> Option<u32> {
    let maze: Vec<_> = input.lines().map(str::as_bytes).collect();
    let deer = find_start(&maze)?;
    let (_, score) = dijkstra(&deer, |nd| options(&maze, nd), |nd| finished(&maze, nd))?;
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let maze: Vec<_> = input.lines().map(str::as_bytes).collect();
    let deer = find_start(&maze)?;
    let end = find_end(&maze)?;
    let (paths, score) = astar_bag_collect(
        &deer,
        |nd| options(&maze, nd),
        |nd| nd.dist_to(end),
        |nd| finished(&maze, nd),
    )?;
    let winning_nodes: HashSet<Coord> = paths.concat().into_iter().map(|d| d.position).collect();
    Some(winning_nodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
