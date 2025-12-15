use crate::*;

pub fn part_1() -> i32 {
    let rotations = parse_input();

    let mut result = 0;
    let mut dial = 50i32;

    for rotation in rotations {
        match rotation.direction {
            Direction::Left => dial -= rotation.amount,
            Direction::Right => dial += rotation.amount,
        }

        dial %= 100;

        if dial == 0 {
            result += 1;
        }
    }

    return result;
}
