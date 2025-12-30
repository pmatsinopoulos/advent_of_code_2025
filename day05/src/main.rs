use std::cmp::Ordering;
use std::cmp::{max, min};
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
        if range_lhs.end() < range_rhs.end() {
            Ordering::Less
        } else if range_lhs.end() == range_rhs.end() {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    } else {
        Ordering::Greater
    }
}

// I have the way to sort ranges. But these ranges might be overlapping.
// I can merge the overlapping and end having less ranges.
//
// Merging two ranges when they are overlapping

fn merge_overlapping_ranges(
    range1: &RangeInclusive<RangeBoundary>,
    range2: &RangeInclusive<RangeBoundary>,
) -> Result<RangeInclusive<RangeBoundary>, String> {
    if *range1.end() < *range2.start() - 1 || *range2.end() < *range1.start() - 1 {
        return Err("Non overlapping ranges".to_string());
    }
    if *range1.end() == *range2.start() - 1 {
        return Ok(*range1.start()..=*range2.end());
    }
    if *range2.end() == *range1.start() - 1 {
        return Ok(*range2.start()..=*range1.end());
    }
    Ok(min(*range1.start(), *range2.start())..=max(*range1.end(), *range2.end()))
}

// Remove overlapping ranges by merging.
// Note that this assumes that ranges are sorted by their `.start()`.

fn remove_overlapping_ranges(
    ranges: &Vec<RangeInclusive<RangeBoundary>>,
) -> Vec<RangeInclusive<RangeBoundary>> {
    let mut non_overlapping_ranges: Vec<RangeInclusive<RangeBoundary>> = vec![];
    for (i, range) in ranges.iter().enumerate() {
        if i == 0 {
            non_overlapping_ranges.push(range.clone());
            continue;
        }
        let merge_result = merge_overlapping_ranges(
            &non_overlapping_ranges[non_overlapping_ranges.len() - 1],
            &range,
        );
        if merge_result.is_ok() {
            non_overlapping_ranges.pop();
            non_overlapping_ranges.push(merge_result.unwrap());
        } else {
            non_overlapping_ranges.push(range.clone());
        }
    }
    non_overlapping_ranges
}

// ------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------

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
    let mut ranges = vec![3..=5, 3..=10];
    ranges.sort_by(|lhs, rhs| compare_ranges(lhs, rhs));
    assert_eq!(ranges, vec![3..=5, 3..=10]);
}

#[test]
fn test_compare_ranges_case_4() {
    let mut ranges = vec![3..=10, 3..=5];
    ranges.sort_by(|lhs, rhs| compare_ranges(lhs, rhs));
    assert_eq!(ranges, vec![3..=5, 3..=10]);
}

// merge_overlapping_ranges

#[test]
fn test_merge_overlapping_ranges_case_1() {
    let range1 = 1..=5;
    let range2 = 2..=6;
    let non_overlapping_range = merge_overlapping_ranges(&range1, &range2);
    assert_eq!(non_overlapping_range, Ok(1..=6));
}

#[test]
fn test_merge_overlapping_ranges_case_2() {
    let range1 = 1..=5;
    let range2 = 6..=8;
    let non_overlapping_range = merge_overlapping_ranges(&range1, &range2);
    assert_eq!(non_overlapping_range, Ok(1..=8));
}

#[test]
fn test_merge_overlapping_ranges_case_3() {
    let range1 = 6..=8;
    let range2 = 1..=5;
    let non_overlapping_range = merge_overlapping_ranges(&range1, &range2);
    assert_eq!(non_overlapping_range, Ok(1..=8));
}

#[test]
fn test_merge_overlapping_ranges_case_4() {
    let range1 = 2..=6;
    let range2 = 1..=5;
    let non_overlapping_range = merge_overlapping_ranges(&range1, &range2);
    assert_eq!(non_overlapping_range, Ok(1..=6));
}

#[test]
fn test_merge_overlapping_ranges_case_5() {
    let range1 = 1..=5;
    let range2 = 5..=8;
    let non_overlapping_range = merge_overlapping_ranges(&range1, &range2);
    assert_eq!(non_overlapping_range, Ok(1..=8));
}

#[test]
fn test_merge_overlapping_ranges_case_6() {
    let range1 = 1..=5;
    let range2 = 6..=8;
    let non_overlapping_range = merge_overlapping_ranges(&range1, &range2);
    assert_eq!(non_overlapping_range, Ok(1..=8));
}

// remove_overlapping_ranges

#[test]
fn test_remove_overlapping_ranges_case_1() {
    let ranges: Vec<RangeInclusive<RangeBoundary>> = vec![1..=5, 2..=6, 8..=20];
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);
    assert_eq!(non_overlapping_ranges, vec![1..=6, 8..=20]);
}

#[test]
fn test_remove_overlapping_ranges_case_2() {
    let ranges: Vec<RangeInclusive<RangeBoundary>> = vec![1..=5, 8..=20];
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);
    assert_eq!(non_overlapping_ranges, vec![1..=5, 8..=20]);
}

#[test]
fn test_remove_overlapping_ranges_case_3() {
    let ranges: Vec<RangeInclusive<RangeBoundary>> = vec![1..=5];
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);
    assert_eq!(non_overlapping_ranges, vec![1..=5]);
}

#[test]
fn test_remove_overlapping_ranges_case_4() {
    let ranges: Vec<RangeInclusive<RangeBoundary>> = vec![];
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);
    assert_eq!(non_overlapping_ranges, vec![]);
}

#[test]
fn test_remove_overlapping_ranges_case_5() {
    let ranges: Vec<RangeInclusive<RangeBoundary>> = vec![1..=5, 5..=8, 5..=9, 8..=10];
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);
    assert_eq!(non_overlapping_ranges, vec![1..=10]);
}

#[test]
fn test_remove_overlapping_ranges_case_6() {
    let ranges: Vec<RangeInclusive<RangeBoundary>> = vec![1..=5, 5..=7, 5..=8, 9..=11];
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);
    assert_eq!(non_overlapping_ranges, vec![1..=11]);
}

// sort and remove

#[test]
fn test_sort_and_remove_overlapping_ranges_case_1() {
    let mut ranges: Vec<RangeInclusive<RangeBoundary>> = vec![5..=7, 1..=5, 9..=11, 5..=8];
    ranges.sort_by(|lhs, rhs| compare_ranges(lhs, rhs));
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);
    assert_eq!(non_overlapping_ranges, vec![1..=11]);
}
