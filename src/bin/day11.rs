//! https://adventofcode.com/2019/day/11

use advent_of_code_2019::{Intcode, Result};
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/2019/day11.txt").trim();
    let code = Intcode::parse(input);

    let map = paint(code.clone(), 0);
    println!("Part 1: {:?}", map.len());

    let map = paint(code.clone(), 1);
    let (start_x, start_y, end_x, end_y) =
        map.keys()
            .fold((0, 0, 0, 0), |(start_x, start_y, end_x, end_y), &(x, y)| {
                (start_x.min(x), start_y.min(y), end_x.max(x), end_y.max(y))
            });

    for y in start_y..=end_y {
        for x in start_x..=end_x {
            let color = *map.get(&(x, y)).unwrap_or(&0);
            match color {
                0 => print!("  "),
                1 => print!("##"),
                _ => {}
            }
        }
        println!();
    }
}

fn paint(mut code: Intcode, starting_color: i64) -> HashMap<(i32, i32), i64> {
    // 0 = up, then clockwise
    let mut direction = 0;
    let mut x = 0;
    let mut y = 0;
    let mut map = HashMap::new();
    map.insert((x, y), starting_color);
    loop {
        code.add_input(*map.get(&(x, y)).unwrap_or(&0));

        match code.run() {
            Result::Output(new_color) => {
                map.insert((x, y), new_color);
            }
            Result::Halt => {
                break;
            }
        }

        match code.run() {
            Result::Output(turn) => match turn {
                0 => direction = if direction == 0 { 3 } else { direction - 1 },
                1 => direction = (direction + 1) % 4,
                other => panic!("Unknown direction output {}", other),
            },
            Result::Halt => {
                break;
            }
        }

        match direction {
            0 => y -= 1,
            1 => x += 1,
            2 => y += 1,
            3 => x -= 1,
            other => panic!("Unknown direction {}", other),
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../../input/2019/day11.txt").trim();
        let code = Intcode::parse(input);

        assert_eq!(paint(code, 0).len(), 1964);
    }
}
