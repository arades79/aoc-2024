advent_of_code::solution!(1);

use std::collections::{BinaryHeap};

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

use std::collections::BTreeMap;

pub fn part_two(input: &str) -> Option<u32> {
    let mut left = BTreeMap::new();
    let mut right = BTreeMap::new();
    for line in input.lines() {
        let (l, r): (u32, u32) = {
            let mut it = line.split_ascii_whitespace();
            (it.next()?.parse().ok()?, it.next()?.parse().ok()?)
        };
        left.entry(l).and_modify(|v| *v += 1).or_insert(1);
        right.entry(r).and_modify(|v| *v += 1).or_insert(1);
    }
    let similarity = left
        .into_iter()
        .map(|(n, c)| right.get(&n).unwrap_or(&0) * n * c)
        .sum();
    Some(similarity)
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
        assert_eq!(result, Some(31));
    }
}
