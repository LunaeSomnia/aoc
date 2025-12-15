use std::time::Instant;

static INPUT: &str = include_str!("../../../inputs/day_5.input");

pub mod part_1;
use part_1::*;

pub mod part_2;
use part_2::*;

#[derive(Debug)]
struct InputData {
    fresh_ranges: Vec<std::ops::RangeInclusive<usize>>,
    ingredients: Vec<usize>,
}

fn parse_input() -> InputData {
    let mut lines = INPUT.lines();
    let mut input_data = InputData {
        fresh_ranges: Vec::new(),
        ingredients: Vec::new(),
    };

    loop {
        let Some(line) = lines.next() else {
            break;
        };
        if line.len() == 0 {
            break;
        }

        let split: Vec<&str> = line.split("-").collect();
        let Ok(starting_id) = split[0].parse::<usize>() else {
            panic!("Error while parsing number");
        };
        let Ok(ending_id) = split[1].parse::<usize>() else {
            panic!("Error while parsing number");
        };

        input_data.fresh_ranges.push(starting_id..=ending_id);
    }

    loop {
        let Some(line) = lines.next() else {
            break;
        };
        let Ok(ingredient_id) = line.parse::<usize>() else {
            panic!("Error while parsing number");
        };

        input_data.ingredients.push(ingredient_id);
    }

    input_data
}

fn main() {
    let now = Instant::now();
    let part_1_result = part_1();
    let after_part_1 = Instant::now();
    let part_1_time = ((after_part_1 - now).as_micros() as f32) / 1000f32;
    println!("Day 5, Part 1: {} ({}ms)", part_1_result, part_1_time);

    let now = Instant::now();
    let part_2_result = part_2();
    let after_part_2 = Instant::now();
    let part_2_time = ((after_part_2 - now).as_micros() as f32) / 1000f32;
    println!("Day 5, Part 2: {} ({}ms)", part_2_result, part_2_time);
}
