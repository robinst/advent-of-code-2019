//! https://adventofcode.com/2019/day/1

fn main() {
    let input = include_str!("../../input/2019/day01.txt");
    let numbers: Vec<u64> = input.lines().map(|line| line.parse().expect(&format!("{:?}", line))).collect();

    let sum: u64 = numbers.iter().map(|mass| fuel(*mass)).sum();
    println!("Sum fuel: {}", sum);

    let sum: u64 = numbers.iter().map(|mass| recursive_fuel(*mass)).sum();
    println!("Sum fuel with fuel mass: {}", sum);
}

fn fuel(mass: u64) -> u64 {
    (mass / 3).checked_sub(2).unwrap_or(0)
}

fn recursive_fuel(mass: u64) -> u64 {
    let fuel_mass = fuel(mass);
    if fuel_mass > 0 {
        fuel_mass + recursive_fuel(fuel_mass)
    } else {
        fuel_mass
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!(fuel(2), 0);
        assert_eq!(recursive_fuel(1969), 966);
    }
}
