//! https://adventofcode.com/2019/day/14

use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/2019/day14.txt").trim();

    println!("Part 1: {}", solve1(input));
    println!("Part 2: {}", solve2(input));
}

fn solve1(input: &str) -> u64 {
    let map = parse(input);

    let mut leftovers = HashMap::new();
    calculate("FUEL", 1, &map, &mut leftovers)
}

fn solve2(input: &str) -> u64 {
    let map = parse(input);

    let mut leftovers = HashMap::new();

    let mut ore = 1_000_000_000_000;
    let mut fuel = 0;
    // Do a bunch at a time at first, until we don't have enough ore, then try smaller numbers.
    // We could do 1 at a time, but that's pretty slow :).
    let mut step = 1_000_000;
    while ore > 0 {
        let needed = calculate("FUEL", step, &map, &mut leftovers);
        if needed > ore {
            if step == 1 {
                break;
            }
            step /= 10;
            continue;
        }

        ore -= needed;
        fuel += step;
    }
    fuel
}

fn parse(input: &str) -> HashMap<String, Reaction> {
    let mut map = HashMap::new();
    for line in input.trim().lines() {
        let mut parts = line.trim().split("=>");
        let inputs = parts.next().unwrap().trim();
        let output = Chemical::new(parts.next().unwrap().trim());

        let inputs: Vec<_> = inputs.split(", ").map(|s| Chemical::new(s)).collect();

        let name = output.name.clone();
        let reaction = Reaction { output, inputs };
        map.insert(name, reaction);
    }
    map
}

fn calculate(
    name: &str,
    mut needed: u64,
    map: &HashMap<String, Reaction>,
    leftovers: &mut HashMap<String, u64>,
) -> u64 {
    // base unit
    if name == "ORE" {
        return needed;
    }

    let reaction = map
        .get(name)
        .expect(&format!("Expected reaction {} to exist", name));
    // Use leftovers first
    if let Some(leftover) = leftovers.get_mut(name) {
        if *leftover > needed {
            *leftover -= needed;
            return 0;
        } else {
            needed -= *leftover;
            *leftover = 0;
        }
    }

    let mut count = needed / reaction.output.number;
    let partly_needed = needed % reaction.output.number;
    if partly_needed > 0 {
        // Need one more, which means there's going to be leftovers
        count += 1;

        let leftover = reaction.output.number - partly_needed;
        *leftovers.entry(name.to_string()).or_insert(0) += leftover;
    }

    reaction
        .inputs
        .iter()
        .map(|ing| calculate(&ing.name, count * ing.number, map, leftovers))
        .sum()
}

struct Reaction {
    pub output: Chemical,
    pub inputs: Vec<Chemical>,
}

#[derive(Debug)]
struct Chemical {
    pub number: u64,
    pub name: String,
}

impl Chemical {
    fn new(s: &str) -> Self {
        let mut parts = s.split(" ");
        let number = parts.next().unwrap().parse().unwrap();
        let name = parts.next().unwrap().to_string();
        Self { number, name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let s = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL
";

        assert_eq!(solve1(s), 165);
    }

    #[test]
    fn test_input() {
        let s = include_str!("../../input/2019/day14.txt").trim();

        assert_eq!(solve1(s), 612880);
        assert_eq!(solve2(s), 2509120);
    }
}
