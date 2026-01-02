use std::io::Read;
use std::io::stdin;

fn main() {
    let mut text = String::new();
    stdin().read_to_string(&mut text).unwrap();
    let gt = grand_total(&text);
    println!("Result = {gt}");
}

type Number = u64;

#[derive(Debug, PartialEq)]
enum Operator {
    Addition,
    Multiplication,
}

#[derive(Debug, PartialEq)]
enum Item {
    NumberItem(Number),
    OperatorItem(Operator),
}

fn grand_total(lines: &str) -> Number {
    let vectors = to_vec_of_vecs(lines);
    let mut column = 0;
    let mut row;
    let number_of_columns = vectors[0].len();
    let number_of_rows = vectors.len();
    let mut result = 0;
    while column < number_of_columns {
        let operator = &vectors[number_of_rows - 1][column];
        let mut column_result = match operator {
            Item::OperatorItem(Operator::Addition) => 0,
            Item::OperatorItem(Operator::Multiplication) => 1,
            _ => panic!("Wrong data. Operator can't be determined"),
        };
        row = 0;
        while row < number_of_rows - 1 {
            match operator {
                Item::OperatorItem(Operator::Addition) => match vectors[row][column] {
                    Item::NumberItem(number) => column_result += number,
                    _ => (),
                },
                Item::OperatorItem(Operator::Multiplication) => match vectors[row][column] {
                    Item::NumberItem(number) => column_result *= number,
                    _ => (),
                },
                _ => (),
            }
            row += 1;
        }
        result += column_result;
        column += 1;
    }
    result
}

fn to_vec_of_vecs(lines: &str) -> Vec<Vec<Item>> {
    lines.lines().map(to_vec).collect()
}

fn to_vec(line: &str) -> Vec<Item> {
    if line.is_empty() {
        return vec![];
    }

    line.split_whitespace()
        .map(|string| {
            let first = string.chars().next().unwrap();
            if first == '*' {
                Item::OperatorItem(Operator::Multiplication)
            } else if first == '+' {
                Item::OperatorItem(Operator::Addition)
            } else {
                Item::NumberItem(string.parse::<Number>().unwrap())
            }
        })
        .collect()
}

#[test]
fn test_to_vec_case_1() {
    let input = "123 328    51 64";
    let result = to_vec(input);
    assert_eq!(
        result,
        vec![
            Item::NumberItem(123),
            Item::NumberItem(328),
            Item::NumberItem(51),
            Item::NumberItem(64)
        ]
    );
}

#[test]
fn test_to_vec_case_2() {
    let input = "* +   *    +";
    let result = to_vec(input);
    assert_eq!(
        result,
        vec![
            Item::OperatorItem(Operator::Multiplication),
            Item::OperatorItem(Operator::Addition),
            Item::OperatorItem(Operator::Multiplication),
            Item::OperatorItem(Operator::Addition)
        ]
    );
}

// ----------- to_vec_of_vecs()

#[test]
fn test_to_vec_of_vecs_case_1() {
    let input = "123 328  51 64\r\n\
                       45 64  387 23\r\n\
                       6 98  215 314\r\n\
                       *   +   *   +\r\n\
                    ";
    let result = to_vec_of_vecs(input);
    assert_eq!(
        result,
        vec![
            vec![
                Item::NumberItem(123),
                Item::NumberItem(328),
                Item::NumberItem(51),
                Item::NumberItem(64)
            ],
            vec![
                Item::NumberItem(45),
                Item::NumberItem(64),
                Item::NumberItem(387),
                Item::NumberItem(23)
            ],
            vec![
                Item::NumberItem(6),
                Item::NumberItem(98),
                Item::NumberItem(215),
                Item::NumberItem(314)
            ],
            vec![
                Item::OperatorItem(Operator::Multiplication),
                Item::OperatorItem(Operator::Addition),
                Item::OperatorItem(Operator::Multiplication),
                Item::OperatorItem(Operator::Addition)
            ]
        ]
    );
}

// -------------- grand_total ------------------

#[test]
fn test_grand_total_case_1() {
    let input = "123 328  51 64\r\n\
                       45 64  387 23\r\n\
                       6 98  215 314\r\n\
                       *   +   *   +\r\n\
                    ";
    let result = grand_total(input);
    assert_eq!(result, 4277556);
}
