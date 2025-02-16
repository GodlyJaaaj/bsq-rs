use bsq_rs::{file_to_string, get_biggest_square, replace_and_display_square, MAP_SIZE};
use std::{env, io};
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut map = file_to_string(filename);
    let map_view = &mut map[6..];
    let Some(square) = get_biggest_square(&map_view, MAP_SIZE, MAP_SIZE) else {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(map_view).unwrap();
        return
    };

    replace_and_display_square(map_view, MAP_SIZE, &square);
}
