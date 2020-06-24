//! https://adventofcode.com/2019/day/18

use std::collections::{HashMap, HashSet};
use std::fmt;

fn main() {
    let input = include_str!("../../input/2019/day18.txt").trim();

    println!("Part 1: {}", solve1(input));
}

fn solve1(input: &str) -> usize {
    let (map, objects) = parse(input);

    let number_of_keys = objects
        .keys()
        .filter(|c| match c {
            'a'..='z' => true,
            _ => false,
        })
        .count();
    dbg!(number_of_keys);

    let ways = explore(&map, objects);
    dbg!(&ways);
    let start = '@';

    let mut visited = HashSet::new();
    visited.insert(start);

    find_shortest(&ways, number_of_keys, start, HashSet::new(), visited, 0).unwrap()
    //
    //    for (point, steps) in find_keys(&map, &start) {
    //        let key = map.get(&point).unwrap();
    //        let door = doors.get(&key.to_ascii_uppercase()).unwrap();
    //
    //        let mut map_with_opened_door = map.clone();
    //        map_with_opened_door.set(door, '.');
    //
    //
    //
    //        map_with_opened_door
    //    }
    //
    //    println!("{:?}", find_keys(&map, &start));
    //    0
}

fn find_shortest(
    ways: &HashMap<char, Vec<(char, usize)>>,
    number_of_keys: usize,
    start: char,
    keys: HashSet<char>,
    visited: HashSet<char>,
    steps_so_far: usize,
) -> Option<usize> {
    if keys.len() == number_of_keys {
        return Some(steps_so_far);
    }

    let mut shortest: Option<usize> = None;

    let destinations = ways.get(&start).unwrap();
    for &(object, steps) in destinations {
        let steps = if object == '@'
            || keys.contains(&object)
            || keys.contains(&object.to_ascii_lowercase())
        {
            // Already open, try destinations from there but don't loop back on this.
            if visited.contains(&object) {
                continue;
            }
            let mut visited = visited.clone();
            visited.insert(object);
            find_shortest(
                ways,
                number_of_keys,
                object,
                keys.clone(),
                visited,
                steps_so_far + steps,
            )
        } else if object.is_ascii_lowercase() {
            // New key, collect it and continue
            let mut keys = keys.clone();
            keys.insert(object);
            find_shortest(
                ways,
                number_of_keys,
                object,
                keys,
                HashSet::new(),
                steps_so_far + steps,
            )
        } else {
            continue;
        };

        if let Some(steps) = steps {
            shortest = shortest.map(|s| s.min(steps)).or(Some(steps));
        }
    }

    shortest
}

fn explore(map: &Map<char>, objects: HashMap<char, Point>) -> HashMap<char, Vec<(char, usize)>> {
    let mut ways = HashMap::new();
    for (c, p) in objects {
        ways.insert(c, find_objects(map, &p));
    }
    ways
}

fn find_objects(map: &Map<char>, start: &Point) -> Vec<(char, usize)> {
    let mut objects = Vec::new();

    let mut check = Vec::new();
    check.push(start.clone());

    let mut visited = HashSet::new();
    visited.insert(start.clone());

    let mut next_check = Vec::new();

    let mut steps = 1;

    while !check.is_empty() {
        while let Some(middle) = check.pop() {
            for point in middle.adjacent() {
                if visited.contains(&point) {
                    continue;
                }
                if let Some(&c) = map.get(&point) {
                    match c {
                        '.' => next_check.push(point.clone()),
                        '@' | 'a'..='z' | 'A'..='Z' => {
                            objects.push((c, steps));
                        }
                        '#' => {
                            // Wall, ignore
                        }
                        other => panic!("Unexpected tile {}", other),
                    }
                }
                visited.insert(point);
            }
        }
        check.append(&mut next_check);
        steps += 1;
    }

    objects
}

fn find_keys(map: &Map<char>, start: &Point, treat_empty: &HashSet<char>) -> Vec<(Point, usize)> {
    let mut keys = Vec::new();

    let mut check = Vec::new();
    check.push(start.clone());

    let mut visited = HashSet::new();
    visited.insert(start.clone());

    let mut next_check = Vec::new();

    let mut steps = 1;

    loop {
        while let Some(middle) = check.pop() {
            for point in middle.adjacent() {
                if visited.contains(&point) {
                    continue;
                }
                if let Some(&c) = map.get(&point) {
                    match c {
                        '.' | '@' => next_check.push(point.clone()),
                        'a'..='z' => {
                            // key
                            if treat_empty.contains(&c) {
                                next_check.push(point.clone());
                            } else {
                                keys.push((point.clone(), steps));
                            }
                            //                            next_check.push(point.clone());
                        }
                        '#' => {
                            // Wall, ignore
                        }
                        'A'..='Z' => {
                            // Door
                            if treat_empty.contains(&c) {
                                next_check.push(point.clone());
                            }
                        }
                        other => panic!("Unexpected tile {}", other),
                    }
                }
                visited.insert(point);
            }
        }
        if !next_check.is_empty() {
            check.append(&mut next_check);
        } else {
            break;
        }
        steps += 1;
    }
    keys
}

fn parse(input: &str) -> (Map<char>, HashMap<char, Point>) {
    let mut map = Vec::new();
    let mut objects = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(c);
            match c {
                '@' => {
                    objects.insert(c, Point::new_from_usize(x, y));
                }
                'A'..='Z' => {
                    objects.insert(c, Point::new_from_usize(x, y));
                }
                'a'..='z' => {
                    objects.insert(c, Point::new_from_usize(x, y));
                }
                _ => {}
            }
        }
        map.push(row);
    }

    (Map { rows: map }, objects)
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

    fn new_from_usize(x: usize, y: usize) -> Self {
        Self {
            x: x as i64,
            y: y as i64,
        }
    }

    fn adjacent(&self) -> Vec<Point> {
        vec![
            Point::new(self.x, self.y - 1),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y + 1),
            Point::new(self.x - 1, self.y),
        ]
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Map<T> {
    rows: Vec<Vec<T>>,
}

impl<T> Map<T> {
    fn get(&self, p: &Point) -> Option<&T> {
        self.rows
            .get(p.y as usize)
            .and_then(|row| row.get(p.x as usize))
    }

    fn set(&mut self, p: &Point, t: T) {
        self.rows[p.y as usize][p.x as usize] = t;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        //        let s = "
        //########################
        //#f.D.E.e.C.b.A.@.a.B.c.#
        //######################.#
        //#d.....................#
        //########################
        //";
        //        assert_eq!(solve1(s), 86);
        //
        //        let s = "
        //########################
        //#...............b.C.D.f#
        //#.######################
        //#.....@.a.B.c.d.A.e.F.g#
        //########################
        //";
        //        assert_eq!(solve1(s), 132);

        let s = "
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################
";
        assert_eq!(solve1(s), 136);

        //        let s = "
        //########################
        //#@..............ac.GI.b#
        //###d#e#f################
        //###A#B#C################
        //###g#h#i################
        //########################
        //";
        //        assert_eq!(solve1(s), 81);
    }

    #[test]
    fn test_input() {
        let s = include_str!("../../input/2019/day18.txt").trim();

        //        assert_eq!(solve1(s));
    }
}
