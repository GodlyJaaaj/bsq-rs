use memmap::{MmapMut, MmapOptions};
use std::fs::File;

pub const MAP_SIZE: usize = 10_000;

#[allow(unused)]

pub fn file_to_string(filename: &str) -> MmapMut {
    let file = File::options()
        .read(true)
        .write(true)
        .open(filename)
        .unwrap();
    let mmap = unsafe { MmapOptions::new().map_mut(&file).unwrap() };

    mmap
}

#[derive(Debug)]
pub struct Square {
    pub size: usize,
    pub pos: (usize, usize),
}

impl Square {
    fn new() -> Self {
        Square {
            size: 0,
            pos: (0, 0),
        }
    }
}


pub fn get_biggest_square(grid: &[u8], rows: usize, cols: usize) -> Option<Square> {
    let mut best = Square::new();
    let mut prev_row = [0; MAP_SIZE];
    let mut curr_row = [0; MAP_SIZE];

    let row_length = cols + 1;
    let mut row_start = 0;

    for y in 0..rows {
        for x in 0..cols {
            let pos = row_start + x;

            if grid[pos] == b'o' {
                curr_row[x] = 0;
            } else {
                curr_row[x] = if y == 0 || x == 0 {
                    1
                } else {
                    1 + prev_row[x - 1].min(prev_row[x]).min(curr_row[x - 1])
                };

                if curr_row[x] > best.size {
                    best.size = curr_row[x];
                    best.pos = (y, x);
                }
            }
        }
        row_start += row_length;
        std::mem::swap(&mut prev_row, &mut curr_row);
    }

    (best.size > 0).then(|| {
        best.pos = match best.size {
            max => (
                best.pos.0 - (max - 1),
                best.pos.1 - (max - 1),
            ),
        };
        best
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
