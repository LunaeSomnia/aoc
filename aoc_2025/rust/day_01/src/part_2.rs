use crate::*;

pub fn part_2() -> i32 {
    let rotations = parse_input();

    let mut result = 0;
    let mut dial = 50i32;

    for rotation in rotations {
        match rotation.direction {
            Direction::Left => {
                result += rotation.amount / 100;
                if dial != 0 && rotation.amount % 100 >= dial {
                    result += 1;
                }
                dial = (dial - rotation.amount).rem_euclid(100);
            }
            Direction::Right => {
                dial += rotation.amount;
                result += dial / 100;
                dial = dial.rem_euclid(100);
            }
        };
    }

    return result;
}
