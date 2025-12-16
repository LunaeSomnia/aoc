use crate::*;

fn parse_input() -> Vec<Problem> {
    let repeated_spaces_regex: Regex =
        Regex::new(r"  +").expect("Couldnt create 'repeated_spaces_regex' Regex");

    let mut problems: Vec<Problem> = Vec::new();

    for line in INPUT
        .lines()
        .map(|v| v.trim())
        .map(|v| repeated_spaces_regex.replace_all(v, " "))
    {
        let curr = line.chars().nth(0).unwrap();
        if curr == '+' || curr == '*' {
            let operations = line.split(" ").map(|v| match v {
                "+" => Operation::Add,
                "*" => Operation::Multiply,
                _ => panic!("Invalid operation: {}", v),
            });
            for (i, operation) in operations.into_iter().enumerate() {
                problems[i].operation = operation;
            }
        } else {
            let numbers: Vec<i64> = line
                .split(" ")
                .map(|v| {
                    v.parse::<i64>()
                        .expect(&format!("Error while parsing number: {}", v))
                })
                .collect();

            if problems.len() == 0 {
                for number in numbers {
                    problems.push(Problem {
                        numbers: vec![number],
                        operation: Operation::Add,
                    });
                }
            } else {
                for (i, number) in numbers.into_iter().enumerate() {
                    problems[i].numbers.push(number);
                }
            }
        }
    }

    problems
}

pub fn part_1() -> i64 {
    let input = parse_input();

    let mut sum = 0;

    for problem in input {
        match problem.operation {
            Operation::Add => sum += problem.numbers.iter().fold(0, |acc, v| acc + v),
            Operation::Multiply => sum += problem.numbers.iter().fold(1, |acc, v| acc * v),
        }
    }

    return sum;
}
