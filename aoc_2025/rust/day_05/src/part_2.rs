use crate::*;

fn are_ranges_overlapping(
    r1: &std::ops::RangeInclusive<usize>,
    r2: &std::ops::RangeInclusive<usize>,
) -> bool {
    r1.start() <= r2.end() && r2.start() <= r1.end()
}

pub fn part_2() -> i64 {
    let input = parse_input();

    let mut ranges = input.fresh_ranges.clone();

    loop {
        let mut ranges_to_remove = Vec::new();
        'outer: for i in 0..ranges.len() {
            for j in 0..ranges.len() {
                if i == j {
                    continue;
                }

                let r1 = &ranges[i];
                let r2 = &ranges[j];

                if are_ranges_overlapping(r1, r2) {
                    ranges[i] = (*r1.start()).min(*r2.start())..=(*r1.end()).max(*r2.end());
                    ranges_to_remove.push(j);
                    break 'outer;
                }
            }
        }

        if ranges_to_remove.len() == 0 {
            break;
        }

        for i in ranges_to_remove {
            ranges.remove(i);
        }
    }

    let mut sum = 0;

    for range in ranges {
        sum += range.end() - range.start() + 1;
    }

    sum as i64
}
