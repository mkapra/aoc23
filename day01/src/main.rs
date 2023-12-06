use std::{fs::File, io::Read};

mod first_star;
mod second_star;

fn main() {
    let mut fh = File::open("puzzle_input.txt").expect("Could not open puzzle input file");
    let mut puzzle_input = String::new();
    fh.read_to_string(&mut puzzle_input)
        .expect("Could not read puzzle input");

    let first_star = first_star::solve(&puzzle_input);
    println!("First star calibration value sum: {first_star}");
    let second_star = second_star::solve(&puzzle_input);
    println!("second star calibration value sum: {second_star}");
}
