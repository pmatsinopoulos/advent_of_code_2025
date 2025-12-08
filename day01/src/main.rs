use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

const MAXIMUM_DIAL_POSITION: u8 = 99;

/// The dial direction is either Left or Right with a number of steps to turn.
enum DialDirection {
    Left(u32),
    Right(u32),
}

fn new_dial_position(current_dial_position: u8, dial_direction: DialDirection) -> u8 {
    match dial_direction {
        DialDirection::Left(steps) => dial_left(current_dial_position, steps),
        DialDirection::Right(steps) => dial_right(current_dial_position, steps),
    }
}

fn dial_right(current_dial_position: u8, steps: u32) -> u8 {
    assert!(current_dial_position <= MAXIMUM_DIAL_POSITION);

    let result: u32 = current_dial_position as u32 + steps;

    (result.rem_euclid(MAXIMUM_DIAL_POSITION as u32 + 1)) as u8
}

/// Calculates the new dial position when dialling left
fn dial_left(current_dial_position: u8, steps: u32) -> u8 {
    assert!(current_dial_position <= MAXIMUM_DIAL_POSITION);

    let result: i32 = current_dial_position as i32 - steps as i32;

    (result.rem_euclid(MAXIMUM_DIAL_POSITION as i32 + 1)) as u8
}

fn read_lines(path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "input.txt")]
    input_file: String,
}

fn main() {
    let args = Args::parse();
    let lines = read_lines(&args.input_file).unwrap();
    let mut current_dial_position = 50;
    let mut number_of_zeros = 0;
    for line in lines {
        if line == "" {
            continue;
        }
        let direction_char: &str = &line[0..1];
        let steps_string: &str = &line[1..line.len()];
        let steps: u32 = steps_string.parse::<u32>().unwrap();
        println!("direction_char {} steps {}", direction_char, steps);
        let dial_direction = match direction_char {
            "R" => DialDirection::Right(steps),
            "L" => DialDirection::Left(steps),
            _ => panic!("Invalid dial direction {}", direction_char),
        };
        current_dial_position = new_dial_position(current_dial_position, dial_direction);
        println!("New dial position = {}", current_dial_position);
        if current_dial_position == 0 {
            number_of_zeros += 1;
        }
    }
    println!("Number of Zeros {}", number_of_zeros);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dial_left_case_dial_before_zero_case_1() {
        let current_dial_position = 50;
        let steps = 1;
        let new_dial_position = dial_left(current_dial_position, steps);
        assert_eq!(new_dial_position, 49);
    }

    #[test]
    fn test_dial_left_case_dial_past_zero_case_1() {
        let current_dial_position = 50;
        let steps = 51;
        let new_dial_position = dial_left(current_dial_position, steps);
        assert_eq!(new_dial_position, 99);
    }

    #[test]
    fn test_dial_left_case_dial_at_zero_case_1() {
        let current_dial_position = 50;
        let steps = 50;
        let new_dial_position = dial_left(current_dial_position, steps);
        assert_eq!(new_dial_position, 0);
    }

    #[test]
    fn test_dial_left_case_dial_past_zero_case_2() {
        let current_dial_position = 50;
        let steps = 52;
        let new_dial_position = dial_left(current_dial_position, steps);
        assert_eq!(new_dial_position, 98);
    }

    #[test]
    fn test_dial_left_case_dial_past_zero_full_cycle_case_1() {
        let current_dial_position = 50;
        let steps = 50 + 50;
        let new_dial_position = dial_left(current_dial_position, steps);
        assert_eq!(new_dial_position, 50);
    }

    #[test]
    fn test_dial_left_case_dial_past_zero_full_cycle_case_2() {
        let current_dial_position = 50;
        let steps = 50 + 50 + 2;
        let new_dial_position = dial_left(current_dial_position, steps);
        assert_eq!(new_dial_position, 48);
    }

    #[test]
    fn test_dial_left_case_dial_past_zero_full_cycle_case_3() {
        let current_dial_position = 50;
        let steps = 10 * 50 + 2;
        let new_dial_position = dial_left(current_dial_position, steps);
        assert_eq!(new_dial_position, 48);
    }

    #[test]
    fn test_dial_left_case_dial_past_zero_full_cycle_case_4() {
        let current_dial_position = 50;
        let steps: u32 = 6 * (MAXIMUM_DIAL_POSITION as u32 + 1) + 2;
        let new_dial_position = dial_left(current_dial_position, steps);
        assert_eq!(new_dial_position, 48);
    }

    #[test]
    fn test_dial_left_case_dial_past_zero_full_cycle_case_5() {
        let current_dial_position = 50;
        let steps: u32 = 6 * (MAXIMUM_DIAL_POSITION as u32 + 1) - 2;
        let new_dial_position = dial_left(current_dial_position, steps);
        assert_eq!(new_dial_position, 52);
    }

    #[test]
    fn test_dial_left_case_6() {
        let current_dial_position = 50;
        let steps = 68;
        let new_dial_position = dial_left(current_dial_position, steps);
        assert_eq!(new_dial_position, 82);
    }

    #[test]
    fn test_dial_left_case_7() {
        let current_dial_position = 82;
        let steps = 30;
        let new_dial_position = dial_left(current_dial_position, steps);
        assert_eq!(new_dial_position, 52);
    }

    #[test]
    fn test_dial_right_case_8() {
        let current_dial_position = 52;
        let steps = 48;
        let new_dial_position = dial_right(current_dial_position, steps);
        assert_eq!(new_dial_position, 0);
    }

    #[test]
    fn test_dial_right_case_1() {
        let current_dial_position = 50;
        let steps = 1;
        let new_dial_position = dial_right(current_dial_position, steps);
        assert_eq!(new_dial_position, 51);
    }

    #[test]
    fn test_dial_right_case_2() {
        let current_dial_position = 50;
        let steps = 49;
        let new_dial_position = dial_right(current_dial_position, steps);
        assert_eq!(new_dial_position, 99);
    }

    #[test]
    fn test_dial_right_case_3() {
        let current_dial_position = 50;
        let steps = 50;
        let new_dial_position = dial_right(current_dial_position, steps);
        assert_eq!(new_dial_position, 0);
    }

    #[test]
    fn test_dial_right_case_4() {
        let current_dial_position = 50;
        let steps = 51;
        let new_dial_position = dial_right(current_dial_position, steps);
        assert_eq!(new_dial_position, 1);
    }

    #[test]
    fn test_dial_right_case_5() {
        let current_dial_position = 50;
        let steps = 151;
        let new_dial_position = dial_right(current_dial_position, steps);
        assert_eq!(new_dial_position, 1);
    }

    #[test]
    fn test_dial_right_case_6() {
        let current_dial_position = 50;
        let steps = 251;
        let new_dial_position = dial_right(current_dial_position, steps);
        assert_eq!(new_dial_position, 1);
    }

    #[test]
    fn test_dial_right_case_7() {
        let current_dial_position = 50;
        let steps = 258;
        let new_dial_position = dial_right(current_dial_position, steps);
        assert_eq!(new_dial_position, 8);
    }
}
