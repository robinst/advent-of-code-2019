//! https://adventofcode.com/2019/day/5

fn main() {
    let input = include_str!("../../input/2019/day05.txt");
    let input_prog = parse(input);

    // 3 wrong, 5346030 right
    println!("Part 1: {}", calculate(input_prog.clone(), 1));
    // 1998926 wrong, 513116 right
    println!("Part 2: {}", calculate(input_prog.clone(), 5));
}

fn parse(prog: &str) -> Vec<i32> {
    prog
        .split(',')
        .map(|s| {
            s.trim()
                .parse()
                .unwrap_or_else(|_| panic!("Error parsing {:?}", s))
        })
        .collect()
}

fn calculate(mut prog: Vec<i32>, input: i32) -> i32 {
    let mut ip = 0;
    let mut output = 0;
    loop {
        let instruction = prog[ip as usize];
        let op = instruction % 100;
        let mut modes = instruction / 100;
        let mode_a = modes % 10;
        modes /= 10;
        let mode_b = modes % 10;
        // unused:
//        modes /= 10;
//        let mode_c = modes % 10;
        match op {
            99 => break,
            1 => {
                let a = prog[(ip + 1) as usize];
                let b = prog[(ip + 2) as usize];
                let t = prog[(ip + 3) as usize];
                prog[t as usize] = if mode_a == 0 { prog[a as usize] } else { a } + if mode_b == 0 { prog[b as usize] } else { b };
                ip += 4;
            }
            2 => {
                let a = prog[(ip + 1) as usize];
                let b = prog[(ip + 2) as usize];
                let t = prog[(ip + 3) as usize];
                prog[t as usize] = if mode_a == 0 { prog[a as usize] } else { a } * if mode_b == 0 { prog[b as usize] } else { b };
                ip += 4;
            }
            3 => {
                let a = prog[(ip + 1) as usize];
                prog[a as usize] = input;
                ip += 2;
            }
            4 => {
                let a = prog[(ip + 1) as usize];
                output = if mode_a == 0 { prog[a as usize] } else { a };
                ip += 2;
            }
            5 => {
                // is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
                let a = prog[(ip + 1) as usize];
                let b = prog[(ip + 2) as usize];
                let val_a = if mode_a == 0 { prog[a as usize] } else { a };
                let val_b = if mode_b == 0 { prog[b as usize] } else { b };
                if val_a != 0 {
                    ip = val_b;
                } else {
                    ip += 3;
                }
            }
            6 => {
                let a = prog[(ip + 1) as usize];
                let b = prog[(ip + 2) as usize];
                let val_a = if mode_a == 0 { prog[a as usize] } else { a };
                let val_b = if mode_b == 0 { prog[b as usize] } else { b };
                if val_a == 0 {
                    ip = val_b;
                } else {
                    ip += 3;
                }
            }
            7 => {
                let a = prog[(ip + 1) as usize];
                let b = prog[(ip + 2) as usize];
                let c = prog[(ip + 3) as usize];
                let val_a = if mode_a == 0 { prog[a as usize] } else { a };
                let val_b = if mode_b == 0 { prog[b as usize] } else { b };
                // c is a position, no mode
                if val_a < val_b {
                    prog[c as usize] = 1;
                } else {
                    prog[c as usize] = 0;
                }
                ip += 4;
            }
            8 => {
                let a = prog[(ip + 1) as usize];
                let b = prog[(ip + 2) as usize];
                let c = prog[(ip + 3) as usize];
                let val_a = if mode_a == 0 { prog[a as usize] } else { a };
                let val_b = if mode_b == 0 { prog[b as usize] } else { b };
                // c is a position, no mode
                if val_a == val_b {
                    prog[c as usize] = 1;
                } else {
                    prog[c as usize] = 0;
                }
                ip += 4;
            }
            _ => unimplemented!("Unknown opcode {}", op),
        }
    }
    output
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let prog = parse(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        assert_eq!(calculate(prog.clone(), 7), 999);
        assert_eq!(calculate(prog.clone(), 8), 1000);
        assert_eq!(calculate(prog.clone(), 9), 1001);
    }
}
