//! https://adventofcode.com/2019/day/4

use std::cmp::Ordering;

fn main() {
    let count = (359282..820401)
        .filter(|d| check_part1(&d.to_string()))
        .count();
    println!("Part 1: {}", count);

    let count = (359282..820401)
        .filter(|d| check_part2(&d.to_string()))
        .count();
    println!("Part 2: {}", count);
}

fn check_part1(pw: &str) -> bool {
    let mut previous = 0;
    let mut repeat = 1;
    let mut double = false;
    for c in pw.bytes() {
        match c.cmp(&previous) {
            Ordering::Less => {
                return false;
            }
            Ordering::Equal => {
                repeat += 1;
                if repeat == 2 {
                    double = true;
                }
            }
            Ordering::Greater => {
                repeat = 1;
            }
        }
        previous = c;
    }
    double
}

fn check_part2(pw: &str) -> bool {
    let mut previous = 0;
    let mut repeat = 1;
    let mut double = false;
    for c in pw.bytes() {
        match c.cmp(&previous) {
            Ordering::Less => {
                return false;
            }
            Ordering::Equal => {
                repeat += 1;
            }
            Ordering::Greater => {
                if repeat == 2 {
                    double = true;
                }
                repeat = 1;
            }
        }
        previous = c;
    }
    double || repeat == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert!(check_part1("111111"));
        assert!(!check_part1("223450"));
        assert!(!check_part1("123789"));
    }

    #[test]
    fn test_examples_part2() {
        assert!(!check_part2("111111"));
        assert!(!check_part2("123444"));
        assert!(check_part2("112233"));
        assert!(check_part2("123445"));
        assert!(check_part2("111122"));
    }
}
