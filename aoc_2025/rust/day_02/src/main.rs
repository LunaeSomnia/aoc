use std::time::Instant;

static INPUT: &str = include_str!("../../../inputs/day_2.input");

pub mod part_1;
use part_1::*;

pub mod part_2;
use part_2::*;

fn parse_input() -> Vec<std::ops::RangeInclusive<i64>> {
    INPUT
        .split(",")
        .map(|v| {
            let id_ranges: Vec<&str> = v.split("-").collect();
            assert_eq!(id_ranges.len(), 2);

            let Ok(range_start) = id_ranges[0].trim().parse::<i64>() else {
                panic!("Failed to deserialize range start: '{}'", id_ranges[0])
            };
            let Ok(range_end) = id_ranges[1].trim().parse::<i64>() else {
                panic!("Failed to deserialize range end: '{}'", id_ranges[1])
            };
            range_start..=range_end
        })
        .collect()
}

fn main() {
    let now = Instant::now();
    let part_1_result = part_1();
    let after_part_1 = Instant::now();
    let part_1_time = ((after_part_1 - now).as_micros() as f32) / 1000f32;
    println!("Day 2, Part 1: {} ({}ms)", part_1_result, part_1_time);

    let now = Instant::now();
    let part_2_result = part_2();
    let after_part_2 = Instant::now();
    let part_2_time = ((after_part_2 - now).as_micros() as f32) / 1000f32;
    println!("Day 2, Part 2: {} ({}ms)", part_2_result, part_2_time);
}
