use std::collections::{HashMap, HashSet};

advent_of_code::solution!(22);

const fn prng(mut seed: u64) -> u64 {
    const PRUNE: u64 = 16777216;
    seed ^= seed << 6;
    seed %= PRUNE;
    seed ^= seed >> 5;
    seed %= PRUNE;
    seed ^= seed << 11;
    seed %= PRUNE;
    seed
}

const fn prng_n_times(mut seed: u64, mut n: u64) -> u64 {
    while n > 0 {
        seed = prng(seed);
        n -= 1;
    }
    seed
}

fn prng_digit_sequence(mut seed: u64) -> Vec<i8> {
    (0..=2000)
        .map(|_| {
            let value = (seed % 10) as i8;
            seed = prng(seed);
            value
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let monkey_numbers: Vec<u64> = input.lines().filter_map(|line| line.parse().ok()).collect();
    let answer = monkey_numbers
        .into_iter()
        .map(|s| prng_n_times(s, 2000))
        .sum();
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let monkey_numbers: Vec<u64> = input.lines().filter_map(|line| line.parse().ok()).collect();
    let mut banana_sequences = HashMap::new();
    for number in monkey_numbers {
        let sequence = prng_digit_sequence(number);
        let diff_sequence: Vec<i8> = sequence.windows(2).map(|nums| nums[0] - nums[1]).collect();
        let mut seen_sequences = HashSet::new();
        diff_sequence.windows(4).enumerate().for_each(|(i, quad)| {
            if seen_sequences.insert(quad.to_vec()) {
                let bananas = sequence[i + 4];
                banana_sequences
                    .entry(quad.to_vec())
                    .and_modify(|b| *b += bananas as u64)
                    .or_insert(bananas as u64);
            }
        });
    }
    for seq in banana_sequences.iter() {
        if *seq.1 > 16 {
            println!("{seq:?}");
        }
    }
    let max_bananas = banana_sequences.into_values().max()?;

    Some(max_bananas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng() {
        assert_eq!(prng_n_times(123, 1), 15887950);
        assert_eq!(prng_n_times(123, 2), 16495136);
        assert_eq!(prng_n_times(123, 3), 527345);
        assert_eq!(prng_n_times(123, 4), 704524);
        assert_eq!(prng_n_times(123, 5), 1553684);
        assert_eq!(prng_n_times(123, 6), 12683156);
        assert_eq!(prng_n_times(123, 7), 11100544);
        assert_eq!(prng_n_times(123, 8), 12249484);
        assert_eq!(prng_n_times(123, 9), 7753432);
        assert_eq!(prng_n_times(123, 10), 5908254);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
