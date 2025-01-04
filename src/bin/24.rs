use itertools::Itertools;
use std::{collections::HashSet, hash::Hash};
use winnow::{
    ascii::{alphanumeric1, dec_uint},
    combinator::{alt, preceded, separated_pair},
    prelude::*,
};

advent_of_code::solution!(24);

#[derive(Debug, Clone, Copy, PartialOrd, Ord)]
struct Wire<'a> {
    id: &'a str,
    value: Option<bool>,
}

impl<'a> Wire<'a> {
    fn new(id: &'a str) -> Self {
        Wire { id, value: None }
    }
    fn with_value(id: &'a str, value: bool) -> Self {
        Wire {
            id,
            value: Some(value),
        }
    }
}
impl PartialEq for Wire<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Wire<'_> {}

impl Hash for Wire<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    Or,
    And,
    Xor,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Gate<'a> {
    inputs: (Wire<'a>, Wire<'a>),
    op: Op,
    output: Wire<'a>,
}

impl<'a> Gate<'a> {
    fn new(in1: &'a str, in2: &'a str, op: Op, output: &'a str) -> Self {
        Gate {
            inputs: (Wire::new(in1), Wire::new(in2)),
            op,
            output: Wire::new(output),
        }
    }

    fn simulate(&self) -> Option<bool> {
        let (in1, in2) = (self.inputs.0.value?, self.inputs.1.value?);
        Some(match self.op {
            Op::Or => in1 | in2,
            Op::And => in1 & in2,
            Op::Xor => in1 ^ in2,
        })
    }

    fn update(&mut self, wires: &mut Wires<'a>) -> Option<()> {
        if self.output.value.is_some() {
            return None;
        }

        self.inputs.0 = *wires.get(&self.inputs.0)?;
        self.inputs.1 = *wires.get(&self.inputs.1)?;
        self.output.value = Some(self.simulate()?);
        wires.replace(self.output);
        Some(())
    }

    fn swap_outs(&mut self, other: &mut Self) {
        std::mem::swap(&mut self.output, &mut other.output);
    }
}

type Wires<'a> = HashSet<Wire<'a>>;
type Gates<'a> = Vec<Gate<'a>>;

/// "x00: 1" -> `Wire {id: "x00", value: Some(1)`
fn wire_parser<'a>(input: &mut &'a str) -> PResult<Wire<'a>> {
    let (id, value): (&str, u8) =
        separated_pair(alphanumeric1, ": ", dec_uint).parse_next(input)?;
    Ok(Wire::with_value(id, value != 0))
}

/// "x00 AND y00 -> z00"
fn gate_parser<'a>(input: &mut &'a str) -> PResult<Gate<'a>> {
    let op_parser = |i: &mut &str| {
        alt((
            "AND".map(|_| Op::And),
            "OR".map(|_| Op::Or),
            "XOR".map(|_| Op::Xor),
        ))
        .parse_next(i)
    };
    let (in1, op, in2, output) = (
        alphanumeric1,
        preceded(" ", op_parser),
        preceded(" ", alphanumeric1),
        preceded(" -> ", alphanumeric1),
    )
        .parse_next(input)?;
    Ok(Gate::new(in1, in2, op, output))
}

fn check_nets(wires: &Wires, prefix: char) -> Option<u64> {
    let outputs = wires
        .iter()
        .filter(|Wire { id, .. }| id.starts_with(prefix));
    let mut value = 0;
    for output in outputs {
        let bit = output.value? as u64;
        let position = output.id.strip_prefix(prefix)?.parse::<u64>().ok()?;
        value |= bit << position;
    }
    Some(value)
}

fn check_output(wires: &Wires) -> Option<u64> {
    check_nets(wires, 'z')
}

fn parse_nets(input: &str) -> Option<(Wires<'_>, Gates<'_>)> {
    let (wire_input, gate_input) = input.split_once("\n\n")?;
    let mut wires: Wires = wire_input
        .lines()
        .filter_map(|line| wire_parser.parse(line).ok())
        .collect();
    let mut gates = Gates::new();
    for line in gate_input.lines() {
        let gate = gate_parser
            .parse(line)
            .inspect_err(|e| {
                dbg!(e);
            })
            .ok()?;
        wires.insert(gate.inputs.0);
        wires.insert(gate.inputs.1);
        wires.insert(gate.output);
        gates.push(gate);
    }
    Some((wires, gates))
}

fn update_all<'a>(gates: &mut Gates<'a>, wires: &mut Wires<'a>) {
    for gate in gates.iter_mut() {
        gate.update(wires);
    }
}

fn run_simulation<'a>(mut gates: Gates<'a>, mut wires: Wires<'a>) -> Option<u64> {
    for _ in 0..gates.len() {
        if let Some(output) = check_output(&wires) {
            return Some(output);
        }
        update_all(&mut gates, &mut wires);
    }
    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let (wires, gates) = parse_nets(input)?;
    run_simulation(gates, wires)
}

pub fn part_two(input: &str) -> Option<String> {
    let (wires, gates) = parse_nets(input)?;
    #[cfg(not(test))]
    const BITS: u64 = 46;
    #[cfg(not(test))]
    const MAX: u64 = (2 << BITS) - 1;
    #[cfg(test)]
    let expected_output = check_nets(&wires, 'x')? & check_nets(&wires, 'y')?;
    #[cfg(not(test))]
    let expected_output = (check_nets(&wires, 'x')? + check_nets(&wires, 'y')?) % MAX;
    println!("expecting {expected_output}");
    #[cfg(test)]
    const COMB: usize = 4;
    #[cfg(not(test))]
    const COMB: usize = 8;
    for oct in (0..gates.len()).permutations(COMB) {
        let mut alt_gates = gates.clone();
        let (left, right) = oct.split_at(COMB / 2);
        for (l, r) in left.iter().copied().zip(right.iter().copied()) {
            let (mut g1, mut g2) = (alt_gates[l].clone(), alt_gates[r].clone());
            g1.swap_outs(&mut g2);
            alt_gates[l] = g1;
            alt_gates[r] = g2;
        }
        if let Some(output) = run_simulation(alt_gates.clone(), wires.clone()) {
            println!("swapping {left:?} and {right:?} yields {output}");
            if output == expected_output {
                return Some(
                    oct.into_iter()
                        .map(|index| gates[index].output.id)
                        .sorted()
                        .join(","),
                );
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_parse() {
        let wire = wire_parser.parse("x00: 1").unwrap();
        assert_eq!(
            wire,
            Wire {
                id: "x00",
                value: Some(true)
            }
        )
    }

    #[test]
    fn test_gate_parse() {
        let gate = gate_parser.parse("x00 AND y00 -> z00").unwrap();
        assert_eq!(
            gate,
            Gate {
                inputs: (Wire::new("x00"), Wire::new("y00")),
                op: Op::And,
                output: Wire::new("z00")
            }
        )
    }

    #[test]
    fn test_update() {
        let mut wires = Wires::new();
        wires.insert(Wire::with_value("x", true));
        wires.insert(Wire::with_value("y", true));
        wires.insert(Wire::new("z"));
        let mut gate = Gate::new("x", "y", Op::And, "z");
        gate.update(&mut wires);
        assert_eq!(Some(true), wires.get(&Wire::new("z")).unwrap().value);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some("z00,z01,z02,z05".to_string()));
    }
}
