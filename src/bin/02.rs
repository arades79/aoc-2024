advent_of_code::solution!(2);

use winnow::{ascii::dec_uint, combinator::separated, prelude::*};

fn line(input: &mut &str) -> PResult<Vec<u32>> {
    separated(0.., dec_uint::<_, u32, _>, ' ').parse_next(input)
}

fn is_safe(row: &[u32]) -> bool {
    const MAX_DIST: u32 = 3;
    let dir = if row.len() >= 2 {
        row[0] > row[1]
    } else {
        false
    };
    for pair in row.windows(2) {
        if (pair[0] > pair[1]) != dir || pair[0].abs_diff(pair[1]) > MAX_DIST || pair[0] == pair[1]
        {
            return false;
        }
    }
    true
}

fn is_safe_with_dampen(row: &[u32]) -> bool {
    const MAX_DIST: u32 = 3;
    let dir = if row.len() >= 2 {
        row[0] > row[1]
    } else {
        false
    };
    for pair in row.windows(2) {
        if (pair[0] > pair[1]) != dir || pair[0].abs_diff(pair[1]) > MAX_DIST || pair[0] == pair[1]
        {
            for i in 0..row.len() {
                let mut shortened = row.to_vec();
                shortened.remove(i);
                if is_safe(&shortened) {return true}
            }
            return false
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let safe_lines = input
        .lines()
        .map(|s| line.parse(s).unwrap())
        .fold(0, |sum, row| sum + is_safe(&row) as u32);
    Some(safe_lines)
}

pub fn part_two(input: &str) -> Option<u32> {
    let safe_lines = input
        .lines()
        .map(|s| line.parse(s).unwrap())
        .fold(0, |sum, row| sum + is_safe_with_dampen(&row) as u32);
    Some(safe_lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
