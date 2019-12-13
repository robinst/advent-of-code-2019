//! https://adventofcode.com/2019/day/12

use num::Integer;
use std::cmp::Ordering;
use std::collections::HashSet;

fn main() {
    let system = System::new(vec![
        Moon::new(-10, -13, 7),
        Moon::new(1, 2, 1),
        Moon::new(-15, -3, 13),
        Moon::new(3, 7, -4),
    ]);

    println!("{}", steps_to_repeat(system));
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Moon {
    pos: Point,
    vel: Point,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct System {
    moons: Vec<Moon>,
}

impl Point {
    fn energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            pos: Point { x, y, z },
            vel: Point { x: 0, y: 0, z: 0 },
        }
    }

    fn energy(&self) -> i32 {
        self.pos.energy() * self.vel.energy()
    }
}

impl System {
    fn new(moons: Vec<Moon>) -> Self {
        Self { moons }
    }

    fn step(&mut self) {
        let xs: Vec<_> = self.moons.iter().map(|moon| moon.pos.x).collect();
        let ys: Vec<_> = self.moons.iter().map(|moon| moon.pos.y).collect();
        let zs: Vec<_> = self.moons.iter().map(|moon| moon.pos.z).collect();
        for mut moon in &mut self.moons {
            moon.vel.x += Self::gravity(moon.pos.x, &xs);
            moon.vel.y += Self::gravity(moon.pos.y, &ys);
            moon.vel.z += Self::gravity(moon.pos.z, &zs);

            moon.pos.x += moon.vel.x;
            moon.pos.y += moon.vel.y;
            moon.pos.z += moon.vel.z;
        }
    }

    fn total_energy(&self) -> i32 {
        self.moons.iter().map(|moon| moon.energy()).sum()
    }

    fn gravity(n: i32, all: &[i32]) -> i32 {
        all.iter()
            .map(|o| match n.cmp(o) {
                Ordering::Less => 1,
                Ordering::Greater => -1,
                Ordering::Equal => 0,
            })
            .sum()
    }

    fn xs(&self) -> Vec<i32> {
        self.moons
            .iter()
            .flat_map(|moon| vec![moon.pos.x, moon.vel.x])
            .collect()
    }

    fn ys(&self) -> Vec<i32> {
        self.moons
            .iter()
            .flat_map(|moon| vec![moon.pos.y, moon.vel.y])
            .collect()
    }

    fn zs(&self) -> Vec<i32> {
        self.moons
            .iter()
            .flat_map(|moon| vec![moon.pos.z, moon.vel.z])
            .collect()
    }
}

fn steps_to_repeat(mut system: System) -> u64 {
    let initial_xs = system.xs();
    let initial_ys = system.ys();
    let initial_zs = system.zs();

    let mut x_steps = None;
    let mut y_steps = None;
    let mut z_steps = None;

    let mut steps: u64 = 0;
    loop {
        system.step();
        steps += 1;

        if system.xs() == initial_xs {
            x_steps = Some(steps);
        }
        if system.ys() == initial_ys {
            y_steps = Some(steps);
        }
        if system.zs() == initial_zs {
            z_steps = Some(steps);
        }

        if let (Some(x), Some(y), Some(z)) = (x_steps, y_steps, z_steps) {
            println!("{}, {}, {}", x, y, z);
            return x.lcm(&y).lcm(&z);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let mut system = System::new(vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ]);

        for _ in 0..10 {
            system.step();
            println!("{:?}", system);
        }

        assert_eq!(system.total_energy(), 179);
    }

    #[test]
    fn test_examples_part2() {
        let system = System::new(vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ]);
        let steps = steps_to_repeat(system);
        assert_eq!(steps, 2772);
    }

    #[test]
    fn test_input() {
        let mut system = System::new(vec![
            Moon::new(-10, -13, 7),
            Moon::new(1, 2, 1),
            Moon::new(-15, -3, 13),
            Moon::new(3, 7, -4),
        ]);

        let system2 = system.clone();

        for _ in 0..1000 {
            system.step();
            println!("{:?}", system);
        }

        assert_eq!(system.total_energy(), 8454);

        assert_eq!(steps_to_repeat(system2), 362336016722948);
    }
}
