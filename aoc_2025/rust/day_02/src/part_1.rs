use crate::*;

fn is_valid(number: i64) -> bool {
    let s = number.to_string();
    if s.len() % 2 != 0 {
        // odd-length numbers are always valid
        return true;
    }

    let l = s.len() / 2;
    if s.len() % l != 0 {
        return true;
    }

    let substr = &s[0..l];
    let substr2 = &s[l..s.len()];
    if substr == substr2 {
        return false;
    }

    true
}

pub fn part_1() -> i64 {
    let ranges = parse_input();

    let mut result = 0;

    for range in ranges {
        for n in range {
            if !is_valid(n) {
                result += n;
            }
        }
    }

    return result;
}
