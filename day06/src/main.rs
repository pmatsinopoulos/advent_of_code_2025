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
    NumberItem(Number),
    OperatorItem(Operator),
}

fn transpose(original: &mut Vec<Vec<Item>>) -> Vec<Vec<Item>> {
    if original.is_empty() || original[0].is_empty() {
        return vec![];
    }
    let mut result: Vec<Vec<Item>> = vec![];
    let number_of_rows = original.len();
    let number_of_columns = original[0].len();
    for j in 0..number_of_columns {
        result.push(vec![]);
        for i in 0..number_of_rows {
            result[j].push(original[i][j].clone());
        }
    }
    original.clear();
    result
}

fn grand_total(lines: &str) -> Number {
    let mut vectors = to_vec_of_vecs(lines);
    let transposed = transpose(&mut vectors);
    let mut result: Number = 0;
    for row in &transposed {
        let (init, operator) = match row[row.len() - 1] {
            Item::OperatorItem(Operator::Addition) => (0, Operator::Addition),
            Item::OperatorItem(Operator::Multiplication) => (1, Operator::Multiplication),
            _ => panic!("Invalid operator"),
        };

        let row_result = row[0..row.len() - 1]
            .iter()
            .fold(init, |acc, item| match operator {
                Operator::Addition => match item {
                    Item::NumberItem(number) => acc + number,
                    _ => panic!("Wrong item type"),
                },
                Operator::Multiplication => match item {
                    Item::NumberItem(number) => acc * number,
                    _ => panic!("Wrong item type"),
                },
            });
        result += row_result;
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

// ----------------- transpose -----------

#[test]
fn test_transpose_case_1() {
    let mut input = vec![
        vec![
            Item::NumberItem(123),
            Item::NumberItem(328),
            Item::NumberItem(51),
            Item::NumberItem(64),
        ],
        vec![
            Item::NumberItem(45),
            Item::NumberItem(64),
            Item::NumberItem(387),
            Item::NumberItem(23),
        ],
        vec![
            Item::NumberItem(6),
            Item::NumberItem(98),
            Item::NumberItem(215),
            Item::NumberItem(314),
        ],
        vec![
            Item::OperatorItem(Operator::Multiplication),
            Item::OperatorItem(Operator::Addition),
            Item::OperatorItem(Operator::Multiplication),
            Item::OperatorItem(Operator::Addition),
        ],
    ];
    let result = transpose(&mut input);
    assert_eq!(
        result,
        vec![
            vec![
                Item::NumberItem(123),
                Item::NumberItem(45),
                Item::NumberItem(6),
                Item::OperatorItem(Operator::Multiplication)
            ],
            vec![
                Item::NumberItem(328),
                Item::NumberItem(64),
                Item::NumberItem(98),
                Item::OperatorItem(Operator::Addition)
            ],
            vec![
                Item::NumberItem(51),
                Item::NumberItem(387),
                Item::NumberItem(215),
                Item::OperatorItem(Operator::Multiplication)
            ],
            vec![
                Item::NumberItem(64),
                Item::NumberItem(23),
                Item::NumberItem(314),
                Item::OperatorItem(Operator::Addition)
            ]
        ]
    )
}
