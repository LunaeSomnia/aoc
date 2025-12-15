use crate::*;

pub fn part_1() -> i32 {
    let input = parse_input();

    let mut sum = 0;

    for ingredient in input.ingredients {
        if input.fresh_ranges.iter().any(|v| v.contains(&ingredient)) {
            sum += 1;
        }
    }

    return sum;
}
