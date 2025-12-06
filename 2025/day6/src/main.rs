use std::str::from_utf8;

use neerajsi::*;

enum Operator {
    Plus,
    Times,
}

fn part1(input: &str) {
    let line_count = input.lines().count();
    println!("Number of lines: {}", line_count);

    let mut lines = input.lines();

    let numbers = lines.by_ref()
        .take(line_count - 1)
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num_str| num_str.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>();

    let operators = lines.next().unwrap()
        .split_ascii_whitespace()
        .map(|op_str| {
            match op_str {
                "+" => Operator::Plus,
                "*" => Operator::Times,
                _ => panic!("Unknown operator"),
            }
        })
        .collect::<Vec<_>>();
    
    let columns = operators.len();

    assert!(numbers.iter().all(|row| row.len() == columns));

    let mut sum: i64 = 0;
    for col in 0..columns {
        let mut col_result = numbers[0][col];
        for row in 1..line_count - 1 {
            match operators[col] {
                Operator::Plus => col_result += numbers[row][col],
                Operator::Times => col_result *= numbers[row][col],
            }
        }

        sum = sum.checked_add(col_result).unwrap();
    }

    println!("Part1 sum: {}", sum);

}

fn part2(input: &str) {
    let lines = input
        .lines()
        .map(|l| l.as_bytes())
        .collect::<Vec<_>>();

    let line_count = lines.len();
    let max_cols = lines.iter().map(|l| l.len()).max().unwrap();

    let mut transposed = Vec::new();

    for col in 0..max_cols {
        let mut col_vec = Vec::new();
        for row in 0..line_count {
            if col < lines[row].len() {
                col_vec.push(lines[row][col]);
            }
        }
        transposed.push(col_vec);
    }


    let mut in_group = false;
    let mut total = 0;
    let mut group_operator = Operator::Plus;
    let mut group_total: Option<i64> = None;
    for t in &transposed {
        let mut num_str = from_utf8(t).unwrap();
        if !in_group {
            let (op_char, remaining) = t.split_last().unwrap();

            num_str = from_utf8(remaining).unwrap();;
            match *op_char {
                b'+' => group_operator = Operator::Plus,
                b'*' => group_operator = Operator::Times,
                _ => panic!("Unknown operator"),
            }
            in_group = true;
            
            group_total = None;
            println!("New group with operator: {:?}", *op_char as char);
        }

        num_str = num_str.trim_ascii();
        if num_str.is_empty() {
            total += group_total.unwrap_or(0);
            in_group = false;
            println!("Group total: {:?}\n", group_total);
            continue;
        }

        let num: i64 = num_str.parse().unwrap();
        if let Some(current_total) = group_total {
            match group_operator {
                Operator::Plus => group_total = Some(current_total + num),
                Operator::Times => group_total = Some(current_total * num),
            }
        } else {
            group_total = Some(num);
        }

        println!("\tNum: {}, Group total: {}", num, group_total.unwrap());

    }

    if let Some(group_total) = group_total {
        total += group_total;
        println!("Group total: {:?}\n", group_total);
    }

    println!("Part2 total: {}", total);

}

fn main() {
    let input = String::from_utf8(read_stdin_input()).unwrap();
    part1(&input);

    part2(&input);
}
