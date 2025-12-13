use clap::Parser;
use std::cmp::PartialEq;
use std::error::Error;
use std::fs;
use std::io;
use std::ops::RangeInclusive;

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

    let split_index = input.len() / 2;
    let (left, right) = input.split_at(split_index);

    if right == left {
        return ValidationResult::Invalid {
            number: left.parse().unwrap(),
        };
    }

    ValidationResult::Valid
}

fn range_from_string(input: &str) -> RangeInclusive<u64> {
    let (left, right) = input.split_once("-").unwrap();
    let start: u64 = left.parse().unwrap();
    let end: u64 = right.parse().unwrap();
    start..=end
}

fn ranges_from_string(input: &str) -> Vec<RangeInclusive<u64>> {
    input.split(',').map(range_from_string).collect()
}

fn add_invalid_ranges(ranges: &[RangeInclusive<u64>]) -> u64 {
    ranges
        .iter()
        .flat_map(|range| range.clone())
        .map(|value| match validate(&value.to_string()) {
            ValidationResult::Invalid { .. } => value,
            ValidationResult::Valid => 0,
        })
        .sum()
}

fn read_line(path: &str) -> io::Result<String> {
    let content = fs::read_to_string(path)?;
    Ok(content.trim_end().to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let input_file = args.input_file;
    let line = read_line(&input_file)?;
    if line.is_empty() {
        return Ok(());
    }
    let ranges = ranges_from_string(&line);
    let result = add_invalid_ranges(&ranges);
    println!("result = {}", result);
    Ok(())
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
    assert_eq!(range_result, 11..=25);
}

// --- ranges_from_string ---------------------

#[test]
fn test_ranges_from_string_case_1() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890";
    let ranges = ranges_from_string(input);
    assert_eq!(
        ranges,
        vec![11..=22, 95..=115, 998..=1012, 1188511880..=1188511890,]
    )
}

// --- add_invalid_ranges ---

#[test]
fn add_invalid_ranges_case_1() {
    let ranges = vec![11..=22];
    let result = add_invalid_ranges(&ranges);
    assert_eq!(result, 33);
}

#[test]
fn add_invalid_ranges_case_2() {
    let ranges = vec![99..=115];
    let result = add_invalid_ranges(&ranges);
    assert_eq!(result, 99);
}

#[test]
fn add_invalid_ranges_case_3() {
    let ranges = vec![998..=1012];
    let result = add_invalid_ranges(&ranges);
    assert_eq!(result, 1010);
}
