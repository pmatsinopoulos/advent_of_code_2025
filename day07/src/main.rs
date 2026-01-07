use std::collections::HashMap;
use std::io;

fn main() -> Result<(), std::io::Error> {
    let input = io::read_to_string(io::stdin())?;
    let mut vector = build_grid(&input);
    let s_position = vector[0].iter().position(|c| *c == 'S').unwrap();
    let mut cache_calculations: HashMap<Step, usize> = HashMap::new();
    let splits = number_of_timelines(
        &mut vector,
        Step {
            row: 0,
            column: s_position,
        },
        &mut cache_calculations,
    );

    println!("splits = {}", splits);

    Ok(())
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Step {
    row: usize,
    column: usize,
}

fn number_of_timelines(
    grid: &Vec<Vec<char>>,
    starting_step: Step,
    cache_calculations: &mut HashMap<Step, usize>,
) -> usize {
    // Given the current_step, in order to calculate the timelines from this step downwards:
    // If the step below is a '.', then I need to calculate the timelines of the step below and then the current step will have
    // equal number of timelines.
    // If the step below is a '^', then I need to calculate the timelines of the step to the left and the timelines of the step to the
    // right and add them.
    // Everytime I calculate the timeslines for a step, I need to save it so that I don't have to calculate it again.

    let cached_value = cache_calculations.get(&starting_step);
    if cached_value.is_some() {
        return *cached_value.unwrap();
    }

    let mut result = 0;

    if starting_step.row == grid.len() - 1 {
        result = 1;
    } else if grid[starting_step.row + 1][starting_step.column] == '.' {
        result = number_of_timelines(
            grid,
            Step {
                row: starting_step.row + 1,
                column: starting_step.column,
            },
            cache_calculations,
        );
    } else if grid[starting_step.row + 1][starting_step.column] == '^' {
        result = number_of_timelines(
            grid,
            Step {
                row: starting_step.row + 1,
                column: starting_step.column - 1,
            },
            cache_calculations,
        ) + number_of_timelines(
            grid,
            Step {
                row: starting_step.row + 1,
                column: starting_step.column + 1,
            },
            cache_calculations,
        );
    }

    cache_calculations.insert(starting_step, result);

    result
}

fn number_of_splits(grid: &mut Vec<Vec<char>>) -> usize {
    if grid.len() < 2 {
        return 0;
    }

    // We do have at least two lines in the grid.

    let mut result: usize = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let ch = grid[i][j];
            match ch {
                'S' => grid[i][j] = '|',
                '^' if grid[i - 1][j] == '|' => {
                    result += 1;
                    if j > 0 {
                        grid[i][j - 1] = '|';
                    }
                    if j + 1 < grid[i].len() {
                        grid[i][j + 1] = '|';
                    }
                }
                '.' if i > 0 && grid[i - 1][j] == '|' => grid[i][j] = '|',
                _ => (),
            }
        }
    }

    result
}

fn build_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

#[test]
fn test_build_grid_case_1() {
    let input = "\
                      .......S..........\n\
                      .......^..........\n\
                      \n\
                      ...";
    let vec: Vec<Vec<char>> = build_grid(input);
    let expected_vec: Vec<Vec<char>> = vec![
        vec![
            '.', '.', '.', '.', '.', '.', '.', 'S', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.',
        ],
        vec![
            '.', '.', '.', '.', '.', '.', '.', '^', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.',
        ],
        vec!['.', '.', '.'],
    ];
    assert_eq!(vec, expected_vec);
}

#[test]
fn test_number_of_splits_case_1() {
    let input = "\
                      .......S..........\n\
                      .......^..........\n\
                      \n\
                      ...";
    let mut vec: Vec<Vec<char>> = build_grid(input);
    let result = number_of_splits(&mut vec);
    assert_eq!(result, 1);
}

#[test]
fn test_number_of_splits_case_2() {
    let input = "\
                      .......S.......\n\
                      ...............\n\
                      .......^.......\n\
                      \n\
                      ...";
    let mut vec: Vec<Vec<char>> = build_grid(input);
    let result = number_of_splits(&mut vec);
    assert_eq!(result, 1);
}

#[test]
fn test_number_of_splits_case_3() {
    let input = "\
                      .......S.......\n\
                      ...............\n\
                      .......^.......\n\
                      ...............\n\
                      ......^.^......\n\
                      \n\
                      ...";
    let mut vec: Vec<Vec<char>> = build_grid(input);
    let result = number_of_splits(&mut vec);
    assert_eq!(result, 3);
}

#[test]
fn test_number_of_timelines_case_1() {
    let input = "\
                      .......S.......\n\
                      ...............\n\
                      .......^.......\n\
                      ...............\n\
                      ......^.^......\n\
                      \n\
                      ...............";
    let vec: Vec<Vec<char>> = build_grid(input);
    let mut cache_calculations: HashMap<Step, usize> = HashMap::new();
    let result = number_of_timelines(
        &vec,
        Step {
            row: 1,
            column: vec[0].iter().position(|c| *c == 'S').unwrap(),
        },
        &mut cache_calculations,
    );
    assert_eq!(result, 4);
}

#[test]
fn test_number_of_timelines_case_2() {
    let input = "\
......................................................................S......................................................................\n\
.............................................................................................................................................\n\
......................................................................^......................................................................\n";
    let vec: Vec<Vec<char>> = build_grid(input);
    let mut cache_calculations: HashMap<Step, usize> = HashMap::new();
    let result = number_of_timelines(
        &vec,
        Step {
            row: 1,
            column: vec[0].iter().position(|c| *c == 'S').unwrap(),
        },
        &mut cache_calculations,
    );
    assert_eq!(result, 2);
}

#[test]
fn test_number_of_timelines_case_3() {
    let input = "\
.....S.....\n\
...........\n\
.....^.....\n\
...........\n\
....^.^....\n\
...........\n\
...^...^...\n\
...........\n\
..^.^...^..\n\
...........\n\
.^...^...^.\n\
...........\n\
";
    let vec: Vec<Vec<char>> = build_grid(input);
    let mut cache_calculations: HashMap<Step, usize> = HashMap::new();
    let result = number_of_timelines(
        &vec,
        Step {
            row: 1,
            column: vec[0].iter().position(|c| *c == 'S').unwrap(),
        },
        &mut cache_calculations,
    );
    assert_eq!(result, 14);
}

#[test]
fn test_number_of_timelines_case_4() {
    let input = "\
.....S.....\n\
...........\n\
.....^.....\n\
...........\n\
";
    let vec: Vec<Vec<char>> = build_grid(input);
    let mut cache_calculations: HashMap<Step, usize> = HashMap::new();
    let result = number_of_timelines(
        &vec,
        Step {
            row: 1,
            column: vec[0].iter().position(|c| *c == 'S').unwrap(),
        },
        &mut cache_calculations,
    );
    assert_eq!(result, 2);
}
