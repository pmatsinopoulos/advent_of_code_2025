use std::io;

fn main() -> Result<(), std::io::Error> {
    let input = io::read_to_string(io::stdin())?;
    let mut vector = build_grid(&input);
    let splits = number_of_timelines(&mut vector);

    println!("splits = {}", splits);

    Ok(())
}

struct Step {
    row: usize,
    column: usize,
}

fn number_of_timelines(grid: &Vec<Vec<char>>) -> usize {
    if grid.len() < 2 {
        return 0;
    }

    // We do have at least two lines in the grid.
    let s_position = grid[0].iter().position(|c| *c == 'S').unwrap();

    let mut current_step = Step {
        row: 1,
        column: s_position,
    };
    let mut result = 0;
    let mut to_visit: Vec<Step> = vec![];
    loop {
        while current_step.row < grid.len() {
            if grid[current_step.row][current_step.column] == '.' {
                // I do nothing, just go at the beginning of the loop to continue building the timeline
                ();
            } else if grid[current_step.row][current_step.column] == '^' {
                // we are at a split position. We can take two paths.
                // We take the left path and we push into the stack the right path.
                to_visit.push(Step {
                    row: current_step.row,
                    column: current_step.column + 1,
                });
                current_step.column -= 1;
            }
            // move one row down, but stay on same column
            current_step.row += 1;
        }

        // if we have reached the bottom of the grid, we have finished with one more timeline
        if current_step.row == grid.len() {
            result += 1;
        }
        if to_visit.is_empty() {
            break;
        } else {
            current_step = to_visit.pop().unwrap();
        }
    }

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
    let result = number_of_timelines(&vec);
    assert_eq!(result, 4);
}

#[test]
fn test_number_of_timelines_case_2() {
    let input = "\
......................................................................S......................................................................\n\
.............................................................................................................................................\n\
......................................................................^......................................................................\n";
    let vec: Vec<Vec<char>> = build_grid(input);
    let result = number_of_timelines(&vec);
    assert_eq!(result, 2);
}

#[test]
fn test_number_of_timelines_case_3() {
    let input = "\
......................................................................S......................................................................\n\
.............................................................................................................................................\n\
......................................................................^......................................................................\n\
.............................................................................................................................................\n\
.....................................................................^.^.....................................................................\n\
.............................................................................................................................................\n\
....................................................................^...^....................................................................\n\
.............................................................................................................................................\n\
...................................................................^.^...^...................................................................\n\
.............................................................................................................................................\n\
..................................................................^...^...^..................................................................\n\
.............................................................................................................................................\n\
";
    let vec: Vec<Vec<char>> = build_grid(input);
    let result = number_of_timelines(&vec);
    assert_eq!(result, 14);
}
