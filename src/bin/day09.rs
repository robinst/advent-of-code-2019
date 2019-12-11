//! https://adventofcode.com/2019/day/9

use advent_of_code_2019::Intcode;

fn main() {
    let input = include_str!("../../input/2019/day09.txt").trim();
    let code = Intcode::parse(input);

    println!("Part 1: {:?}", code.clone().add_input(1).run_all());
    println!("Part 2: {:?}", code.clone().add_input(2).run_all());
}
