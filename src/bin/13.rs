use winnow::{
    ascii::{dec_uint, multispace1},
    combinator::{preceded, separated, separated_pair, seq},
    prelude::*,
    token::any,
};

use nalgebra::{Matrix2, Vector2};

type Coord = Vector2<u64>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Claw {
    a: Coord,
    b: Coord,
    prize: Coord,
}

impl Claw {
    const A_COST: u64 = 3;
    const B_COST: u64 = 1;

    fn cost(&self, p2: bool) -> Option<u64> {
        let mat = Matrix2::new(
            self.a.x as f64,
            self.b.x as f64,
            self.a.y as f64,
            self.b.y as f64,
        );
        let prize = Vector2::new(self.prize.x as f64, self.prize.y as f64);
        dbg!(mat);
        let solution = mat.lu().solve(&prize)?;
        let (a, b) = (solution.x.round() as u64, solution.y.round() as u64);
        if !self.check(a, b, p2) {
            return None;
        }
        Some(a * Self::A_COST + b * Self::B_COST)
    }

    fn check(&self, a: u64, b: u64, p2: bool) -> bool {
        dbg!(a, b);
        if !p2 && (a > 100 || b > 100) {
            return false;
        }
        let reprize_x = a * self.a.x + b * self.b.x;
        let reprize_y = a * self.a.y + b * self.b.y;
        dbg!(reprize_x, reprize_y);
        if reprize_x != self.prize.x || reprize_y != self.prize.y {
            return false;
        }
        true
    }
}

fn coord_parser(input: &mut &str) -> PResult<Coord> {
    let (x, y): (u64, u64) = separated_pair(
        preceded(("X", any), dec_uint),
        ", ",
        preceded(("Y", any), dec_uint),
    )
    .parse_next(input)?;
    Ok(Coord::new(x, y))
}

fn claw_parser(input: &mut &str) -> PResult<Claw> {
    seq! {Claw {
        _: "Button A: ",
        a: coord_parser,
        _: "\nButton B: ",
        b: coord_parser,
        _: "\nPrize: ",
        prize: coord_parser,
    }}
    .parse_next(input)
}

fn claws_parser(input: &mut &str) -> PResult<Vec<Claw>> {
    separated(1.., claw_parser, multispace1).parse_next(input)
}

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u64> {
    let claws = claws_parser.parse(input.trim()).ok()?;
    let cost = claws.iter().filter_map(|c| c.cost(false)).sum();
    Some(cost)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut claws = claws_parser.parse(input.trim()).ok()?;
    claws
        .iter_mut()
        .for_each(|claw| claw.prize.add_scalar_mut(10000000000000));
    let cost = claws.iter().filter_map(|c| c.cost(true)).sum();
    Some(cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
