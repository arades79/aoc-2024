use std::collections::HashMap;

advent_of_code::solution!(11);

fn next_rocks(rock: u64) -> (u64, Option<u64>) {
    if rock == 0 {
        (1, None)
    } else {
        let mut strock = rock.to_string();
        if strock.len() % 2 == 0 {
            let strock2 = strock.split_off(strock.len() / 2);
            (strock.parse().unwrap(), Some(strock2.parse().unwrap()))
        } else {
            (rock * 2024, None)
        }
    }
}

fn blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut next_map = HashMap::new();
    for (rock, count) in stones {
        let (next_rock, other_next_rock) = next_rocks(rock);
        next_map
            .entry(next_rock)
            .and_modify(|current_count| *current_count += count)
            .or_insert(count);
        if let Some(other_next_rock) = other_next_rock {
            next_map
                .entry(other_next_rock)
                .and_modify(|current_count| *current_count += count)
                .or_insert(count);
        }
    }
    next_map
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones = input
        .trim()
        .split(' ')
        .map(|strock| (strock.parse().unwrap(), 1))
        .collect();
    for _ in 0..25 {
        stones = blink(stones);
    }
    let stone_sum = stones.into_values().sum();
    Some(stone_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut stones = input
        .trim()
        .split(' ')
        .map(|strock| (strock.parse().unwrap(), 1))
        .collect();
    for _ in 0..75 {
        stones = blink(stones);
    }
    let stone_sum = stones.into_values().sum();
    Some(stone_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
