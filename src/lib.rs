use std::collections::LinkedList;

#[derive(Clone)]
pub struct Intcode {
    pub prog: Vec<i64>,
    ip: i64,
    inputs: LinkedList<i64>,
    relative_base: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Result {
    Output(i64),
    NeedInput,
    Halt,
}

impl Intcode {
    pub fn parse(instructions: &str) -> Self {
        let prog = instructions
            .trim()
            .split(',')
            .map(|s| {
                s.trim()
                    .parse()
                    .unwrap_or_else(|_| panic!("Error parsing {:?}", s))
            })
            .collect();
        Self::new(prog)
    }

    pub fn new(prog: Vec<i64>) -> Self {
        Self {
            prog,
            ip: 0,
            inputs: LinkedList::new(),
            relative_base: 0,
        }
    }

    pub fn add_input(&mut self, input: i64) -> &mut Self {
        self.inputs.push_back(input);
        self
    }

    pub fn run_last(&mut self) -> i64 {
        let mut output = 0;
        while let Result::Output(o) = self.run() {
            output = o;
        }
        output
    }

    pub fn run_all(&mut self) -> Vec<i64> {
        let mut outputs = Vec::new();
        while let Result::Output(o) = self.run() {
            outputs.push(o);
        }
        outputs
    }

    pub fn run_expect_output(&mut self) -> i64 {
        match self.run() {
            Result::Output(o) => o,
            result => panic!("Expected output, got {:?}", result),
        }
    }

    pub fn run(&mut self) -> Result {
        loop {
            let instruction = self.get(self.ip);
            let op = instruction % 100;
            let mut modes = instruction / 100;
            let mode1 = modes % 10;
            modes /= 10;
            let mode2 = modes % 10;
            modes /= 10;
            let mode3 = modes % 10;

            match op {
                99 => break,
                // TODO: Abstract over instructions, so that we can maybe inspect/modify programs too?
                1 => {
                    let a = self.param(self.ip + 1, mode1);
                    let b = self.param(self.ip + 2, mode2);
                    self.store(self.ip + 3, mode3, a + b);
                    self.ip += 4;
                }
                2 => {
                    let a = self.param(self.ip + 1, mode1);
                    let b = self.param(self.ip + 2, mode2);
                    self.store(self.ip + 3, mode3, a * b);
                    self.ip += 4;
                }
                3 => {
                    // input
                    if let Some(input) = self.inputs.pop_front() {
                        self.store(self.ip + 1, mode1, input);
                    } else {
                        return Result::NeedInput;
                    }
                    self.ip += 2;
                }
                4 => {
                    // output
                    let a = self.param(self.ip + 1, mode1);
                    self.ip += 2;
                    return Result::Output(a);
                }
                5 => {
                    // jump-if-true
                    let a = self.param(self.ip + 1, mode1);
                    let b = self.param(self.ip + 2, mode2);
                    if a != 0 {
                        self.ip = b;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    // jump-if-false
                    let a = self.param(self.ip + 1, mode1);
                    let b = self.param(self.ip + 2, mode2);
                    if a == 0 {
                        self.ip = b;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    // less than
                    let a = self.param(self.ip + 1, mode1);
                    let b = self.param(self.ip + 2, mode2);
                    if a < b {
                        self.store(self.ip + 3, mode3, 1);
                    } else {
                        self.store(self.ip + 3, mode3, 0);
                    }
                    self.ip += 4;
                }
                8 => {
                    // equals
                    let a = self.param(self.ip + 1, mode1);
                    let b = self.param(self.ip + 2, mode2);
                    if a == b {
                        self.store(self.ip + 3, mode3, 1);
                    } else {
                        self.store(self.ip + 3, mode3, 0);
                    }
                    self.ip += 4;
                }
                9 => {
                    // adjusts the relative base
                    let a = self.param(self.ip + 1, mode1);
                    self.relative_base += a;
                    self.ip += 2;
                }
                _ => unimplemented!("Unknown opcode {}", op),
            }
        }
        Result::Halt
    }

    fn get(&mut self, p: i64) -> i64 {
        self.ensure_memory(p);
        self.prog[p as usize]
    }

    fn set(&mut self, p: i64, value: i64) {
        self.ensure_memory(p);
        self.prog[p as usize] = value;
    }

    fn ensure_memory(&mut self, p: i64) {
        let needed_size = p as usize + 1;
        if needed_size > self.prog.len() {
            self.prog.resize(needed_size, 0);
        }
    }

    fn param(&mut self, p: i64, mode: i64) -> i64 {
        let val = self.get(p);
        match mode {
            // position
            0 => self.get(val),
            // immediate
            1 => val,
            // relative
            2 => self.get(self.relative_base + val),
            _ => panic!("Unknown mode {} for param", mode),
        }
    }

    fn store(&mut self, p: i64, mode: i64, result: i64) {
        let val = self.get(p);
        match mode {
            0 => self.set(val, result),
            2 => self.set(self.relative_base + val, result),
            _ => panic!("Unknown mode {} for store", mode),
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

        assert_eq!(code.clone().add_input(1).run_last(), 5346030);
        assert_eq!(code.clone().add_input(5).run_last(), 513116);
    }

    #[test]
    fn test_day9_examples() {
        let code = Intcode::parse("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        assert_eq!(
            code.clone().run_all(),
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );

        let code = Intcode::parse("1102,34915192,34915192,7,4,7,99,0");
        assert_eq!(code.clone().run_last(), 1219070632396864);

        let code = Intcode::parse("104,1125899906842624,99");
        assert_eq!(code.clone().run_last(), 1125899906842624);
    }

    #[test]
    fn test_day9_input() {
        let input = include_str!("../input/2019/day09.txt");
        let code = Intcode::parse(input);

        assert_eq!(code.clone().add_input(1).run_all(), vec![2955820355]);
        assert_eq!(code.clone().add_input(2).run_all(), vec![46643]);
    }
}
