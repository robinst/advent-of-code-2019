use std::collections::LinkedList;

#[derive(Clone)]
pub struct Intcode {
    prog: Vec<i32>,
    ip: i32,
    inputs: LinkedList<i32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Result {
    Output(i32),
    Halt,
}

impl Intcode {
    pub fn parse(instructions: &str) -> Self {
        let prog = instructions
            .split(',')
            .map(|s| {
                s.trim()
                    .parse()
                    .unwrap_or_else(|_| panic!("Error parsing {:?}", s))
            })
            .collect();
        Self::new(prog)
    }

    pub fn new(prog: Vec<i32>) -> Self {
        Self {
            prog,
            ip: 0,
            inputs: LinkedList::new(),
        }
    }

    pub fn add_input(&mut self, input: i32) -> &mut Self {
        self.inputs.push_back(input);
        self
    }

    pub fn run_until_halt(&mut self) -> i32 {
        let mut output = 0;
        while let Result::Output(o) = self.run() {
            output = o;
        }
        output
    }

    pub fn run(&mut self) -> Result {
        loop {
            let instruction = self.get(self.ip);
            let op = instruction % 100;
            let mut modes = instruction / 100;
            let mode_a = modes % 10;
            modes /= 10;
            let mode_b = modes % 10;

            match op {
                99 => break,
                // TODO: Abstract over instructions, so that we can maybe inspect/modify programs too?
                1 => {
                    let a = self.param(self.ip + 1, mode_a);
                    let b = self.param(self.ip + 2, mode_b);
                    let t = self.get(self.ip + 3);
                    self.set(t, a + b);
                    self.ip += 4;
                }
                2 => {
                    let a = self.param(self.ip + 1, mode_a);
                    let b = self.param(self.ip + 2, mode_b);
                    let t = self.get(self.ip + 3);
                    self.set(t, a * b);
                    self.ip += 4;
                }
                3 => {
                    // input
                    let t = self.get(self.ip + 1);
                    if let Some(input) = self.inputs.pop_front() {
                        self.set(t, input);
                    } else {
                        panic!("Expected input but didn't have any");
                    }
                    self.ip += 2;
                }
                4 => {
                    // output
                    let a = self.param(self.ip + 1, mode_a);
                    self.ip += 2;
                    return Result::Output(a);
                }
                5 => {
                    // jump-if-true
                    let a = self.param(self.ip + 1, mode_a);
                    let b = self.param(self.ip + 2, mode_b);
                    if a != 0 {
                        self.ip = b;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    // jump-if-false
                    let a = self.param(self.ip + 1, mode_a);
                    let b = self.param(self.ip + 2, mode_b);
                    if a == 0 {
                        self.ip = b;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    // less than
                    let a = self.param(self.ip + 1, mode_a);
                    let b = self.param(self.ip + 2, mode_b);
                    let t = self.get(self.ip + 3);
                    if a < b {
                        self.set(t, 1);
                    } else {
                        self.set(t, 0);
                    }
                    self.ip += 4;
                }
                8 => {
                    // equals
                    let a = self.param(self.ip + 1, mode_a);
                    let b = self.param(self.ip + 2, mode_b);
                    let t = self.get(self.ip + 3);
                    if a == b {
                        self.set(t, 1);
                    } else {
                        self.set(t, 0);
                    }
                    self.ip += 4;
                }
                _ => unimplemented!("Unknown opcode {}", op),
            }
        }
        Result::Halt
    }

    fn get(&self, p: i32) -> i32 {
        self.prog[p as usize]
    }

    fn set(&mut self, p: i32, value: i32) {
        self.prog[p as usize] = value;
    }

    fn param(&self, p: i32, mode: i32) -> i32 {
        let val = self.get(p);
        if mode == 0 {
            self.get(val)
        } else {
            val
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_examples() {
        let code = Intcode::parse(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        assert_eq!(code.clone().add_input(7).run(), Result::Output(999));
        assert_eq!(code.clone().add_input(8).run(), Result::Output(1000));
        assert_eq!(code.clone().add_input(9).run(), Result::Output(1001));
    }

    #[test]
    fn test_day5_input() {
        let input = include_str!("../input/2019/day05.txt");
        let code = Intcode::parse(input);

        assert_eq!(code.clone().add_input(1).run_until_halt(), 5346030);
        assert_eq!(code.clone().add_input(5).run_until_halt(), 513116);
    }
}
