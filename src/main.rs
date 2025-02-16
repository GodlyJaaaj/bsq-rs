use bsq_rs::{file_to_string, get_biggest_square, replace_and_display_square, MAP_SIZE};
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut map = file_to_string(filename);
    let map_view = &mut map[6..];
    let Some(square) = get_biggest_square(&map_view, MAP_SIZE, MAP_SIZE) else {
        eprintln!("Error: No square found");
        exit(1);
    };

    replace_and_display_square(map_view, MAP_SIZE, &square);
}
