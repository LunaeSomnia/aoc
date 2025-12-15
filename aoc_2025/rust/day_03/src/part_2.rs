use crate::*;

const VEC_SIZE: usize = 12;

fn u8_vec_to_string(vec: &Vec<u8>) -> i64 {
    let mut num = 0;
    for (i, n) in vec.iter().enumerate() {
        num += 10i64.pow((vec.len() - 1 - i) as u32) * (*n) as i64;
    }
    num
}

// Same as in part_1, but generalized to any size we want (in this case, 12)
fn find_two_largest_numbers(vec: &Vec<u8>) -> i64 {
    let mut current = (&vec[0..VEC_SIZE]).to_vec();

    for n in VEC_SIZE..vec.len() {
        let current_value = u8_vec_to_string(&current);
        let incoming = vec[n];

        let mut max: Vec<u8> = Vec::new();
        let mut max_value = 0;
        let mut max_i: usize = VEC_SIZE;

        // For each position, we try removing each position and pushing the new value. Then checking whether THAT new value is better
        for i in 0..VEC_SIZE {
            let mut changed = current.clone();
            changed.remove(i);
            changed.push(incoming);
            let possible_new_max = u8_vec_to_string(&changed);
            if possible_new_max > current_value && possible_new_max > max_value {
                max_i = i;
                max = changed;
                max_value = possible_new_max;
            }
        }

        if max_i != VEC_SIZE {
            current = max;
        }
    }

    u8_vec_to_string(&current)
}

pub fn part_2() -> i64 {
    let banks = parse_input();

    let mut sum = 0i64;

    for bank in banks {
        let joltage = find_two_largest_numbers(&bank);
        sum += joltage as i64;
    }

    return sum;
}

#[test]
fn test_u8_vec_to_string() {
    assert_eq!(u8_vec_to_string(&vec![5, 8, 0]), 580);
    assert_eq!(u8_vec_to_string(&vec![2]), 2);
}
