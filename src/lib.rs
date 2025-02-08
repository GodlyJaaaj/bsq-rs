use std::fs;
use std::io::Error;

#[derive(Debug)]
pub enum ParsingError {
    InvalidNb,
    NotEnoughLines,
    LineSizeNotMatching,
    CouldNotParse(Error),
}

pub fn file_to_string(filename: &str) -> Result<(String, usize, usize), ParsingError> {
    let content = fs::read_to_string(filename).map_err(ParsingError::CouldNotParse)?;
    let mut lines = content.lines();

    let nb_lines = lines.next().ok_or(ParsingError::NotEnoughLines)?;
    let nb_lines: u64 = nb_lines.parse().map_err(|_| ParsingError::InvalidNb)?;

    let grid_lines: Vec<&str> = lines.collect();

    if grid_lines.len() as u64 != nb_lines {
        return Err(ParsingError::NotEnoughLines);
    }

    let cols = grid_lines
        .first()
        .map(|s| s.len() as u64)
        .ok_or(ParsingError::NotEnoughLines)?;

    for line in &grid_lines {
        if line.len() as u64 != cols {
            return Err(ParsingError::LineSizeNotMatching);
        }
    }

    let mut grid = String::with_capacity((nb_lines * cols) as usize);
    for line in grid_lines {
        grid.push_str(line);
    }

    Ok((grid, nb_lines as usize, cols as usize))
}

#[derive(Debug)]
pub struct Square {
    pub size: usize,
    pub pos: (usize, usize),
}

pub fn get_biggest_square(grid: &str, rows: usize, cols: usize) -> Option<Square> {
    let mut dp = vec![0u8; rows * cols];
    let mut max_size = 0;
    let mut max_pos = (0, 0);
    let bytes = grid.as_bytes();

    for i in 0..rows {
        for j in 0..cols {
            let idx = i * cols + j;
            if bytes[idx] == b'.' {
                if i == 0 || j == 0 {
                    dp[idx] = 1;
                } else {
                    let left = dp[idx - 1];
                    let top = dp[idx - cols];
                    let diagonal = dp[idx - cols - 1];
                    dp[idx] = left.min(top).min(diagonal) + 1;
                }

                if dp[idx] > max_size {
                    max_size = dp[idx];
                    max_pos = (i, j);
                }
            }
        }
    }

    (max_size > 0).then(|| Square {
        size: max_size as usize,
        pos: match max_pos {
            (pos_x, pos_y) => (
                pos_x - (max_size as usize - 1),
                pos_y - (max_size as usize - 1),
            ),
        },
    })
}

pub fn replace_and_display_square(grid: &mut String, rows: usize, cols: usize, square: &Square) {
    let (start_row, start_col) = square.pos;
    let size = square.size;

    for i in start_row..start_row + size {
        let start_idx = i * cols + start_col;
        let end_idx = start_idx + size;

        grid.replace_range(start_idx..end_idx, &"x".repeat(size));
    }

    for i in 0..rows {
        let start_idx = i * cols;
        let end_idx = start_idx + cols;
        println!("{}", &grid[start_idx..end_idx]);
    }
}
