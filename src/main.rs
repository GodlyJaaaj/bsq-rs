use bsq_rs::{file_to_string, get_biggest_square, replace_and_display_square};
use std::env;
use std::process::exit;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("No file specified");

    let (mut grid, rows, cols) = file_to_string(filename).unwrap();

    let Some(square) = get_biggest_square(&grid, rows, cols) else {
        eprintln!("Error: No square found");
        exit(1);
    };

    replace_and_display_square(&mut grid, rows, cols, &square);

    let duration = start.elapsed();
    eprintln!("Duration: {:?}", duration);
}
