//! https://adventofcode.com/2019/day/10

use num::integer::gcd;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/2019/day10.txt").trim();

    println!("Part 1: {:?}", solve1(input));
    println!("Part 2: {:?}", solve2(input));
}

fn solve1(map: &str) -> usize {
    let asteroids = parse(map);

    asteroids
        .iter()
        .map(|(x, y)| group_by_angle((*x, *y), &asteroids).len())
        .max()
        .unwrap()
}

fn solve2(map: &str) -> i32 {
    let asteroids = parse(map);

    let ((x, y), mut angles) = asteroids
        .iter()
        .map(|&(x, y)| ((x, y), group_by_angle((x, y), &asteroids)))
        .max_by_key(|(_, angles)| angles.len())
        .unwrap();

    let mut count = 1;
    let mut keys: Vec<_> = angles.keys().cloned().collect();
    keys.sort_by_key(|&(h, v)| (((-(h as f64)).atan2(v as f64)) * 1000f64) as i64);
    dbg!(&keys);

    for key in keys {
        if angles[&key].is_empty() {
            continue;
        }

        let closest = angles[&key]
            .iter()
            .enumerate()
            .min_by_key(|(_, &(x2, y2))| (x - x2).abs() + (y - y2).abs())
            .map(|(i, _)| i)
            .unwrap();
        let (x2, y2) = angles.get_mut(&key).unwrap().remove(closest);
        if count == 200 {
            return x2 * 100 + y2;
        }

        count += 1;
    }
    0
}

fn group_by_angle(
    (x1, y1): (i32, i32),
    asteroids: &Vec<(i32, i32)>,
) -> HashMap<(i32, i32), Vec<(i32, i32)>> {
    let mut angles = HashMap::new();
    for &(x2, y2) in asteroids {
        if x2 == x1 && y2 == y1 {
            continue;
        }

        let h = x2 - x1;
        let v = y2 - y1;

        let (h, v) = reduce(h, v);
        angles.entry((h, v)).or_insert(Vec::new()).push((x2, y2));
    }
    angles
}

fn reduce(h: i32, v: i32) -> (i32, i32) {
    let divisor = gcd(h.abs(), v.abs());
    (h / divisor, v / divisor)
}

fn parse(map: &str) -> Vec<(i32, i32)> {
    let mut vec = Vec::new();
    for (y, line) in map.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                vec.push((x as i32, y as i32));
            }
        }
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
";
        assert_eq!(solve1(input), 33);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../../input/2019/day10.txt").trim();
        assert_eq!(solve1(input), 344);
        assert_eq!(solve2(input), 2732);
    }
}
