use std::cmp::Ordering;
use std::cmp::{max, min};
use std::io::BufRead;
use std::ops::RangeInclusive;

type RangeBoundary = u64;
type IngredientsRange = RangeInclusive<RangeBoundary>;

// Will read lines from the standard input.
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut ranges: Vec<IngredientsRange> = vec![];

    for line in &mut lines {
        let line = line.ok().unwrap();
        if line.is_empty() {
            break;
        }

        if let Some(r) = turn_into_range(&line) {
            ranges.push(r);
        }
    }

    // sort
    // ranges.sort_by(compare_ranges);
    ranges.sort_by(compare_ranges);

    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);

    let result = lines
        .flatten() // converts Iterator<Result<String, _>> to Iterator<String>
        .filter(|line| {
            integer_position(
                &non_overlapping_ranges,
                line.parse::<RangeBoundary>().unwrap(),
            )
            .is_some()
        })
        .count();

    println!("result = {result}");
}

fn compare_ranges(range_lhs: &IngredientsRange, range_rhs: &IngredientsRange) -> Ordering {
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

fn turn_into_range(line: &str) -> Option<IngredientsRange> {
    if line.is_empty() || line.len() < 3 {
        return None;
    }

    let (start, end) = line.trim().split_once("-")?;
    let start = start.trim().parse::<RangeBoundary>().ok()?;
    let end = end.trim().parse::<RangeBoundary>().ok()?;

    Some(start..=end)
}

// I have the way to sort ranges. But these ranges might be overlapping.
// I can merge the overlapping and end having less ranges.
//
// Merging two ranges when they are overlapping

fn merge_overlapping_ranges(
    range1: &IngredientsRange,
    range2: &IngredientsRange,
) -> Result<IngredientsRange, String> {
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

fn remove_overlapping_ranges(ranges: &[IngredientsRange]) -> Vec<IngredientsRange> {
    let mut non_overlapping_ranges: Vec<IngredientsRange> = vec![];
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

// Given a sorted and non-overlapping sequence of ranges
// and an integer, I can find the position of the range
// the integer belongs to

// integer_position

fn integer_position(ranges: &[IngredientsRange], integer: RangeBoundary) -> Option<usize> {
    ranges
        .binary_search_by(|range| {
            if integer < *range.start() {
                Ordering::Greater
            } else if integer > *range.end() {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
        .ok()
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
    ranges.sort_by(compare_ranges);
    assert_eq!(ranges, vec![2..=5, 3..=10]);
}

#[test]
fn test_compare_ranges_case_2() {
    let mut ranges = vec![2..=10, 3..=5];
    ranges.sort_by(compare_ranges);
    assert_eq!(ranges, vec![2..=10, 3..=5]);
}

#[test]
fn test_compare_ranges_case_3() {
    let mut ranges = vec![3..=5, 3..=10];
    ranges.sort_by(compare_ranges);
    assert_eq!(ranges, vec![3..=5, 3..=10]);
}

#[test]
fn test_compare_ranges_case_4() {
    let mut ranges = vec![3..=10, 3..=5];
    ranges.sort_by(compare_ranges);
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
    let ranges: Vec<IngredientsRange> = vec![1..=5, 2..=6, 8..=20];
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);
    assert_eq!(non_overlapping_ranges, vec![1..=6, 8..=20]);
}

#[test]
fn test_remove_overlapping_ranges_case_2() {
    let ranges: Vec<IngredientsRange> = vec![1..=5, 8..=20];
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);
    assert_eq!(non_overlapping_ranges, vec![1..=5, 8..=20]);
}

#[test]
fn test_remove_overlapping_ranges_case_3() {
    let ranges: Vec<IngredientsRange> = vec![1..=5];
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);
    assert_eq!(non_overlapping_ranges, vec![1..=5]);
}

#[test]
fn test_remove_overlapping_ranges_case_4() {
    let ranges: Vec<IngredientsRange> = vec![];
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);
    assert_eq!(non_overlapping_ranges, vec![]);
}

#[test]
fn test_remove_overlapping_ranges_case_5() {
    let ranges: Vec<IngredientsRange> = vec![1..=5, 5..=8, 5..=9, 8..=10];
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);
    assert_eq!(non_overlapping_ranges, vec![1..=10]);
}

#[test]
fn test_remove_overlapping_ranges_case_6() {
    let ranges: Vec<IngredientsRange> = vec![1..=5, 5..=7, 5..=8, 9..=11];
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);
    assert_eq!(non_overlapping_ranges, vec![1..=11]);
}

// sort and remove

#[test]
fn test_sort_and_remove_overlapping_ranges_case_1() {
    let mut ranges: Vec<IngredientsRange> = vec![5..=7, 1..=5, 9..=11, 5..=8];
    ranges.sort_by(compare_ranges);
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);
    assert_eq!(non_overlapping_ranges, vec![1..=11]);
}

// integer position

#[test]
fn test_integer_position_case_1() {
    let mut ranges: Vec<IngredientsRange> = vec![5..=7, 1..=5, 9..=11, 5..=8];

    // need to sort
    ranges.sort_by(|lhs, rhs| compare_ranges(lhs, rhs));

    // remove overlapping
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges); // this is : [1..=11]

    let integer: RangeBoundary = 1;
    let position = integer_position(&non_overlapping_ranges, integer);

    assert_eq!(position, Some(0));

    let integer: RangeBoundary = 11;
    let position = integer_position(&non_overlapping_ranges, integer);

    assert_eq!(position, Some(0));

    let integer: RangeBoundary = 2;
    let position = integer_position(&non_overlapping_ranges, integer);

    assert_eq!(position, Some(0));
}

#[test]
fn test_integer_position_case_2() {
    let mut ranges: Vec<IngredientsRange> = vec![5..=7, 1..=5, 9..=11, 5..=8];

    // need to sort
    ranges.sort_by(compare_ranges);

    // remove overlapping
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges); // this is : [1..=11]

    let integer: RangeBoundary = 1;
    let position = integer_position(&non_overlapping_ranges, integer);

    assert_eq!(position, Some(0));

    let integer: RangeBoundary = 11;
    let position = integer_position(&non_overlapping_ranges, integer);

    assert_eq!(position, Some(0));

    let integer: RangeBoundary = 2;
    let position = integer_position(&non_overlapping_ranges, integer);

    assert_eq!(position, Some(0));
}

#[test]
fn test_integer_position_case_3() {
    let mut ranges: Vec<IngredientsRange> = vec![];

    // need to sort
    ranges.sort_by(compare_ranges);

    // remove overlapping
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);

    let integer: RangeBoundary = 1;
    let position = integer_position(&non_overlapping_ranges, integer);

    assert_eq!(position, None);
}

#[test]
fn test_integer_position_case_4() {
    let mut ranges: Vec<IngredientsRange> = vec![1..=5];

    // need to sort
    ranges.sort_by(compare_ranges);

    // remove overlapping
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);

    let integer: RangeBoundary = 3;
    let position = integer_position(&non_overlapping_ranges, integer);

    assert_eq!(position, Some(0));
}

#[test]
fn test_integer_position_case_5() {
    let mut ranges: Vec<IngredientsRange> = vec![8..=15, 1..=5];

    // need to sort
    ranges.sort_by(compare_ranges);

    // remove overlapping
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);

    let integer: RangeBoundary = 14;
    let position = integer_position(&non_overlapping_ranges, integer);

    assert_eq!(position, Some(1));
}

#[test]
fn test_integer_position_case_6() {
    let mut ranges: Vec<IngredientsRange> = vec![8..=15, 1..=5];

    // need to sort
    ranges.sort_by(compare_ranges);

    // remove overlapping
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);

    let integer: RangeBoundary = 20;
    let position = integer_position(&non_overlapping_ranges, integer);

    assert_eq!(position, None);
}

#[test]
fn test_integer_position_case_7() {
    let mut ranges: Vec<IngredientsRange> = vec![8..=15, 1..=5];

    // need to sort
    ranges.sort_by(compare_ranges);

    // remove overlapping
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);

    let integer: RangeBoundary = 0;
    let position = integer_position(&non_overlapping_ranges, integer);

    assert_eq!(position, None);
}

#[test]
fn test_integer_position_case_8() {
    let mut ranges: Vec<IngredientsRange> = vec![8..=15, 1..=5];

    // need to sort
    ranges.sort_by(compare_ranges);

    // remove overlapping
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);

    let integer: RangeBoundary = 6;
    let position = integer_position(&non_overlapping_ranges, integer);

    assert_eq!(position, None);
}

#[test]
fn test_integer_position_case_9() {
    let mut ranges: Vec<IngredientsRange> = vec![8..=15, 1..=5];

    // need to sort
    ranges.sort_by(compare_ranges);

    // remove overlapping
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);

    let integer: RangeBoundary = 5;
    let position = integer_position(&non_overlapping_ranges, integer);

    assert_eq!(position, Some(0));
}

#[test]
fn test_integer_position_case_10() {
    let mut ranges: Vec<IngredientsRange> = vec![8..=15, 1..=5];

    // need to sort
    ranges.sort_by(compare_ranges);

    // remove overlapping
    let non_overlapping_ranges = remove_overlapping_ranges(&ranges);

    let integer: RangeBoundary = 8;
    let position = integer_position(&non_overlapping_ranges, integer);

    assert_eq!(position, Some(1));
}
