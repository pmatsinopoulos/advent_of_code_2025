use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::{BufRead, Result};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "input.txt")]
    input_file: String,
}

fn max_digits_into_integer(input: &str, num_of_digits: usize) -> u128 {
    assert!(num_of_digits >= 1);
    assert!(input.len() >= num_of_digits);

    let mut stack: Vec<u8> = Vec::with_capacity(input.len());

    for (idx, byte) in input.bytes().enumerate() {
        let num = byte - b'0';
        while stack.len() > 0
            && num > stack[stack.len() - 1]
            && stack.len() - 1 + (input.len() - idx) >= num_of_digits
        {
            stack.pop();
        }
        if stack.len() < num_of_digits {
            stack.push(num);
        }
    }

    let mut value: u128 = 0;
    for &digit in &stack {
        value = value * 10 + digit as u128;
    }

    value
}

fn read_lines(path: &str) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn main() -> Result<()> {
    let args: Args = Args::parse();
    let sum: u128 = read_lines(&args.input_file)?
        .into_iter()
        .filter(|l| !l.trim().is_empty())
        .map(|line| max_digits_into_integer(&line, 12) as u128)
        .sum();

    println!("sum = {}", sum);

    Ok(())
}

// --- Tests ---

// max_digits_into_integer()

#[test]
fn test_max_digits_into_integer_case_1() {
    let input = "818181911112111";
    let num_of_digits = 12;
    let result = max_digits_into_integer(input, num_of_digits);
    assert_eq!(result, 888911112111);
}

#[test]
fn test_max_digits_into_integer_case_2() {
    let input = "987654321111111";
    let num_of_digits = 12;
    let result = max_digits_into_integer(input, num_of_digits);
    assert_eq!(result, 987654321111);
}

#[test]
fn test_max_digits_into_integer_case_3() {
    let input = "811111111111119";
    let num_of_digits = 12;
    let result = max_digits_into_integer(input, num_of_digits);
    assert_eq!(result, 811111111119);
}

#[test]
fn test_max_digits_into_integer_case_4() {
    let input = "234234234234278";
    let num_of_digits = 12;
    let result = max_digits_into_integer(input, num_of_digits);
    assert_eq!(result, 434234234278);
}
