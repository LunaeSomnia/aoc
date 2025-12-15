use crate::*;

pub fn part_2() -> i32 {
    let mut input = parse_input();

    let mut sum = 0;

    loop {
        let mut valid_positions: Vec<(usize, usize)> = Vec::new();

        for y in 0..input.len() {
            for x in 0..input[0].len() {
                if input[y][x] != '@' {
                    continue;
                }

                if valid_position(&input, x, y) {
                    valid_positions.push((x, y));
                }
            }
        }

        if valid_positions.len() == 0 {
            break;
        }
        for (x, y) in &valid_positions {
            input[*y][*x] = 'o';
        }

        sum += valid_positions.len();
    }

    return sum as i32;
}
