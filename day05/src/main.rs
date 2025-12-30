use std::cmp::Ordering;
use std::ops::RangeInclusive;

fn main() {
    println!("Hello, world!");
}

type RangeBoundary = u64;
type IngredientsRange = RangeInclusive<RangeBoundary>;

fn turn_into_range(line: &str) -> Option<IngredientsRange> {
    if line.is_empty() || line.len() < 3 {
        return None;
    }

    let boundaries: Vec<RangeBoundary> = line
        .split("-")
        .into_iter()
        .map(|i| i.parse::<RangeBoundary>().unwrap_or_default())
        .take(2)
        .collect();

    Some(boundaries[0]..=boundaries[1])
}

fn compare_ranges(
    range_lhs: &RangeInclusive<RangeBoundary>,
    range_rhs: &RangeInclusive<RangeBoundary>,
) -> Ordering {
    if range_lhs.start() < range_rhs.start() {
        Ordering::Less
    } else if range_lhs.start() == range_rhs.start() {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
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

// Compare Ranges

#[test]
fn test_compare_ranges_case_1() {
    let mut ranges = vec![3..=10, 2..=5];
    ranges.sort_by(|lhs, rhs| compare_ranges(lhs, rhs));
    assert_eq!(ranges, vec![2..=5, 3..=10]);
}

#[test]
fn test_compare_ranges_case_2() {
    let mut ranges = vec![2..=10, 3..=5];
    ranges.sort_by(|lhs, rhs| compare_ranges(lhs, rhs));
    assert_eq!(ranges, vec![2..=10, 3..=5]);
}

#[test]
fn test_compare_ranges_case_3() {
    let mut ranges = vec![3..=10, 3..=5];
    ranges.sort_by(|lhs, rhs| compare_ranges(lhs, rhs));
    assert_eq!(ranges, vec![3..=10, 3..=5]);
}
