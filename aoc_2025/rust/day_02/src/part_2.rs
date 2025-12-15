use crate::*;

fn is_valid(number: i64) -> bool {
    let s = number.to_string();

    for l in 1..=(s.len() / 2) {
        if s.len() % l != 0 {
            continue;
        }

        let mut valid = true;
        let substr = &s[0..l];
        for j in 1..s.len() / l {
            let substr2 = &s[(j * l)..(j * l + l)];
            if substr != substr2 {
                valid = false;
                break;
            }
        }

        if valid {
            return false;
        }
    }

    true
}

pub fn part_2() -> i64 {
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
