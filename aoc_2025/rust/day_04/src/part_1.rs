use crate::*;

pub fn part_1() -> i32 {
    let input = parse_input();

    let mut sum = 0;
    let mut valid_positions: Vec<(usize, usize)> = Vec::new();

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] != '@' {
                continue;
            }

            if valid_position(&input, x, y) {
                sum += 1;
                valid_positions.push((x, y));
            } else {
            }
        }
    }

    return sum;
}
