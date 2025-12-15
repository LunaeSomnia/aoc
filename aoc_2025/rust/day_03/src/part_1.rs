use crate::*;

fn find_two_largest_numbers(vec: &Vec<u8>) -> u8 {
    let mut current = Vec::new();
    current.push(vec[0]);
    current.push(vec[1]);
    let mut max = current[0] * 10 + current[1];

    for n in 2..vec.len() {
        let current_stack_number = current[0] * 10 + current[1];
        let possible_1 = current[0] * 10 + vec[n] as u8;
        let possible_2 = current[1] * 10 + vec[n] as u8;

        if possible_1 > current_stack_number && possible_1 >= possible_2 {
            current[1] = vec[n] as u8;
            max = possible_1;
        } else if possible_2 > current_stack_number && possible_2 >= possible_1 {
            current[0] = current[1];
            current[1] = vec[n] as u8;
            max = possible_2;
        }
    }

    max
}

pub fn part_1() -> i32 {
    let banks = parse_input();

    let mut sum = 0i32;

    for bank in banks {
        let joltage = find_two_largest_numbers(&bank);
        sum += joltage as i32;
    }

    return sum;
}
