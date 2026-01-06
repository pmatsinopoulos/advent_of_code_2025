use std::io;

fn main() -> Result<(), std::io::Error> {
    let input = io::read_to_string(io::stdin())?;
    let mut vector = build_grid(&input);
    let splits = number_of_splits(&mut vector);

    println!("splits = {}", splits);

    Ok(())
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
