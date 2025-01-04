use pathfinding::directed::{count_paths::count_paths, dfs::dfs};

advent_of_code::solution!(19);

fn successors(
    current_towels: &str,
    remaining_towels: &str,
    available_towels: &[&str],
) -> Vec<(String, String)> {
    let mut next_towels = Vec::new();
    for towel in available_towels.iter().copied() {
        if let Some(new_remaining) = remaining_towels.strip_prefix(towel) {
            let new_current = format!("{current_towels}{towel}");
            next_towels.push((new_current, new_remaining.to_owned()));
        }
    }
    next_towels
}

fn finished(current_towels: &str, target_towel: &str) -> bool {
    current_towels == target_towel
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let towels: Vec<&str> = {
        let towel_line = lines.next()?;
        towel_line.trim().split(", ").collect()
    };
    let _empty = lines.next()?;
    let mut possible_arrangements = 0;
    for pattern in lines {
        possible_arrangements += count_paths(
            (String::new(), pattern.to_string()),
            |(set, remaining): &(String, String)| successors(set, remaining, &towels),
            |(set, _): &(String, String)| finished(set, pattern),
        );
    }

    Some(possible_arrangements)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let towels: Vec<&str> = {
        let towel_line = lines.next()?;
        towel_line.trim().split(", ").collect()
    };
    let _empty = lines.next()?;
    let mut possible_arrangements = 0;
    for pattern in lines {
        if dfs(
            (String::new(), pattern.to_string()),
            |(set, remaining): &(String, String)| successors(set, remaining, &towels),
            |(set, _): &(String, String)| finished(set, pattern),
        )
        .is_some()
        {
            possible_arrangements += 1;
        }
    }

    Some(possible_arrangements)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
