//! https://adventofcode.com/2019/day/3

use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input/2019/day03.txt");
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let second = lines.next().unwrap();

    println!("part 1: {}", distance(first, second));
}

fn distance(a: &str, b: &str) -> i32 {
    let coords_a = coords(a);
    let coords_b = coords(b);
    let set_a: HashSet<_> = coords_a.iter().collect();
    let set_b: HashSet<_> = coords_b.iter().collect();

    let crosses = set_a.intersection(&set_b);
    let min = crosses
        .filter(|(x, y)| *x != 0 && *y != 0)
        .min_by_key(|&&a| manhattan(&(0, 0), a))
        .unwrap();

    manhattan(&(0, 0), min)
}

fn manhattan(a: &(i32, i32), b: &(i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn coords(ops: &str) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    let mut x = 0;
    let mut y = 0;
    for op in ops.split(',') {
        let (d, num) = op.split_at(1);
        let num = num.parse().unwrap();
        match d {
            "U" => {
                for _ in 0..num {
                    y += 1;
                    result.push((x, y));
                }
            }
            "D" => {
                for _ in 0..num {
                    y -= 1;
                    result.push((x, y));
                }
            }
            "L" => {
                for _ in 0..num {
                    x -= 1;
                    result.push((x, y));
                }
            }
            "R" => {
                for _ in 0..num {
                    x += 1;
                    result.push((x, y));
                }
            }
            _ => unimplemented!("Unknown direction {}", d),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(distance("R8,U5,L5,D3", "U7,R6,D4,L4"), 6);
        assert_eq!(
            distance(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            159
        );
    }
}
