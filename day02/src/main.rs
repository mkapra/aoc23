use std::{fs::File, io::Read};

use day02::{solve_part1, solve_part2, Cube};

fn main() {
    let mut fh = File::open("puzzle_input.txt").expect("Could not open puzzle input");
    let mut puzzle_input = String::new();
    fh.read_to_string(&mut puzzle_input)
        .expect("Could not read puzzle input");

    println!(
        "Part 1: Sum of Game IDs: {}",
        solve_part1(
            &puzzle_input,
            &[Cube::Red(12), Cube::Green(13), Cube::Blue(14)]
        )
    );
    println!(
        "Part 2: Sum of all game powers: {}",
        solve_part2(&puzzle_input)
    )
}
