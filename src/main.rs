use bsq_rs::file_to_string;
use std::time::Instant;
use std::env;

pub const MAX_SIZE: usize = 100020000;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut buffer = [0u8; MAX_SIZE];
    let (rows, cols, bytes_to_skip) = file_to_string(&mut buffer, filename);

    let duration = start.elapsed();
    eprintln!("Duration: {:?} {:?} {:?} {:?}", duration, rows, cols, bytes_to_skip);

    //let Some(square) = get_biggest_square(&grid, rows, cols) else {
    //    eprintln!("Error: No square found");
    //    exit(1);
    //};
    //
    //replace_and_display_square(&mut grid, rows, cols, &square);
    //
}
