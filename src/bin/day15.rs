//! https://adventofcode.com/2019/day/15

use advent_of_code_2019::*;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/2019/day15.txt").trim();
    let code = Intcode::parse(input);

    println!("Part 1: {}", solve1(code.clone()));
    println!("Part 2: {}", solve2(code.clone()));
}

fn solve1(mut code: Intcode) -> usize {
    find_oxygen(&mut code).2
}

fn solve2(mut code: Intcode) -> usize {
    let (mut map, oxygen, _) = find_oxygen(&mut code);

    let mut check = Vec::new();
    check.push(oxygen);

    let mut next_check = Vec::new();

    let mut minutes = 0;

    loop {
        while let Some(Point(x, y)) = check.pop() {
            for point in vec![
                Point(x, y + 1),
                Point(x, y - 1),
                Point(x + 1, y),
                Point(x - 1, y),
            ] {
                if let Some(&t) = map.get(&point) {
                    match t {
                        0 => {}
                        1 => {
                            // Empty, fill with oxygen and check others
                            map.insert(point.clone(), 2);
                            next_check.push(point);
                        }
                        2 => {
                            // Oxygen already, ignore
                        }
                        other => panic!("Unexpected tile {}", other),
                    }
                }
            }
        }
        if !next_check.is_empty() {
            check.append(&mut next_check);
        } else {
            break;
        }
        minutes += 1;
    }
    minutes
}

fn find_oxygen(code: &mut Intcode) -> (HashMap<Point, i64>, Point, usize) {
    let mut steps = Vec::new();
    for dir in vec![1, 2, 3, 4] {
        steps.push(Step::Back(back(dir)));
        steps.push(Step::Try(dir));
    }

    let mut map = HashMap::new();

    let mut point = Point(0, 0);
    // Could be a linked hash set I think
    let mut path = Vec::new();
    path.push(point.clone());

    let mut oxygen_steps = 0;
    let mut oxygen = Point(0, 0);

    while let Some(step) = steps.pop() {
        match step {
            Step::Try(direction) => {
                code.add_input(direction);
                let output = code.run_expect_output();
                match output {
                    // The repair droid hit a wall. Its position has not changed.
                    0 => {
                        // Remove the back direction, we haven't moved
                        steps.pop();

                        map.insert(point.next(direction), 0);
                    }
                    // The repair droid has moved one step in the requested direction.
                    1 | 2 => {
                        point = point.next(direction);

                        if output == 2 {
                            // If we just wanted to find the oxygen, we could return here.
                            // But we also want the whole map, so keep walking.
                            oxygen_steps = path.len();
                            oxygen = point.clone();
                        }

                        map.insert(point.clone(), output);

                        path.push(point.clone());
                        for dir in vec![1, 2, 3, 4] {
                            // Avoid loops
                            if !path.contains(&point.next(dir)) {
                                steps.push(Step::Back(back(dir)));
                                steps.push(Step::Try(dir));
                            }
                        }
                    }
                    other => panic!("Unexpected output {}", other),
                }
            }
            Step::Back(direction) => {
                code.add_input(direction);
                let output = code.run_expect_output();
                assert_eq!(output, 1);

                point = point.next(direction);
                path.pop();
            }
        }
    }
    (map, oxygen, oxygen_steps)
}

#[derive(Debug)]
enum Step {
    Try(i64),
    Back(i64),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point(i64, i64);

impl Point {
    fn next(&self, direction: i64) -> Point {
        match direction {
            // north (1), south (2), west (3), and east (4)
            1 => Point(self.0, self.1 + 1),
            2 => Point(self.0, self.1 - 1),
            3 => Point(self.0 - 1, self.1),
            4 => Point(self.0 + 1, self.1),
            other => panic!("Unexpected direction {}", other),
        }
    }
}

fn back(direction: i64) -> i64 {
    // north (1), south (2), west (3), and east (4)
    match direction {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        other => panic!("Unexpected direction {}", other),
    }
}

// Tried recursive approach first, but didn't work due to:
// thread 'main' has overflowed its stack
fn go_recursive(code: &mut Intcode, steps: u64) -> Option<u64> {
    for direction in vec![1, 2, 3, 4] {
        code.add_input(direction);
        let output = code.run_expect_output();
        match output {
            // The repair droid hit a wall. Its position has not changed.
            0 => {}
            // The repair droid has moved one step in the requested direction.
            1 => {
                if let Some(steps) = go_recursive(code, steps + 1) {
                    return Some(steps);
                }
                code.add_input(back(direction));
                let output_back = code.run_expect_output();
                assert_eq!(output_back, 1);
            }
            // The repair droid has moved one step in the requested direction;
            // its new position is the location of the oxygen system.
            2 => {
                return Some(steps + 1);
            }
            other => panic!("Unexpected output {}", other),
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let s = include_str!("../../input/2019/day15.txt").trim();
        let code = Intcode::parse(s);

        assert_eq!(solve1(code), 374);
    }
}
