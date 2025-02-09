use bsq_rs::file_to_string;
use std::time::Instant;
use std::env;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let (mut grid, rows, cols) = file_to_string(filename).unwrap();

    let duration = start.elapsed();
    eprintln!("Duration: {:?}", duration);

    //let Some(square) = get_biggest_square(&grid, rows, cols) else {
    //    eprintln!("Error: No square found");
    //    exit(1);
    //};
    //
    //replace_and_display_square(&mut grid, rows, cols, &square);
    //
}
