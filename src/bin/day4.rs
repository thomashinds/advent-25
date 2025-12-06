use core::num;

use advent_25::input::get_input_lines;

fn main() {
    let lines = get_input_lines(4);
    let paper_grid: Vec<Vec<bool>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .filter_map(|c| match c {
                    '@' => Some(true),
                    '.' => Some(false),
                    _ => None,
                })
                .collect()
        })
        .collect();

    let mut num_accessible = 0;
    for row in 0..paper_grid.len() {
        for col in 0..paper_grid[0].len() {
            if paper_grid[row][col] && num_adjacent(&paper_grid, row as i32, col as i32) < 4 {
                num_accessible += 1;
            }
        }
    }

    println!("{num_accessible} total accessible papers");

    let mut paper_grid = paper_grid;
    let mut num_removed = 0;
    loop {
        let removed_so_far = num_removed;
        println!("Removed {removed_so_far} so far");
        for row in 0..paper_grid.len() {
            for col in 0..paper_grid[0].len() {
                if paper_grid[row][col] && num_adjacent(&paper_grid, row as i32, col as i32) < 4 {
                    num_removed += 1;
                    paper_grid[row][col] = false;
                }
            }
        }
        if num_removed == removed_so_far {
            break;
        }
    }
    println!("{num_removed} total removed papers");

}

fn num_adjacent(grid: &[Vec<bool>], row: i32, col: i32) -> i32 {
    let mut num = 0;
    for (row_step, col_step) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let new_row = row + row_step;
        let new_col = col + col_step;
        if new_row < 0 || new_col < 0 {
            continue;
        }
        if let Some(true) = grid
            .get(new_row as usize)
            .and_then(|r| r.get(new_col as usize))
        {
            num += 1;
        }
    }

    num
}
