advent_of_code::solution!(14);

use itertools::Itertools;
use winnow::{
    ascii::{dec_int, dec_uint},
    combinator::{separated_pair, seq},
    prelude::*,
};

use image::{save_buffer, ExtendedColorType};

#[cfg(test)]
const WIDTH: u32 = 11;

#[cfg(test)]
const HEIGHT: u32 = 7;

#[cfg(not(test))]
const WIDTH: u32 = 101;
#[cfg(not(test))]
const HEIGHT: u32 = 103;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Robot {
    pos: (u32, u32),
    v: (i32, i32),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Quadrant {
    Q1,
    Q2,
    Q3,
    Q4,
}

impl From<Quadrant> for usize {
    fn from(value: Quadrant) -> Self {
        match value {
            Quadrant::Q1 => 0,
            Quadrant::Q2 => 1,
            Quadrant::Q3 => 2,
            Quadrant::Q4 => 3,
        }
    }
}

impl Robot {
    const fn new(x: u32, y: u32, vx: i32, vy: i32) -> Self {
        Robot {
            pos: (x, y),
            v: (vx, vy),
        }
    }
    const fn quadrant(&self) -> Option<Quadrant> {
        const Q1W: u32 = WIDTH / 2;
        const Q2W: u32 = Q1W + 1;
        const Q1H: u32 = HEIGHT / 2;
        const Q3H: u32 = Q1H + 1;
        match self.pos {
            (0..Q1W, 0..Q1H) => Some(Quadrant::Q1),
            (Q2W..WIDTH, 0..Q1H) => Some(Quadrant::Q2),
            (0..Q1W, Q3H..HEIGHT) => Some(Quadrant::Q3),
            (Q2W..WIDTH, Q3H..HEIGHT) => Some(Quadrant::Q4),
            _ => None,
        }
    }
    const fn step_n(&self, n: i32) -> Self {
        let (mut step_x, mut step_y) = (self.v.0 * n, self.v.1 * n);
        if step_x.is_negative() {
            step_x += (step_x.abs().div_euclid(WIDTH as i32) + 1) * WIDTH as i32;
        }
        if step_y.is_negative() {
            step_y += (step_y.abs().div_euclid(HEIGHT as i32) + 1) * HEIGHT as i32;
        }
        let (new_x, new_y) = (
            self.pos.0.wrapping_add_signed(step_x) % WIDTH,
            self.pos.1.wrapping_add_signed(step_y) % HEIGHT,
        );
        Self::new(new_x, new_y, self.v.0, self.v.1)
    }
}

fn robot_parser(input: &mut &str) -> PResult<Robot> {
    seq! {Robot{
        _: "p=",
        pos: separated_pair(dec_uint, ",", dec_uint),
        _: " v=",
        v: separated_pair(dec_int, ",", dec_int),
    }}
    .parse_next(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let robots = input
        .lines()
        .filter_map(|line| robot_parser.parse(line).ok());
    let final_robots = robots.map(|rob| rob.step_n(100));
    let mut quadrants = [0, 0, 0, 0];
    final_robots.for_each(|rob| {
        if let Some(q) = rob.quadrant() {
            quadrants[usize::from(q)] += 1u32;
        }
    });
    let out = quadrants.into_iter().product();
    Some(out)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots: Vec<_> = input
        .lines()
        .filter_map(|line| robot_parser.parse(line).ok())
        .collect();
    for n in 1..10000 {
        robots.iter_mut().for_each(|rob| *rob = rob.step_n(1));
        let mut map = [[0u8; WIDTH as usize]; HEIGHT as usize];
        robots.iter().for_each(|rob| {
            map[rob.pos.1 as usize][rob.pos.0 as usize] = 255u8;
        });
        if map
            .concat()
            .as_slice()
            .windows(16)
            .contains(&[255u8; 16].as_slice())
        {
            save_buffer(
                format!("robots/{n}.png"),
                map.concat().as_slice(),
                WIDTH,
                HEIGHT,
                ExtendedColorType::L8,
            )
            .ok();
            return Some(n);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
