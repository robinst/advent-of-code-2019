//! https://adventofcode.com/2019/day/6

use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/2019/day06.txt");
    let pairs = parse(input);
    let orbits = orbits(&pairs);

    println!("Part 1: {}", total_orbits(&orbits));
    println!("Part 2: {}", transfers(&orbits));
}

fn parse(prog: &str) -> Vec<(String, String)> {
    prog.trim()
        .lines()
        .map(|line| {
            let mut split = line.split(')');
            let a = split.next().unwrap();
            let b = split.next().unwrap();
            (a.to_string(), b.to_string())
        })
        .collect()
}

fn orbits(pairs: &Vec<(String, String)>) -> HashMap<String, String> {
    let mut orbits = HashMap::new();
    for (a, b) in pairs {
        orbits.insert(b.clone(), a.clone());
    }
    orbits
}

fn total_orbits(orbits: &HashMap<String, String>) -> u32 {
    let mut result = 0;
    for key in orbits.keys() {
        let mut count = 0;

        let mut o = key;
        while let Some(other) = orbits.get(o) {
            count += 1;
            o = other;
        }

        result += count;
    }

    result
}

fn path(orbits: &HashMap<String, String>, start: String) -> Vec<String> {
    let mut path = Vec::new();
    let mut o = &start;
    while let Some(other) = orbits.get(o) {
        path.push(other.to_string());
        o = other;
    }

    path
}

fn transfers(orbits: &HashMap<String, String>) -> usize {
    let san = path(orbits, "SAN".to_string());
    let you = path(orbits, "YOU".to_string());

    for (i, o) in san.iter().enumerate() {
        if let Some(p) = you.iter().position(|x| x == o) {
            return i + p;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let s = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";

        assert_eq!(total_orbits(&orbits(&parse(s))), 42);
    }

    #[test]
    fn test_examples_part2() {
        let s = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
";

        assert_eq!(transfers(&orbits(&parse(s))), 4);
    }
}
