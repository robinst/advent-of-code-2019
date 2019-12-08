//! https://adventofcode.com/2019/day/8

use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/2019/day08.txt").trim();

    println!("Part 1: {}", product(input, 25, 6));
    print_image(input, 25, 6);
}

fn product(input: &str, width: usize, height: usize) -> usize {
    let length = width * height;
    let pixels: Vec<_> = input.chars().collect();

    let counts = pixels
        .chunks(length)
        .map(|layer| {
            let mut counts = HashMap::new();
            for pixel in layer {
                *counts.entry(pixel).or_insert(0) += 1;
            }
            counts
        })
        .min_by_key(|counts| counts[&'0'])
        .unwrap();
    counts[&'1'] * counts[&'2']
}

fn print_image(input: &str, width: usize, height: usize) {
    let length = width * height;
    let pixels: Vec<_> = input.chars().collect();

    let mut image = vec!['2'; length];

    pixels.chunks(length).for_each(|layer| {
        for (i, pixel) in layer.iter().enumerate() {
            if image[i] == '2' {
                image[i] = *pixel;
            }
        }
    });

    image.chunks(width).for_each(|line| {
        let s: String = line
            .iter()
            .map(|x| if x == &'1' { "##" } else { "  " })
            .collect();
        println!("{}", s);
    });
}
