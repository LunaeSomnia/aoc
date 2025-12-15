use std::time::Instant;

static INPUT: &str = include_str!("../../../inputs/day_3.input");

pub mod part_1;
use part_1::*;

pub mod part_2;
use part_2::*;

fn parse_input() -> Vec<Vec<u8>> {
    INPUT
        .lines()
        .map(|v| {
            v.chars()
                .map(|c| c.to_digit(10).expect(&format!("Invalid number: {}", c)) as u8)
                .collect()
        })
        .collect()
}

fn main() {
    let now = Instant::now();
    let part_1_result = part_1();
    let after_part_1 = Instant::now();
    let part_1_time = ((after_part_1 - now).as_micros() as f32) / 1000f32;
    println!("Day 3, Part 1: {} ({}ms)", part_1_result, part_1_time);

    let now = Instant::now();
    let part_2_result = part_2();
    let after_part_2 = Instant::now();
    let part_2_time = ((after_part_2 - now).as_micros() as f32) / 1000f32;
    println!("Day 3, Part 2: {} ({}ms)", part_2_result, part_2_time);
}
