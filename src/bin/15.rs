use std::collections::HashSet;

advent_of_code::solution!(15);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Move {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
enum Tile {
    Wall,
    #[default]
    Empty,
    Box,
    Robot,
    WideBoxLeft,
    WideBoxRight,
}

impl Tile {
    const fn from_c(v: &u8) -> Option<Tile> {
        match v {
            b'#' => Some(Tile::Wall),
            b'.' => Some(Tile::Empty),
            b'@' => Some(Tile::Robot),
            b'O' => Some(Tile::Box),
            b'[' => Some(Tile::WideBoxLeft),
            b']' => Some(Tile::WideBoxRight),
            _ => None,
        }
    }
    const fn to_c(self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Empty => '.',
            Tile::Box => 'O',
            Tile::Robot => '@',
            Tile::WideBoxLeft => '[',
            Tile::WideBoxRight => ']',
        }
    }
}

type Map = Vec<Vec<Tile>>;

impl Move {
    const fn from_c(v: &u8) -> Option<Self> {
        match v {
            b'<' => Some(Move::Left),
            b'^' => Some(Move::Up),
            b'>' => Some(Move::Right),
            b'v' => Some(Move::Down),
            _ => None,
        }
    }
    const fn apply(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Move::Left => (x - 1, y),
            Move::Up => (x, y - 1),
            Move::Right => (x + 1, y),
            Move::Down => (x, y + 1),
        }
    }
    const fn is_horizontal(&self) -> bool {
        match self {
            Move::Left => true,
            Move::Up => false,
            Move::Right => true,
            Move::Down => false,
        }
    }
    const fn is_vertical(&self) -> bool {
        !self.is_horizontal()
    }
}

fn parse_input(input: &str) -> Option<(Map, Vec<Move>)> {
    let (map_str, dir_str) = input.split_once("\n\n")?;
    let map = map_str
        .lines()
        .map(|row| row.as_bytes().iter().filter_map(Tile::from_c).collect())
        .collect();
    let dir = dir_str.as_bytes().iter().filter_map(Move::from_c).collect();
    Some((map, dir))
}

fn parse_input2(input: &str) -> Option<(Map, Vec<Move>)> {
    let (map_str, dir_str) = input.split_once("\n\n")?;
    let map = map_str
        .lines()
        .map(|row| {
            let mut v = Vec::new();
            row.as_bytes()
                .iter()
                .for_each(|c| match Tile::from_c(c).unwrap() {
                    Tile::Wall => v.extend([Tile::Wall, Tile::Wall]),
                    Tile::Empty => v.extend([Tile::Empty, Tile::Empty]),
                    Tile::Box => v.extend([Tile::WideBoxLeft, Tile::WideBoxRight]),
                    Tile::Robot => v.extend([Tile::Robot, Tile::Empty]),
                    Tile::WideBoxLeft => unreachable!(),
                    Tile::WideBoxRight => unreachable!(),
                });
            v
        })
        .collect();
    let dir = dir_str.as_bytes().iter().filter_map(Move::from_c).collect();
    Some((map, dir))
}

fn find_robot(map: &Map) -> Option<(usize, usize)> {
    for (j, row) in map.iter().enumerate() {
        for (i, tile) in row.iter().enumerate() {
            if *tile == Tile::Robot {
                return Some((i, j));
            }
        }
    }
    None
}

fn at(map: &mut Map, (i, j): (usize, usize)) -> Option<&mut Tile> {
    map.get_mut(j)?.get_mut(i)
}

fn try_push(
    map: &mut Map,
    (i, j): (usize, usize),
    direction: Move,
    tile: Tile,
    pushing: &mut HashSet<(usize, usize)>,
) -> Option<()> {
    let neighbor = direction.apply((i, j));
    if pushing.contains(&(i, j)) {
        return Some(());
    } else {
        pushing.insert((i, j));
    }

    match map[neighbor.1][neighbor.0] {
        Tile::Wall => None,
        Tile::Empty => {
            *at(map, neighbor)? = tile;
            Some(())
        }
        Tile::Box => try_push(map, neighbor, direction, Tile::Box, pushing),
        Tile::Robot => None,
        Tile::WideBoxLeft => {
            *at(map, neighbor)? = tile;
            match direction {
                Move::Left | Move::Right => {
                    try_push(map, neighbor, direction, Tile::WideBoxLeft, pushing)
                }
                Move::Up | Move::Down => {
                    try_push(map, neighbor, direction, Tile::WideBoxLeft, pushing)?;
                    try_push(
                        map,
                        (neighbor.0 + 1, neighbor.1),
                        direction,
                        Tile::WideBoxRight,
                        pushing,
                    )
                }
            }
        }
        Tile::WideBoxRight => {
            *at(map, neighbor)? = tile;
            match direction {
                Move::Left | Move::Right => {
                    try_push(map, neighbor, direction, Tile::WideBoxRight, pushing)
                }
                Move::Up | Move::Down => {
                    try_push(map, neighbor, direction, Tile::WideBoxRight, pushing)?;
                    try_push(
                        map,
                        (neighbor.0 - 1, neighbor.1),
                        direction,
                        Tile::WideBoxLeft,
                        pushing,
                    )
                }
            }
        }
    }
}

fn step(map: &mut Map, robot: &mut (usize, usize), direction: Move) -> Option<()> {
    assert_eq!(map[robot.1][robot.0], Tile::Robot);
    let neighbor = direction.apply(*robot);
    match map[neighbor.1][neighbor.0] {
        Tile::Wall => return None,
        Tile::Empty => {}
        Tile::Box => {
            try_push(map, neighbor, direction, Tile::Box, &mut HashSet::new())?;
        }
        Tile::Robot => unreachable!(),
        Tile::WideBoxLeft => {
            let mut mymap = map.clone();
            try_push(
                &mut mymap,
                *robot,
                direction,
                Tile::WideBoxLeft,
                &mut HashSet::new(),
            )?;
            if direction.is_vertical() {
                *at(&mut mymap, (neighbor.0 + 1, neighbor.1))? = Tile::Empty;
            }
            cleanup(&mut mymap);
            *map = mymap;
        }
        Tile::WideBoxRight => {
            let mut mymap = map.clone();
            try_push(
                &mut mymap,
                *robot,
                direction,
                Tile::WideBoxRight,
                &mut HashSet::new(),
            )?;
            if direction.is_vertical() {
                *at(&mut mymap, (neighbor.0 - 1, neighbor.1))? = Tile::Empty;
            }
            cleanup(&mut mymap);
            *map = mymap;
        }
    };
    *at(map, neighbor)? = Tile::Robot;
    *at(map, *robot)? = Tile::Empty;
    *robot = neighbor;
    render(map.clone());
    Some(())
}

fn gps_total(map: &Map) -> u32 {
    let mut total = 0;
    for (j, row) in map.iter().enumerate() {
        for (i, tile) in row.iter().enumerate() {
            if *tile == Tile::Box || *tile == Tile::WideBoxLeft {
                total += (100 * j + i) as u32;
            }
        }
    }
    total
}

#[cfg(test)]
fn render(map: Map) {
    use std::io::Write;

    let mut s = String::new();
    for row in map {
        for t in row {
            s.push(t.to_c());
        }
        s.push('\n');
    }
    let mut f = std::fs::OpenOptions::new()
        .append(true)
        .open("robots/lantern.txt")
        .unwrap();
    f.write_all(s.as_bytes()).unwrap();
}

#[cfg(not(test))]
fn render(_map: Map) {}

fn cleanup(map: &mut Map) {
    for row in map {
        for i in 0..(row.len() - 1) {
            if row[i] == Tile::WideBoxLeft && row[i + 1] == Tile::WideBoxLeft {
                row[i] = Tile::Empty;
            }
            if row[i] == Tile::WideBoxRight && row[i + 1] == Tile::WideBoxRight {
                row[i + 1] = Tile::Empty;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, dirs) = parse_input(input)?;
    let mut robot = find_robot(&map)?;
    for direction in dirs {
        step(&mut map, &mut robot, direction);
    }
    let score = gps_total(&map);
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    std::fs::File::create("robots/lantern.txt").ok()?;
    let (mut map, dirs) = parse_input2(input)?;
    let mut robot = find_robot(&map)?;
    for direction in dirs {
        step(&mut map, &mut robot, direction);
    }
    let score = gps_total(&map);
    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
