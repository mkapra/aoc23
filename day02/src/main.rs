use std::{fs::File, io::Read};

mod first_star;
use first_star::Cube;

fn main() {
    let mut fh = File::open("puzzle_input.txt").expect("Could not open puzzle input");
    let mut puzzle_input = String::new();
    fh.read_to_string(&mut puzzle_input)
        .expect("Could not read puzzle input");

    println!(
        "Part 1: Sum of Game IDs: {}",
        first_star::solve(
            &puzzle_input,
            &[Cube::Red(12), Cube::Green(13), Cube::Blue(14)]
        )
    )
}
