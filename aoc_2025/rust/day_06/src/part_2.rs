use crate::*;

fn transpose_matrix<T>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }

    let m = matrix.len();
    let n = matrix.iter().map(|v| v.len()).max().unwrap();

    let mut transposed = vec![vec![]; n];

    for i in 0..n {
        for j in 0..m {
            transposed[i].push(matrix[j][n - i - 1].clone());
        }
    }

    transposed
}

fn parse_input() -> Vec<Problem> {
    let mut problems: Vec<Problem> = Vec::new();

    let mut lines: Vec<String> = INPUT.lines().map(|v| v.to_string()).collect();

    let n = lines.iter().map(|v| v.len()).max().unwrap();
    let m = lines.len();

    for line in lines.iter_mut() {
        while line.len() < n {
            line.push(' ');
        }
    }

    let chars: Vec<Vec<char>> = (&lines[0..(m - 1)])
        .iter()
        .map(|v| v.chars().collect())
        .collect();
    let transposed_chars = transpose_matrix(&chars);

    let mut numbers: Vec<Vec<i64>> = Vec::new();
    let mut current_numbers_vec = Vec::new();
    let number_lines: Vec<String> = transposed_chars
        .iter()
        .map(|v| v.iter().collect())
        .collect();
    for line in number_lines {
        if line.trim().is_empty() {
            numbers.push(current_numbers_vec.clone());
            current_numbers_vec = Vec::new();
            continue;
        }

        let Ok(number) = line.trim().parse::<i64>() else {
            panic!("Couldnt parse number: {}", line);
        };
        current_numbers_vec.push(number);
    }
    numbers.push(current_numbers_vec);

    let mut operations = Vec::new();
    for c in lines.last().unwrap().chars().into_iter() {
        if c != ' ' {
            operations.push(match c {
                '+' => Operation::Add,
                '*' => Operation::Multiply,
                _ => panic!("Invalid operation: {}", c),
            })
        }
    }

    for (numbers, operation) in numbers.into_iter().zip(operations.into_iter().rev()) {
        problems.push(Problem { numbers, operation })
    }

    problems
}

pub fn part_2() -> i64 {
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
