//! https://adventofcode.com/2019/day/2

fn main() {
    let input = include_str!("../../input/2019/day02.txt");
    let input_prog: Vec<usize> = input
        .split(',')
        .map(|s| {
            s.trim()
                .parse()
                .unwrap_or_else(|_| panic!("Error parsing {:?}", s))
        })
        .collect();

    let mut first = input_prog.clone();
    first[1] = 12;
    first[2] = 2;
    println!("part 1: {}", calculate(first));

    println!("part 2: {}", part2(&input_prog));
}

fn part2(input_prog: &[usize]) -> usize {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut prog = input_prog.to_owned();
            prog[1] = noun;
            prog[2] = verb;
            let result = calculate(prog);
            if result == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

fn calculate(mut prog: Vec<usize>) -> usize {
    let mut ip = 0;
    loop {
        let op = prog[ip];
        match op {
            99 => break,
            1 => {
                let a = prog[ip + 1];
                let b = prog[ip + 2];
                let t = prog[ip + 3];
                prog[t] = prog[a] + prog[b];
                ip += 4;
            }
            2 => {
                let a = prog[ip + 1];
                let b = prog[ip + 2];
                let t = prog[ip + 3];
                prog[t] = prog[a] * prog[b];
                ip += 4;
            }
            _ => unimplemented!("Unknown opcode {}", op),
        }
    }
    prog[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        #[rustfmt::skip]
        assert_eq!(
            calculate(vec![
                1, 9, 10, 3,
                2, 3, 11, 0,
                99,
                30, 40, 50]),
            3500);
    }
}
