use std::ops::RangeInclusive;

fn main() {
    println!("Hello, world!");
}

fn turn_into_range(line: &str) -> Option<RangeInclusive<u32>> {
    if line.is_empty() || line.len() < 3 {
        return None;
    }

    let boundaries: Vec<u32> = line
        .split("-")
        .into_iter()
        .map(|i| i.parse::<u32>().unwrap_or_default())
        .take(2)
        .collect();

    Some(boundaries[0]..=boundaries[1])
}

#[test]
fn test_turn_into_range_case_1() {
    let line = "3-5";
    let result = turn_into_range(line);
    assert_eq!(result, Some(3..=5));
}

#[test]
fn test_turn_into_range_case_2() {
    let line = "12-18";
    let result = turn_into_range(line);
    assert_eq!(result, Some(12..=18));
}

#[test]
fn test_turn_into_range_case_3() {
    let line = "";
    let result = turn_into_range(line);
    assert_eq!(result, None);
}

#[test]
fn test_turn_into_range_case_4() {
    let line = " ";
    let result = turn_into_range(line);
    assert_eq!(result, None);
}

#[test]
fn test_turn_into_range_case_5() {
    let line = "1-";
    let result = turn_into_range(line);
    assert_eq!(result, None);
}
