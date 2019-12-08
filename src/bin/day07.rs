//! https://adventofcode.com/2019/day/7

use advent_of_code_2019::{Intcode, Result};

fn main() {
    let input = include_str!("../../input/2019/day07.txt");

    println!("Part 1: {}", largest_part1(input));
    println!("Part 2: {}", largest_part2(input));
}

fn largest_part1(prog: &str) -> i32 {
    let mut max = 0;

    for a in 0..=4 {
        for b in 0..=4 {
            if a == b {
                continue;
            }
            for c in 0..=4 {
                if c == a || c == b {
                    continue;
                }
                for d in 0..=4 {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    for e in 0..=4 {
                        // OMG, haha
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }
                        let sequence = vec![a, b, c, d, e];
                        let result = score_part1(prog, sequence);
                        if result > max {
                            max = result;
                        }
                    }
                }
            }
        }
    }

    max
}

fn largest_part2(prog: &str) -> i32 {
    let mut max = 0;

    for a in 5..=9 {
        for b in 5..=9 {
            if a == b {
                continue;
            }
            for c in 5..=9 {
                if c == a || c == b {
                    continue;
                }
                for d in 5..=9 {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    for e in 5..=9 {
                        // OMG, haha
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }
                        let sequence = vec![a, b, c, d, e];
                        let result = score_part2(prog, sequence);
                        if result > max {
                            max = result;
                        }
                    }
                }
            }
        }
    }

    max
}

fn score_part1(prog: &str, sequence: Vec<i32>) -> i32 {
    let code = Intcode::parse(prog);
    let mut signal = 0;
    for n in sequence {
        if let Result::Output(o) = code.clone().add_input(n).add_input(signal).run() {
            signal = o;
        }
    }
    signal
}

fn score_part2(prog: &str, sequence: Vec<i32>) -> i32 {
    let prototype = Intcode::parse(prog);

    let mut amps = Vec::new();
    for n in sequence {
        let mut amp = prototype.clone();
        amp.add_input(n);
        amps.push(amp);
    }

    let mut signal = 0;
    loop {
        for amp in &mut amps {
            amp.add_input(signal);
            match amp.run() {
                Result::Output(o) => signal = o,
                Result::Halt => return signal,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let s = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        assert_eq!(largest_part1(s), 43210);
    }

    #[test]
    fn test_examples_part2() {
        let s =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        assert_eq!(largest_part2(s), 139629729);

        let s = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        assert_eq!(largest_part2(s), 18216);
    }
}
