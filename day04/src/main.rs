use clap::Parser;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;

#[derive(Debug)]
struct GridPosition {
    column: usize,
    row: usize,
}

type Grid = Vec<String>;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "input.txt")]
    input_file: String,
}

fn read_lines(path: &str) -> Result<Grid> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut grid = read_lines(&args.input_file)?;

    let sum: usize = std::iter::from_fn(|| {
        let result = number_of_rolls_that_can_be_accessed_and_removed(&mut grid);
        (result > 0).then_some(result)
    })
    .sum();

    println!("result = {sum}");
    Ok(())
}

fn number_of_rolls_that_can_be_accessed(grid: &Grid) -> usize {
    let mut result = 0;

    let number_of_rows = grid.len();
    assert!(number_of_rows >= 1);

    let number_of_columns = grid[0].len();
    assert!(number_of_columns >= 1);

    for i in 0..number_of_rows {
        for j in 0..number_of_columns {
            let current_position = GridPosition { column: j, row: i };
            if can_be_accessed(&grid, &current_position) {
                result += 1;
            }
        }
    }
    result
}

fn number_of_rolls_that_can_be_accessed_and_removed(grid: &mut Vec<String>) -> usize {
    let mut result = 0;

    let number_of_rows = grid.len();
    assert!(number_of_rows >= 1);

    let number_of_columns = grid[0].len();
    assert!(number_of_columns >= 1);
    let mut positions_to_remove: Vec<GridPosition> = vec![];
    for i in 0..number_of_rows {
        for j in 0..number_of_columns {
            let current_position = GridPosition { column: j, row: i };
            if can_be_accessed(grid, &current_position) {
                positions_to_remove.push(current_position);
                result += 1;
            }
        }
    }
    for position_to_remove in positions_to_remove {
        grid[position_to_remove.row].replace_range(
            position_to_remove.column..position_to_remove.column + 1,
            ".",
        )
    }
    result
}

fn can_be_accessed(grid: &Grid, current_position: &GridPosition) -> bool {
    grid[current_position.row].as_bytes()[current_position.column] as char == '@'
        && number_of_adjacent_rolls_of_paper(grid, current_position) < 4
}

fn number_of_adjacent_rolls_of_paper(grid: &Grid, current_position: &GridPosition) -> usize {
    assert!(grid.len() > 0 && grid[0].len() > 0);
    assert!(current_position.column < grid[0].len() && current_position.row < grid.len());

    let mut i = if current_position.row > 0 {
        current_position.row - 1
    } else {
        0
    };

    let mut count = 0;
    while i <= current_position.row + 1 && i < grid.len() {
        let mut j = if current_position.column > 0 {
            current_position.column - 1
        } else {
            0
        };
        while j <= current_position.column + 1 && j < grid[i].len() {
            if i == current_position.row && j == current_position.column {
                j += 1;
                continue;
            }
            if grid[i].as_bytes()[j] as char == '@' {
                count += 1;
            }
            j += 1;
        }
        i += 1;
    }
    count
}

fn number_of_rolls_that_can_be_removed(grid: &Grid) -> usize {
    0
}

#[cfg(test)]
#[derive(Debug)]
struct TestGridPosition {
    position: GridPosition,
    result: usize,
    can_be_accessed: bool,
}

#[test]
fn test_number_of_adjacent_rolls_of_paper_case_1() {
    let grid: Grid = vec![
        "..@@.@@@@.".to_string(),
        "@@@.@.@.@@".to_string(),
        "@@@@@.@.@@".to_string(),
        "@.@@@@..@.".to_string(),
        "@@.@@@@.@@".to_string(),
        ".@@@@@@@.@".to_string(),
        ".@.@.@.@@@".to_string(),
        "@.@@@.@@@@".to_string(),
        ".@@@@@@@@.".to_string(),
        "@.@.@@@.@.".to_string(),
    ];
    let test_grid_positions = vec![
        // row 0
        TestGridPosition {
            position: GridPosition { column: 0, row: 0 },
            result: 2,
            can_be_accessed: false,
        },
        TestGridPosition {
            position: GridPosition { column: 1, row: 0 },
            result: 4,
            can_be_accessed: false,
        },
        TestGridPosition {
            position: GridPosition { column: 2, row: 0 },
            result: 3,
            can_be_accessed: true,
        },
        TestGridPosition {
            position: GridPosition { column: 3, row: 0 },
            result: 3,
            can_be_accessed: true,
        },
        TestGridPosition {
            position: GridPosition { column: 4, row: 0 },
            result: 3,
            can_be_accessed: false,
        },
        TestGridPosition {
            position: GridPosition { column: 5, row: 0 },
            result: 3,
            can_be_accessed: true,
        },
        TestGridPosition {
            position: GridPosition { column: 6, row: 0 },
            result: 3,
            can_be_accessed: true,
        },
        TestGridPosition {
            position: GridPosition { column: 7, row: 0 },
            result: 4,
            can_be_accessed: false,
        },
        TestGridPosition {
            position: GridPosition { column: 8, row: 0 },
            result: 3,
            can_be_accessed: true,
        },
        TestGridPosition {
            position: GridPosition { column: 9, row: 0 },
            result: 3,
            can_be_accessed: false,
        },
        // row 1
        TestGridPosition {
            position: GridPosition { column: 0, row: 1 },
            result: 3,
            can_be_accessed: true,
        },
        TestGridPosition {
            position: GridPosition { column: 1, row: 1 },
            result: 6,
            can_be_accessed: false,
        },
        TestGridPosition {
            position: GridPosition { column: 2, row: 1 },
            result: 6,
            can_be_accessed: false,
        },
        // last row last column
        TestGridPosition {
            position: GridPosition {
                column: grid[0].len() - 1,
                row: grid.len() - 1,
            },
            result: 2,
            can_be_accessed: false, // because it is not a roll of paper
        },
    ];

    for test_grid_position in test_grid_positions {
        let number = number_of_adjacent_rolls_of_paper(&grid, &test_grid_position.position);
        assert_eq!(
            number, test_grid_position.result,
            "test_grid_position: {:?}",
            test_grid_position
        );

        let can_be_accessed_result = can_be_accessed(&grid, &test_grid_position.position);
        assert_eq!(
            can_be_accessed_result, test_grid_position.can_be_accessed,
            "test_grid_position: {:?}",
            test_grid_position
        );
    }

    let number_of_rolls = number_of_rolls_that_can_be_accessed(&grid);
    assert_eq!(number_of_rolls, 13);
}

#[test]
fn test_case_2() {
    let mut grid: Grid = vec![
        "..@@.@@@@.".to_string(),
        "@@@.@.@.@@".to_string(),
        "@@@@@.@.@@".to_string(),
        "@.@@@@..@.".to_string(),
        "@@.@@@@.@@".to_string(),
        ".@@@@@@@.@".to_string(),
        ".@.@.@.@@@".to_string(),
        "@.@@@.@@@@".to_string(),
        ".@@@@@@@@.".to_string(),
        "@.@.@@@.@.".to_string(),
    ];

    let result = number_of_rolls_that_can_be_accessed_and_removed(&mut grid);
    assert_eq!(result, 13);
    assert_eq!(
        grid,
        vec![
            ".......@..".to_string(),
            ".@@.@.@.@@".to_string(),
            "@@@@@...@@".to_string(),
            "@.@@@@..@.".to_string(),
            ".@.@@@@.@.".to_string(),
            ".@@@@@@@.@".to_string(),
            ".@.@.@.@@@".to_string(),
            "..@@@.@@@@".to_string(),
            ".@@@@@@@@.".to_string(),
            "....@@@...".to_string(),
        ]
    );
}
