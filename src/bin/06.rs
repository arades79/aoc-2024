use std::collections::HashSet;

use tokio::task::JoinSet;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    const fn rotate(self) -> Self {
        match self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty(bool),
    Obstacle,
    Guard(Direction),
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty(false)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Guard {
    position: (usize, usize),
    direction: Direction,
}

impl Guard {
    const fn next_pos(&self) -> Option<(usize, usize)> {
        let (x, y) = self.position;
        match self.direction {
            Direction::Left => Some((
                match x.checked_sub(1) {
                    Some(x) => x,
                    None => return None,
                },
                y,
            )),
            Direction::Right => Some((x + 1, y)),
            Direction::Down => Some((x, y + 1)),
            Direction::Up => Some((
                x,
                match y.checked_sub(1) {
                    Some(y) => y,
                    None => return None,
                },
            )),
        }
    }
    const fn rotate(&mut self) {
        self.direction = self.direction.rotate()
    }
}

const fn parse_tile(c: char) -> Tile {
    match c {
        '.' => Tile::Empty(false),
        '#' => Tile::Obstacle,
        'v' => Tile::Guard(Direction::Down),
        '^' => Tile::Guard(Direction::Up),
        '<' => Tile::Guard(Direction::Left),
        '>' => Tile::Guard(Direction::Right),
        _ => panic!("input shouldn't have this character!"),
    }
}

type Map = Box<[Box<[Tile]>]>;

fn parse_map(input: &str) -> Map {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(parse_tile)
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn find_guard(map: &Map) -> Option<Guard> {
    for (y, tiles) in map.iter().enumerate() {
        for (x, tile) in tiles.iter().enumerate() {
            if let Tile::Guard(dir) = tile {
                return Some(Guard {
                    position: (x, y),
                    direction: *dir,
                });
            }
        }
    }
    None
}

const fn iterate_map(map: &mut Map, mut guard: Guard) -> Option<Guard> {
    map[guard.position.1][guard.position.0] = Tile::Empty(true);
    let (mut x, mut y) = match guard.next_pos() {
        Some((x, y)) => (x,y),
        None => return None,
    };
    loop {
        if y >= map.len() || x >= map[0].len() {return None};
        match map[y][x] {
            Tile::Empty(_) => {
                map[y][x] = Tile::Guard(guard.direction);
                break;
            }
            Tile::Obstacle => {
                guard.rotate();
                (x, y) = match guard.next_pos() {
                    Some((x, y)) => (x,y),
                    None => return None,
                };
            }
            Tile::Guard(_) => panic!("*spiderman pointing at spiderman*"),
        }
    }
    Some(Guard {
        position: (x, y),
        direction: guard.direction,
    })
}

fn detect_loop(map: &mut Map, mut guard: Guard, mut guard_states: HashSet<Guard>) -> bool {
    guard_states.insert(guard);
    while let Some(new_guard) = iterate_map(map, guard) {
        if guard_states.contains(&new_guard) {
            return true;
        }
        guard = new_guard;
        guard_states.insert(guard);
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse_map(input);
    let mut guard = find_guard(&map)?;
    while let Some(new_guard) = iterate_map(&mut map, guard) {
        guard = new_guard
    }
    let traversed = map.iter().flatten().fold(0, |traversed, t| {
        if *t == Tile::Empty(true) {
            traversed + 1
        } else {
            traversed
        }
    });
    Some(traversed)
}

#[tokio::main]
pub async fn part_two(input: &str) -> Option<u32> {
    let initial_map = parse_map(input);
    let initial_guard = find_guard(&initial_map)?;
    let mut map = initial_map.clone();
    let mut guard = initial_guard;
    let mut obstructed_locations = HashSet::new();
    let mut guard_states = HashSet::new();
    let mut tasks = JoinSet::new();
    while let Some(new_guard) = iterate_map(&mut map, guard) {
        guard_states.insert(guard);
        if !obstructed_locations.contains(&new_guard.position) {
            {
                let alt_guard_states = guard_states.clone();
                let mut alt_map = map.clone();
                alt_map[guard.position.1][guard.position.0] = Tile::Guard(guard.direction);
                alt_map[new_guard.position.1][new_guard.position.0] = Tile::Obstacle;
                tasks.spawn_blocking(move || {
                    detect_loop(&mut alt_map, guard, alt_guard_states) as u32
                });
            }
            obstructed_locations.insert(new_guard.position);
        }
        guard = new_guard;
    }
    let loops = tasks.join_all().await.into_iter().sum();
    Some(loops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
