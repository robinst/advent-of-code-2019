//! https://adventofcode.com/2019/day/19

use advent_of_code_2019::*;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/2019/day19.txt").trim();
    let code = Intcode::parse(input);

    println!("Part 1: {}", solve1(code.clone()));
    let (x, y) = solve2(code.clone());
    println!("Part 2: {}", (x * 10000 + y));
}

fn solve1(code: Intcode) -> usize {
    let mut count = 0;
    for y in 0..50 {
        for x in 0..50 {
            let mut code = code.clone();
            code.add_input(x);
            code.add_input(y);
            if code.run_expect_output() == 1 {
                count += 1;
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    count
}

fn solve2(code: Intcode) -> (i64, i64) {
    let side_length = 100;
    let mut x = side_length;
    let mut y = side_length;

    let mut drone = Drone::new(code);

    // Find area where width and height are at least 100 first, otherwise no point checking if
    // a square fits. Not sure if we need this, but in the beginning the beam is not contiguous,
    // so makes the second loop easier.
    loop {
        if drone.check((x, y)) {
            println!("Hit: {}, {}", x, y);
            let width = drone.get_width((x, y));
            let height = drone.get_height((x, y));
            if width >= side_length && height >= side_length {
                println!("Wide enough: {}, {}", width, height);
                break;
            }

            x *= 2;
            y *= 2;
        } else {
            println!("Miss: {}, {}", x, y);
            if let Some((start_x, start_y)) = drone.find_start((x, y), (1, -1)) {
                // top right start of beam
                let (end_x, end_y) = drone.find_end((start_x, start_y), (1, -1));
                x = (start_x + end_x) / 2;
                y = (start_y + end_y) / 2;
            } else {
                if let Some((start_x, start_y)) = drone.find_start((x, y), (-1, 1)) {
                    let (end_x, end_y) = drone.find_end((start_x, start_y), (-1, 1));
                    x = (start_x + end_x) / 2;
                    y = (start_y + end_y) / 2;
                } else {
                    panic!("Couldn't find any beam from {}, {}", x, y);
                }
            }
        }
    }

    let mut best = None;
    let mut direction = 0;
    while drone.check((x, y)) {
        //        dbg!(x, y);

        if let Some((left, top)) = drone.find_box((x, y), side_length) {
            best = Some((left, top));
            if direction == 0 {
                println!("Square fits already, trying to find a better match closer to emitter");
                direction = -1;
            } else if direction == 1 {
                break;
            }
        } else {
            if direction == 0 {
                println!("Square doesn't fit yet, trying to find a match further from emitter");
                direction = 1;
            } else if direction == -1 {
                break;
            }
        }

        x += direction;
        // Make sure we're staying in the beam
        y = drone.find_start((x, y), (0, direction)).unwrap().1;
    }

    println!("Took {} checks", drone.checks());

    best.expect("No best square found")
}

struct Drone {
    code: Intcode,
    map: HashMap<(i64, i64), bool>,
    checks: usize,
}

impl Drone {
    fn new(code: Intcode) -> Drone {
        Drone {
            code,
            map: HashMap::new(),
            checks: 0,
        }
    }

    fn check(&mut self, point: (i64, i64)) -> bool {
        if let Some(&result) = self.map.get(&point) {
            result
        } else {
            let mut code = self.code.clone();
            code.add_input(point.0);
            code.add_input(point.1);
            let result = code.run_expect_output() == 1;
            self.checks += 1;
            self.map.insert(point, result);
            result
        }
    }

    fn checks(&self) -> usize {
        self.checks
    }

    fn get_width(&mut self, (x, y): (i64, i64)) -> i64 {
        self.find_end((x, y), (1, 0)).0 - self.find_end((x, y), (-1, 0)).0
    }

    fn get_height(&mut self, (x, y): (i64, i64)) -> i64 {
        self.find_end((x, y), (0, 1)).1 - self.find_end((x, y), (0, -1)).1
    }

    fn find_box(&mut self, (x, y): (i64, i64), side_length: i64) -> Option<(i64, i64)> {
        let (right, _) = self.find_end((x, y), (1, 0));
        let left = right - side_length;
        if self.check((left, y)) {
            let (_, bottom) = self.find_end((left, y), (0, 1));
            let height = bottom - y;
            if height >= side_length {
                Some((left, y))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn find_start(
        &mut self,
        (start_x, start_y): (i64, i64),
        (step_x, step_y): (i64, i64),
    ) -> Option<(i64, i64)> {
        let mut x = start_x;
        let mut y = start_y;

        while x >= 0 && y >= 0 {
            if self.check((x, y)) {
                return Some((x, y));
            }
            x += step_x;
            y += step_y;
        }

        None
    }

    fn find_end(
        &mut self,
        (start_x, start_y): (i64, i64),
        (step_x, step_y): (i64, i64),
    ) -> (i64, i64) {
        assert!(self.check((start_x, start_y)));

        let mut x = start_x;
        let mut y = start_y;

        while self.check((x, y)) {
            x += step_x;
            y += step_y;
        }

        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let s = include_str!("../../input/2019/day19.txt").trim();
        let code = Intcode::parse(s);

        assert_eq!(solve1(code.clone()), 110);
        assert_eq!(solve2(code.clone()), (1730, 2065));
    }
}
