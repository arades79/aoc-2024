advent_of_code::solution!(1);

use std::collections::BinaryHeap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();
    for line in input.lines() {
        let (l, r): (i32, i32) = {
            let mut it = line.split_ascii_whitespace();
            (it.next()?.parse().ok()?, it.next()?.parse().ok()?)
        };
        left.push(l);
        right.push(r);
    }
    let left = left.into_sorted_vec();
    let right = right.into_sorted_vec();
    let sum = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs() as u32)
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
