use std::fmt::Display;

use winnow::{
    ascii::dec_int,
    combinator::{separated, separated_pair},
    prelude::*,
};

use itertools::{repeat_n, Itertools};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Problem {
    result: i64,
    inputs: Vec<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Operation {
    Mul,
    Add,
    Concat,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Operation::Mul => "*",
            Operation::Add => "+",
            Operation::Concat => "||",
        })
    }
}

impl Operation {
    fn apply(self, v1: i64, v2: i64) -> i64 {
        match self {
            Operation::Mul => v1 * v2,
            Operation::Add => v1 + v2,
            Operation::Concat => format!("{v1}{v2}").parse().unwrap(),
        }
    }
}

fn problem(input: &mut &str) -> PResult<Problem> {
    let (result, inputs) = separated_pair(dec_int, ": ", separated(2.., dec_int::<_, i64, _>, ' '))
        .parse_next(input)?;
    Ok(Problem { result, inputs })
}

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<i64> {
    let mut results = 0;
    let problems: Vec<Problem> = input
        .lines()
        .map(|line| problem.parse(line).unwrap())
        .collect();
    for Problem { result, inputs } in problems {
        let (start, rest) = inputs.split_first()?;
        let operation_sequences =
            repeat_n([Operation::Add, Operation::Mul], rest.len()).multi_cartesian_product();
        for operations in operation_sequences {
            let calculated: i64 = operations
                .into_iter()
                .zip(rest)
                .fold(*start, |init, (op, num)| op.apply(init, *num));
            if calculated == result {
                results += result;
                break;
            }
        }
    }
    Some(results)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut results = 0;
    let problems: Vec<Problem> = input
        .lines()
        .map(|line| problem.parse(line).unwrap())
        .collect();
    for Problem { result, inputs } in problems {
        let (start, rest) = inputs.split_first()?;
        let operation_sequences = repeat_n(
            [Operation::Add, Operation::Mul, Operation::Concat],
            rest.len(),
        )
        .multi_cartesian_product();
        for operations in operation_sequences {
            let calculated: i64 = operations
                .into_iter()
                .zip(rest)
                .fold(*start, |init, (op, num)| op.apply(init, *num));
            if calculated == result {
                results += result;
                break;
            }
        }
    }
    Some(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let problem = problem
            .parse("79325232924618: 907 466 857 34 3 618")
            .unwrap();
        assert_eq!(
            problem,
            Problem {
                result: 79325232924618,
                inputs: vec![907, 466, 857, 34, 3, 618]
            }
        )
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
