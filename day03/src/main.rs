use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::{BufRead, Result};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "input.txt")]
    input_file: String,
}

fn find_maximum_digit_and_position(input: &str) -> Option<(u8, usize)> {
    if input.len() == 0 {
        return None;
    }
    let mut max: u8 = (&input[0..1]).parse().unwrap();
    let mut position = 0_usize;

    if input.len() == 1 {
        return Some((max, position));
    }

    let initial_position = position + 1;
    for (i, byte) in input[initial_position..input.len()].bytes().enumerate() {
        let num = byte - b'0';
        if num > max {
            max = num;
            position = initial_position + i;
        }
    }
    // at the end of the iteration we have the maximum number and its position
    Some((max, position))
}

fn turn_two_max_digits_into_integer(input: &str) -> u8 {
    assert!(input.len() >= 2);

    let (max1, position) = find_maximum_digit_and_position(input).unwrap();

    let found = find_maximum_digit_and_position(&input[position + 1..]);
    if found.is_none() {
        // we are missing the second maximum. This means that the first maximum
        // is at the end of the input string. Hence, we have to find the maximum
        // before that
        let (max2, _) = find_maximum_digit_and_position(&input[0..position]).unwrap();
        max2 * 10 + max1
    } else {
        max1 * 10 + found.unwrap().0
    }
}

fn read_lines(path: &str) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn main() {
    let args: Args = Args::parse();
    let lines = read_lines(&args.input_file).unwrap();
    let mut sum = 0_u64;
    for line in lines {
        if line == "" {
            continue;
        }

        sum = sum + turn_two_max_digits_into_integer(&line) as u64;
    }

    println!("sum = {}", sum);
}

#[test]
fn test_find_maximum_digit_and_position_case_1() {
    let input = "1";
    let result = find_maximum_digit_and_position(input);
    assert_eq!(result, Some((1, 0)));
}

#[test]
fn test_find_maximum_digit_and_position_case_2() {
    let input = "12";
    let result = find_maximum_digit_and_position(input);
    assert_eq!(result, Some((2, 1)));
}

#[test]
fn test_find_maximum_digit_and_position_case_3() {
    let input = "132";
    let result = find_maximum_digit_and_position(input);
    assert_eq!(result, Some((3, 1)));
}

#[test]
fn test_find_maximum_digit_and_position_case_4() {
    let input = "1324";
    let result = find_maximum_digit_and_position(input);
    assert_eq!(result, Some((4, 3)));
}

#[test]
fn test_turn_two_max_digits_into_integer_case_1() {
    let input = "18";
    let result = turn_two_max_digits_into_integer(input);
    assert_eq!(result, 18);
}

#[test]
fn test_turn_two_max_digits_into_integer_case_2() {
    let input = "832";
    let result = turn_two_max_digits_into_integer(input);
    assert_eq!(result, 83);
}

#[test]
fn test_turn_two_max_digits_into_integer_case_3() {
    let input = "987654321111111";
    let result = turn_two_max_digits_into_integer(input);
    assert_eq!(result, 98);
}

#[test]
fn test_turn_two_max_digits_into_integer_case_4() {
    let input = "811111111111119";
    let result = turn_two_max_digits_into_integer(input);
    assert_eq!(result, 89);
}

#[test]
fn test_turn_two_max_digits_into_integer_case_5() {
    let input = "234234234234278";
    let result = turn_two_max_digits_into_integer(input);
    assert_eq!(result, 78);
}

#[test]
fn test_turn_two_max_digits_into_integer_case_6() {
    let input = "11";
    let result = turn_two_max_digits_into_integer(input);
    assert_eq!(result, 11);
}

#[test]
fn test_turn_two_max_digits_into_integer_case_7() {
    let input = "12";
    let result = turn_two_max_digits_into_integer(input);
    assert_eq!(result, 12);
}

#[test]
fn test_turn_two_max_digits_into_integer_case_8() {
    let input = "21";
    let result = turn_two_max_digits_into_integer(input);
    assert_eq!(result, 21);
}

#[test]
fn test_turn_two_max_digits_into_integer_case_9() {
    let input = "2121212121";
    let result = turn_two_max_digits_into_integer(input);
    assert_eq!(result, 22);
}

#[test]
fn test_turn_two_max_digits_into_integer_case_10() {
    let input = "818181911112111";
    let result = turn_two_max_digits_into_integer(input);
    assert_eq!(result, 92);
}
