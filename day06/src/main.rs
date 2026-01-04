use std::io::Read;
use std::io::stdin;

fn main() {
    let mut text = String::new();
    stdin().read_to_string(&mut text).unwrap();
    let gt = grand_total(&text);
    println!("Result = {gt}");
}

type Number = u64;

#[derive(Debug, PartialEq, Clone)]
enum Operator {
    Addition,
    Multiplication,
}

#[derive(Debug, PartialEq, Clone)]
enum Item {
    NumberItem(String),
    OperatorItem(Operator),
}

fn grand_total(lines: &str) -> Number {
    let vectors = to_vec_of_vecs(lines);
    vectors
        .iter()
        .fold(0, |acc, item| acc + solve_problem(item))
}

fn to_vec_of_vecs(lines: &str) -> Vec<Vec<Item>> {
    let vector_of_lines = lines.lines().collect::<Vec<&str>>();
    if vector_of_lines.is_empty() {
        return vec![vec![]];
    }
    let length_of_first_line = vector_of_lines[0].len();

    assert!(
        vector_of_lines
            .iter()
            .filter(|line| !line.is_empty())
            .all(|line| line.len() == length_of_first_line),
    );

    // we need to find the first column that it has all values a blank space
    let mut starting_column = 0;
    let mut result = vec![];
    let mut result_item = 0;
    while starting_column < length_of_first_line {
        let mut column: usize = starting_column;
        let mut column_found = false;
        while column < length_of_first_line {
            let mut all_spaces = true;
            let mut row = 0;
            while row < vector_of_lines.len() {
                if vector_of_lines[row].chars().nth(column).unwrap() != ' ' {
                    all_spaces = false;
                    break;
                }
                row += 1;
            }
            if row == vector_of_lines.len() && all_spaces {
                column_found = true;
                break;
            }
            column += 1;
        }

        if !column_found && starting_column < length_of_first_line {
            column_found = true;
            column = length_of_first_line;
        }

        if column_found {
            result.push(vec![]);
            for row in &vector_of_lines {
                let string = row[starting_column..column].to_string();
                let first_char = string.chars().next().unwrap();
                if first_char == '*' {
                    result[result_item].push(Item::OperatorItem(Operator::Multiplication))
                } else if first_char == '+' {
                    result[result_item].push(Item::OperatorItem(Operator::Addition))
                } else {
                    result[result_item].push(Item::NumberItem(string))
                }
            }
            result_item += 1;
            starting_column = column + 1;
        }
    }

    result
}

fn solve_problem(problem: &Vec<Item>) -> Number {
    let line_length = match &problem[0] {
        Item::NumberItem(line_string) => line_string.len(),
        _ => panic!("First line should be Item::NumberItem(String)"),
    };
    let operator = &problem[problem.len() - 1];
    let mut result: Number = match operator {
        Item::OperatorItem(Operator::Addition) => 0,
        Item::OperatorItem(Operator::Multiplication) => 1,
        _ => panic!("Operator should be Item::OperatorItem"),
    };
    for digit in 0..line_length {
        let mut digit_to_number_string = "".to_string();
        for row in 0..(problem.len() - 1) {
            match &problem[row] {
                Item::NumberItem(line_string) => {
                    let digit_str = line_string.chars().nth(digit).unwrap();
                    if digit_str != ' ' {
                        digit_to_number_string.push_str(&digit_str.to_string());
                    }
                }
                _ => panic!("problem[row] should be Item::NumberItem(String)"),
            }
        }
        match operator {
            Item::OperatorItem(Operator::Addition) => {
                result += digit_to_number_string.parse::<Number>().unwrap()
            }
            Item::OperatorItem(Operator::Multiplication) => {
                result *= digit_to_number_string.parse::<Number>().unwrap()
            }
            _ => panic!("Operator should be Item::OperatorItem"),
        }
    }
    result
}

// ----------- to_vec_of_vecs()

#[test]
fn test_to_vec_of_vecs_case_1() {
    let input = "123 328  51 64 \r\n 45 64  387 23 \r\n  6 98  215 314\r\n*   +   *   +  \r\n";
    let result = to_vec_of_vecs(input);
    assert_eq!(
        result,
        vec![
            vec![
                Item::NumberItem("123".to_string()),
                Item::NumberItem(" 45".to_string()),
                Item::NumberItem("  6".to_string()),
                Item::OperatorItem(Operator::Multiplication)
            ],
            vec![
                Item::NumberItem("328".to_string()),
                Item::NumberItem("64 ".to_string()),
                Item::NumberItem("98 ".to_string()),
                Item::OperatorItem(Operator::Addition)
            ],
            vec![
                Item::NumberItem(" 51".to_string()),
                Item::NumberItem("387".to_string()),
                Item::NumberItem("215".to_string()),
                Item::OperatorItem(Operator::Multiplication)
            ],
            vec![
                Item::NumberItem("64 ".to_string()),
                Item::NumberItem("23 ".to_string()),
                Item::NumberItem("314".to_string()),
                Item::OperatorItem(Operator::Addition)
            ]
        ]
    );
}

// -------------- grand_total ------------------

#[test]
fn test_grand_total_case_1() {
    let input = "123 328  51 64 \r\n 45 64  387 23 \r\n  6 98  215 314\r\n*   +   *   +  \r\n";
    let result = grand_total(input);
    assert_eq!(result, 3263827);
}

// ---- solve_problem

#[test]
fn test_solve_problem_case_1() {
    let input: Vec<Item> = vec![
        Item::NumberItem("123".to_string()),
        Item::NumberItem(" 45".to_string()),
        Item::NumberItem("  6".to_string()),
        Item::OperatorItem(Operator::Multiplication),
    ];
    let result = solve_problem(&input);
    assert_eq!(result, 8544);
}

#[test]
fn test_solve_problem_case_2() {
    let input: Vec<Item> = vec![
        Item::NumberItem("328".to_string()),
        Item::NumberItem("64 ".to_string()),
        Item::NumberItem("98 ".to_string()),
        Item::OperatorItem(Operator::Addition),
    ];
    let result = solve_problem(&input);
    assert_eq!(result, 625);
}

#[test]
fn test_solve_problem_case_3() {
    let input: Vec<Item> = vec![
        Item::NumberItem(" 51".to_string()),
        Item::NumberItem("387".to_string()),
        Item::NumberItem("215".to_string()),
        Item::OperatorItem(Operator::Multiplication),
    ];
    let result = solve_problem(&input);
    assert_eq!(result, 3253600);
}

#[test]
fn test_solve_problem_case_4() {
    let input: Vec<Item> = vec![
        Item::NumberItem("64 ".to_string()),
        Item::NumberItem("23 ".to_string()),
        Item::NumberItem("314".to_string()),
        Item::OperatorItem(Operator::Addition),
    ];
    let result = solve_problem(&input);
    assert_eq!(result, 1058);
}
