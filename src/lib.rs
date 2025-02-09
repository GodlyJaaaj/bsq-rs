use std::io::Read;
use std::fs::File;
use memchr::memchr;

#[allow(unused)]

pub fn file_to_string(mut buffer: &mut [u8], filename: &str) -> (usize, usize, usize) {
    let size = File::open(filename).unwrap().read(&mut buffer).unwrap();
    
    let first_newline_index = memchr(b'\n', &buffer[..size]).unwrap();
    let nb_lines: usize = std::str::from_utf8(&buffer[..first_newline_index])
        .unwrap()
        .parse()
        .unwrap();
    
    let start_index = first_newline_index + 1;
    let cols = memchr(b'\n', &buffer[start_index..size])
        .map(|idx| idx)
        .unwrap();
    
    (nb_lines, cols, first_newline_index)
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
