use itertools::Itertools;

advent_of_code::solution!(25);

const TUMBLERS: usize = 5;
const DEPTH: u8 = 7;

type Bitting = [u8; TUMBLERS];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum LocKey {
    Lock(Bitting),
    Key(Bitting),
}

impl LocKey {
    fn into_lock(self) -> Option<Bitting> {
        match self {
            LocKey::Lock(bitting) => Some(bitting),
            LocKey::Key(_) => None,
        }
    }
    fn into_key(self) -> Option<Bitting> {
        match self {
            LocKey::Key(bitting) => Some(bitting),
            LocKey::Lock(_) => None,
        }
    }
}

fn parse_lockey(input: &str) -> LocKey {
    let mut arr = [0; TUMBLERS];
    for line in input.lines().map(str::as_bytes) {
        assert_eq!(
            line.len(),
            TUMBLERS,
            "Got a line with {} tumblers instead of expected {}",
            line.len(),
            TUMBLERS
        );
        for i in 0..TUMBLERS {
            if line[i] == b'#' {
                arr[i] += 1;
            }
        }
    }
    match input.as_bytes()[0] {
        b'#' => LocKey::Lock(arr),
        b'.' => LocKey::Key(arr),
        _ => panic!("Bad schematic"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lockeys: Vec<_> = input.split("\n\n").map(parse_lockey).collect();
    for lockey in lockeys.iter() {
        println!("{lockey:?}");
    }
    let locks = lockeys.iter().copied().filter_map(LocKey::into_lock);
    let keys = lockeys.iter().copied().filter_map(LocKey::into_key);
    let working_combos = locks
        .cartesian_product(keys)
        .map(|(lock, key)| {
            lock.into_iter()
                .zip(key)
                .map(|(lb, kb)| lb + kb)
                .all(|d| d <= DEPTH) as u32
        })
        .sum();
    Some(working_combos)
}

pub fn part_two(_input: &str) -> Option<u32> {
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
