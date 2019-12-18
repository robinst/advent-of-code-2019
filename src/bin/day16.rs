//! https://adventofcode.com/2019/day/16

use itertools::Itertools;
use std::iter;

fn main() {
    let input = include_str!("../../input/2019/day16.txt").trim();

    println!("Part 1: {}", solve1(input));
    println!("Part 2: {}", solve2(input));
}

fn solve1(input: &str) -> String {
    let signal = parse(input);
    let result = calculate1(signal);
    result.iter().join("")[0..8].to_string()
}

fn solve2(input: &str) -> String {
    let base_signal = parse(input);
    let mut signal = Vec::new();
    for _ in 0..10_000 {
        signal.append(&mut base_signal.clone());
    }

    let result = calculate2(signal);

    let offset: usize = input[0..7].parse().unwrap();
    result[offset..offset + 8].iter().join("").to_string()
}

fn parse(input: &str) -> Vec<u8> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn calculate1(mut signal: Vec<u8>) -> Vec<u8> {
    let mut next_signal: Vec<u8> = Vec::with_capacity(signal.len());
    let base_pattern: Vec<i32> = vec![0, 1, 0, -1];
    for _ in 0..100 {
        for i in 0..signal.len() {
            let repeat = i + 1;
            let pattern = base_pattern
                .iter()
                .cycle()
                .flat_map(|&s| iter::once(s).cycle().take(repeat))
                .skip(1);
            let result: i32 = signal.iter().zip(pattern).map(|(&s, p)| s as i32 * p).sum();
            let digit = result.abs() % 10;
            next_signal.push(digit as u8);

            // Slightly faster but nowhere near fast enough for part 2:
            // let repeat = i + 1;
            //
            // let mut result: i32 = 0;
            // let mut x = repeat - 1;
            // let mut sign = 1;
            // while x < signal.len() {
            //     let sum: i32 = signal[x..(x+repeat).min(signal.len())].iter().map(|&n| n as i32).sum();
            //     result += sign * sum;
            //
            //     sign = -sign;
            //     // Skip 0 patterns
            //     x += repeat * 2;
            // }
            //
            // let digit = result.abs() % 10;
            // next_signal.push(digit as u8);
        }

        signal.clear();
        signal.append(&mut next_signal);
    }
    signal
}

// I didn't realize the trick with the offset (that you can just skip the millions of leading
// digits), so I found a way to make it fast enough with a smarter algorithm. Looking at just the
// signs:
//
// 1  0 -1  0  1  0 -1  0  1  0 -1  0  1  0 -1  0  1  0 -1
// 0  1  1  0  0 -1 -1  0  0  1  1  0  0 -1 -1  0  0  1  1
// 0  0  1  1  1  0  0  0 -1 -1 -1  0  0  0  1  1  1  0  0
// 0  0  0  1  1  1  1  0  0  0  0 -1 -1 -1 -1  0  0  0  0
// 0  0  0  0  1  1  1  1  1  0  0  0  0  0 -1 -1 -1 -1 -1
// 0  0  0  0  0  1  1  1  1  1  1  0  0  0  0  0  0 -1 -1
// 0  0  0  0  0  0  1  1  1  1  1  1  1  0  0  0  0  0  0
// 0  0  0  0  0  0  0  1  1  1  1  1  1  1  1  0  0  0  0
//
// You can see that for the first triangle of 1s, the difference between a row and the next is that
// one of the numbers in the beginning (of the previous row) is no longer in the sum, and two new
// numbers at the end (of the next row) are part of the sum.
//
// We can take advantage of that and not re-sum all those common numbers. In the end I think the
// complexity is O(n*log(n)).
//
// Indexes for the first sign:
// col = 0, sign = 1
// row 0: from 0 to 1 (length 1)
// row 1: from 1 to 3 (length 2)
// row 2: from 2 to 5 (length 3)
// row 3: from 3 to 7 (length 4)
//
// Indexes for the second sign (we can ignore 0):
// col = 2, sign = -1
// row 0: from  2 to 3 (length 1)
// row 1: from  5 to 7 (length 2)
// row 2: from  8 to 11 (length 3)
// row 3: from 11 to 15 (length 4)
//
fn calculate2(mut signal: Vec<u8>) -> Vec<u8> {
    let mut next_signal: Vec<i32> = vec![0; signal.len()];

    let base_pattern: Vec<i32> = vec![0, 1, 0, -1];

    for phase in 0..100 {
        println!("Phase {}", phase);
        for (col, &sign) in base_pattern
            .iter()
            .cycle()
            .skip(1)
            .take(signal.len())
            .enumerate()
        {
            if sign == 0 {
                continue;
            }

            let mut sum: i32 = 0;

            let mut previous_start = 0;
            let mut previous_end = 0;

            for row in 0..signal.len() {
                let start = (col * (row + 1)) + row;
                let end = (start + row + 1).min(signal.len());

                if start >= signal.len() {
                    break;
                }
                let remove: i32 = signal[previous_start..(previous_end.min(start))]
                    .iter()
                    .map(|&d| d as i32)
                    .sum();
                let add: i32 = signal[(previous_end.max(start)..end.min(signal.len()))]
                    .iter()
                    .map(|&d| d as i32)
                    .sum();

                sum -= sign * remove;
                sum += sign * add;

                next_signal[row] += sum;

                previous_start = start;
                previous_end = end;
            }
        }

        for (i, n) in next_signal.iter().enumerate() {
            signal[i] = (n.abs() % 10) as u8;
        }

        for n in next_signal.iter_mut() {
            *n = 0;
        }
    }

    signal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(solve1("80871224585914546619083218645595"), "24176176");
        assert_eq!(solve1("19617804207202209144916044189917"), "73745418");
        assert_eq!(solve1("69317163492948606335995924319873"), "52432133");

        assert_eq!(solve2("80871224585914546619083218645595"), "24176176");
        assert_eq!(solve2("19617804207202209144916044189917"), "73745418");
        assert_eq!(solve2("69317163492948606335995924319873"), "52432133");
    }

    #[test]
    fn test_input() {
        let s = include_str!("../../input/2019/day16.txt").trim();

        assert_eq!(solve1(s), "45834272");
    }
}
