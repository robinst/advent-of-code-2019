//! https://adventofcode.com/2019/day/17

use advent_of_code_2019::*;
use std::collections::HashSet;
use std::fmt;

fn main() {
    let input = include_str!("../../input/2019/day17.txt").trim();
    let code = Intcode::parse(input);

    print(code.clone());

    println!("Part 1: {}", solve1(code.clone()));
    println!("Part 2: {}", solve2(code.clone()));
}

fn print(mut code: Intcode) {
    while let Result::Output(output) = code.run() {
        print!("{}", char::from(output as u8));
    }
}

fn solve1(code: Intcode) -> i64 {
    let (_robot, map) = map(code);

    let mut result = 0;
    for point in &map {
        if point.adjacent().iter().all(|p| map.contains(p)) {
            result += point.x * point.y;
        }
    }
    result
}

fn solve2(mut code: Intcode) -> i64 {
    let (mut robot, map) = map(code.clone());

    let mut direction = Direction::Up;
    let mut turn;

    let mut movements = Vec::new();
    loop {
        if map.contains(&robot.next(&direction.turn_left())) {
            turn = 'L';
            direction = direction.turn_left();
        } else if map.contains(&robot.next(&direction.turn_right())) {
            turn = 'R';
            direction = direction.turn_right();
        } else {
            // End of map
            break;
        }

        let mut steps = 0;
        while map.contains(&robot.next(&direction)) {
            robot = robot.next(&direction);
            steps += 1;
        }

        movements.push(Movement { turn, steps });
    }

    println!("{:?}", movements);

    // Did this manually, eh :).
    let commands = "A,B,B,A,C,B,C,C,B,A
R,10,R,8,L,10,L,10
R,8,L,6,L,6
L,10,R,10,L,6
n
";
    dbg!(commands);

    code.prog[0] = 2;
    for c in commands.chars() {
        let ascii = c as u8;
        code.add_input(ascii as i64);
    }

    let mut dust = 0;
    while let Result::Output(output) = code.run() {
        if output <= 127 {
            print!("{}", char::from(output as u8));
        }
        dust = output;
    }
    dust
}

fn map(mut code: Intcode) -> (Point, HashSet<Point>) {
    let mut set = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    let mut robot = None;
    while let Result::Output(output) = code.run() {
        let c = char::from(output as u8);
        match c {
            '#' => {
                set.insert(Point::new(x, y));
                x += 1;
            }
            '.' => {
                x += 1;
            }
            '^' => {
                robot = Some(Point::new(x, y));
                x += 1;
            }
            '\n' => {
                y += 1;
                x = 0;
            }
            _ => panic!("Unexpected output {:?}", c),
        }
    }
    (robot.expect("Robot not found"), set)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn adjacent(&self) -> [Point; 4] {
        [
            Point::new(self.x, self.y - 1),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y + 1),
            Point::new(self.x - 1, self.y),
        ]
    }

    fn next(&self, direction: &Direction) -> Point {
        match direction {
            Direction::Up => Point::new(self.x, self.y - 1),
            Direction::Right => Point::new(self.x + 1, self.y),
            Direction::Down => Point::new(self.x, self.y + 1),
            Direction::Left => Point::new(self.x - 1, self.y),
        }
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Movement {
    turn: char,
    steps: usize,
}

impl fmt::Debug for Movement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.turn, self.steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let s = include_str!("../../input/2019/day17.txt").trim();
        let code = Intcode::parse(s);

        assert_eq!(solve1(code), 5620);
        assert_eq!(solve2(code), 768115);
    }
}
