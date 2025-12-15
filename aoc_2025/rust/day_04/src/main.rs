use std::time::Instant;

static INPUT: &str = include_str!("../../../inputs/day_4.input");

pub mod part_1;
use part_1::*;

pub mod part_2;
use part_2::*;

fn valid_coordinate(n: i32, m: i32, x: i32, y: i32) -> bool {
    x >= 0 && x < n && y >= 0 && y < m
}

fn valid_position(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let n = input[0].len() as i32;
    let m = input.len() as i32;
    let mut roll_count = 0;
    for i in -1i32..=1i32 {
        for j in -1i32..=1i32 {
            if i == 0 && j == 0 {
                continue;
            }

            let x = x as i32 + i;
            let y = y as i32 + j;

            if !valid_coordinate(n, m, x, y) {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            if input[y][x] == '@' {
                roll_count += 1;
            }
        }
    }

    roll_count < 4
}

fn parse_input() -> Vec<Vec<char>> {
    INPUT.lines().map(|v| v.chars().collect()).collect()
}

fn main() {
    let now = Instant::now();
    let part_1_result = part_1();
    let after_part_1 = Instant::now();
    let part_1_time = ((after_part_1 - now).as_micros() as f32) / 1000f32;
    println!("Day 4, Part 1: {} ({}ms)", part_1_result, part_1_time);

    let now = Instant::now();
    let part_2_result = part_2();
    let after_part_2 = Instant::now();
    let part_2_time = ((after_part_2 - now).as_micros() as f32) / 1000f32;
    println!("Day 4, Part 2: {} ({}ms)", part_2_result, part_2_time);
}
