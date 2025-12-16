use std::time::Instant;

static INPUT: &str = include_str!("../../../inputs/day_6.input");

pub mod part_1;
use part_1::*;

pub mod part_2;
use part_2::*;
use regex::Regex;

#[derive(Debug, Clone)]
pub enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub numbers: Vec<i64>,
    pub operation: Operation,
}

fn main() {
    let now = Instant::now();
    let part_1_result = part_1();
    let after_part_1 = Instant::now();
    let part_1_time = ((after_part_1 - now).as_micros() as f32) / 1000f32;
    println!("Day 6, Part 1: {} ({}ms)", part_1_result, part_1_time);

    let now = Instant::now();
    let part_2_result = part_2();
    let after_part_2 = Instant::now();
    let part_2_time = ((after_part_2 - now).as_micros() as f32) / 1000f32;
    println!("Day 6, Part 2: {} ({}ms)", part_2_result, part_2_time);
}
