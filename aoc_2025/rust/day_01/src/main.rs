use std::time::Instant;

static INPUT: &str = include_str!("../../../inputs/day_1.input");

pub mod part_1;
use part_1::*;

pub mod part_2;
use part_2::*;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    amount: i32,
}

fn parse_input() -> Vec<Rotation> {
    let mut rotations = Vec::new();
    for line in INPUT.lines() {
        let length = line.len();
        let direction = match line.chars().nth(0) {
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            c => panic!("Invalid direction: {:?}", c),
        };
        let amount_str = &line[1..length];
        let Ok(amount) = amount_str.parse::<i32>() else {
            panic!("Error while attempting to parse number");
        };

        rotations.push(Rotation { direction, amount })
    }
    rotations
}

fn main() {
    let now = Instant::now();
    let part_1_result = part_1();
    let after_part_1 = Instant::now();
    let part_1_time = ((after_part_1 - now).as_micros() as f32) / 1000f32;
    println!("Day 1, Part 1: {} ({}ms)", part_1_result, part_1_time);

    let now = Instant::now();
    let part_2_result = part_2();
    let after_part_2 = Instant::now();
    let part_2_time = ((after_part_2 - now).as_micros() as f32) / 1000f32;
    println!("Day 1, Part 2: {} ({}ms)", part_2_result, part_2_time);
}
