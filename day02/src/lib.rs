use nom::{
    bytes::complete::tag, character::complete::digit1, multi::separated_list1, sequence::preceded,
    IResult,
};

mod cube;
pub use cube::Cube;
mod game;
use game::Game;

fn parse_line(line: &str) -> IResult<&str, Game> {
    let (line, game_id) = preceded(tag("Game "), digit1)(line)?;
    let (line, pulls) = preceded(tag(": "), separated_list1(tag("; "), cube::parse))(line)?;
    assert!(line.is_empty());

    Ok((
        "",
        Game::new(game_id.parse().expect("Should contain game id"), pulls),
    ))
}

pub fn solve_part1(input: &str, cubes: &[Cube]) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let (_, game) = parse_line(line).expect("Could not parse input");
            match game.is_possible(cubes) {
                true => Some(game.id),
                false => None,
            }
        })
        .sum::<u32>()
}

pub fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, game) = parse_line(line).expect("Could not parse input");
            game.get_fewest_cubes_power()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_testinput() -> &'static str {
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(
            solve_part1(
                get_testinput(),
                &[Cube::Red(12), Cube::Green(13), Cube::Blue(14)]
            ),
            8
        )
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(get_testinput()), 2286)
    }
}
