use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(23);

fn parse_connections(input: &str) -> Vec<(&str, &str)> {
    let mut connections: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect();
    connections
}

fn find_t_thruples<'a>(connections: &[(&'a str, &'a str)]) -> Vec<(&'a str, &'a str, &'a str)> {
    let mut t_thruples = Vec::new();
    for (i, (comp1, comp2)) in connections.iter().enumerate() {
        let mut comp1n: HashSet<&str> = HashSet::new();
        let mut comp2n: HashSet<&str> = HashSet::new();
        connections[i..].iter().for_each(|(alt_comp1, alt_comp2)| {
            if alt_comp1 == comp1 {
                comp1n.insert(alt_comp2);
            } else if alt_comp2 == comp1 {
                comp1n.insert(alt_comp1);
            } else if alt_comp1 == comp2 {
                comp2n.insert(alt_comp2);
            } else if alt_comp2 == comp2 {
                comp2n.insert(alt_comp1);
            }
        });
        for comp in comp1n.intersection(&comp2n) {
            if comp.starts_with('t') || comp1.starts_with('t') || comp2.starts_with('t') {
                t_thruples.push((*comp, *comp1, *comp2));
            }
        }
    }
    t_thruples
}

fn find_networks<'a>(connections: &[(&'a str, &'a str)]) -> BTreeSet<BTreeSet<&'a str>> {
    let mut networks = BTreeSet::new();
    for (comp1, comp2) in connections.iter() {
        let mut comp1n: BTreeSet<&str> = BTreeSet::new();
        let mut comp2n: BTreeSet<&str> = BTreeSet::new();
        let mut network = BTreeSet::new();
        connections.iter().for_each(|(alt_comp1, alt_comp2)| {
            if alt_comp1 == comp1 && alt_comp2 == comp2 {
                return;
            }
            if alt_comp1 == comp1 {
                comp1n.insert(alt_comp2);
            } else if alt_comp2 == comp1 {
                comp1n.insert(alt_comp1);
            } else if alt_comp1 == comp2 {
                comp2n.insert(alt_comp2);
            } else if alt_comp2 == comp2 {
                comp2n.insert(alt_comp1);
            }
        });
        for morcomp in comp1n.intersection(&comp2n).combinations(2) {
            if connections.iter().any(|(c1, c2)| {
                c1 == morcomp[0] && c2 == morcomp[1] || c2 == morcomp[0] && c1 == morcomp[1]
            }) {
                network.insert(*morcomp[0]);
                network.insert(*morcomp[1]);
            }
        }
        if !network.is_empty() {
            network.insert(*comp1);
            network.insert(*comp2);
            if network.iter().combinations(2).all(|cs| {
                connections
                    .iter()
                    .any(|(c1, c2)| c1 == cs[0] && c2 == cs[1] || c2 == cs[0] && c1 == cs[1])
            }) {
                networks.insert(network);
            }
        }
    }
    for network in networks.iter() {
        println!("{network:?}");
    }
    networks
}

pub fn part_one(input: &str) -> Option<usize> {
    let connections = parse_connections(input);
    let t_thruples = find_t_thruples(&connections);
    Some(t_thruples.len())
}

pub fn part_two(input: &str) -> Option<String> {
    let connections = parse_connections(input);
    let networks = find_networks(&connections);
    let largest = networks
        .into_iter()
        .max_by(|one, two| one.len().cmp(&two.len()))?;
    let password = largest.into_iter().sorted().join(",");
    Some(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
