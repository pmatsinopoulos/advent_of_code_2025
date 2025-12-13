use clap::Parser;
use std::cmp::PartialEq;
use std::fs;
use std::io::{BufRead, BufReader};
use std::ops::Range;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "input.txt")]
    input_file: String,
}

// I need to create a function that
// takes as input:
// - String
// returns back whether it is invalid and its integer representation.

#[derive(PartialEq, Debug)]
enum ValidationResult {
    Invalid { number: u64 },
    Valid,
}

fn validate(input: &str) -> ValidationResult {
    if input.len() <= 1 {
        return ValidationResult::Valid;
    }

    let i = 0;
    let mut j = i + 1;
    loop {
        let left = &input[i..j];
        let right = &input[j..];
        if right.len() < left.len() {
            break;
        }
        if left == &right[0..left.len()] && left.len() == right.len() {
            return ValidationResult::Invalid {
                number: left.parse().unwrap(),
            };
        }
        j += 1;
    }
    ValidationResult::Valid
}

fn range_from_string(input: &str) -> Range<u64> {
    let (left, right) = input.split_once("-").unwrap();
    let start: u64 = left.parse().unwrap();
    let end: u64 = right.parse().unwrap();
    Range {
        start,
        end: end + 1,
    }
}

fn ranges_from_string(input: &str) -> Vec<Range<u64>> {
    let ranges_strings: Vec<&str> = input.split(',').collect();
    ranges_strings
        .iter()
        .map(|x| range_from_string(x))
        .collect()
}

fn add_invalid_ranges(ranges: &Vec<Range<u64>>) -> u64 {
    let mut result = 0;
    for range in ranges {
        let r_cloned = range.clone();
        for i in r_cloned {
            let i_str = format!("{}", i);
            let is_invalid = validate(&i_str);
            match is_invalid {
                ValidationResult::Invalid { number } => {
                    result += i;
                }
                ValidationResult::Valid => {}
            }
        }
    }
    result
}

fn read_line(path: &str) -> String {
    fs::read_to_string(path).unwrap().trim_end().to_string()
}

fn main() {
    let args = Args::parse();
    let input_file = args.input_file;
    let line = read_line(&input_file);
    if line == "" {
        return;
    }
    let ranges = ranges_from_string(&line);
    let result = add_invalid_ranges(&ranges);
    println!("result = {}", result);
}

// ----- validate() -----------------------------------------------------------------------

#[test]
fn test_case_1() {
    let input = "55";
    let validation_result = validate(input);
    assert_eq!(validation_result, ValidationResult::Invalid { number: 5 });
}

#[test]
fn test_case_2() {
    let input = "12";
    let validation_result = validate(input);
    assert_eq!(validation_result, ValidationResult::Valid);
}

#[test]
fn test_case_3() {
    let input = "1";
    let validation_result = validate(input);
    assert_eq!(validation_result, ValidationResult::Valid);
}

#[test]
fn test_case_4() {
    let input = "";
    let validation_result = validate(input);
    assert_eq!(validation_result, ValidationResult::Valid);
}

#[test]
fn test_case_5() {
    let input = "6464";
    let validation_result = validate(input);
    assert_eq!(validation_result, ValidationResult::Invalid { number: 64 });
}

#[test]
fn test_case_6() {
    let input = "123123";
    let validation_result = validate(input);
    assert_eq!(validation_result, ValidationResult::Invalid { number: 123 });
}

#[test]
fn test_case_7() {
    let input = "101";
    let validation_result = validate(input);
    assert_eq!(validation_result, ValidationResult::Valid);
}

#[test]
fn test_case_8() {
    let input = "16464";
    let validation_result = validate(input);
    assert_eq!(validation_result, ValidationResult::Valid);
}

#[test]
fn test_case_9() {
    let input = "64641";
    let validation_result = validate(input);
    assert_eq!(validation_result, ValidationResult::Valid);
}

#[test]
fn test_case_10() {
    let input = "64164";
    let validation_result = validate(input);
    assert_eq!(validation_result, ValidationResult::Valid);
}

// --- range_from_string -------------------------

#[test]
fn test_range_from_string_case_1() {
    let input = "11-25";
    let range_result = range_from_string(input);
    assert_eq!(range_result, Range { start: 11, end: 26 });
}

// --- ranges_from_string ---------------------

#[test]
fn test_ranges_from_string_case_1() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890";
    let ranges = ranges_from_string(input);
    assert_eq!(
        ranges,
        vec![
            Range { start: 11, end: 23 },
            Range {
                start: 95,
                end: 116
            },
            Range {
                start: 998,
                end: 1013
            },
            Range {
                start: 1188511880,
                end: 1188511891,
            }
        ]
    )
}

// --- add_invalid_ranges ---

#[test]
fn add_invalid_ranges_case_1() {
    let ranges = vec![Range { start: 11, end: 23 }];
    let result = add_invalid_ranges(&ranges);
    assert_eq!(result, 33);
}

#[test]
fn add_invalid_ranges_case_2() {
    let ranges = vec![Range {
        start: 95,
        end: 116,
    }];
    let result = add_invalid_ranges(&ranges);
    assert_eq!(result, 99);
}

#[test]
fn add_invalid_ranges_case_3() {
    let ranges = vec![Range {
        start: 998,
        end: 1013,
    }];
    let result = add_invalid_ranges(&ranges);
    assert_eq!(result, 1010);
}
