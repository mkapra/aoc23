use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

pub struct Game {
    id: u32,
    pulls: Vec<Vec<Cube>>,
}

impl Game {
    fn new(id: u32, pulls: Vec<Vec<Cube>>) -> Self {
        Game { id, pulls }
    }

    fn is_possible(&self, bag_content: &[Cube]) -> bool {
        let bag_content = Cube::to_hashmap(bag_content);

        let impossible_pulls = self
            .pulls
            .iter()
            .filter(|pull| {
                !pull
                    .iter()
                    .filter(|cube| !cube.keep(&bag_content))
                    .collect::<Vec<&Cube>>()
                    .is_empty()
            })
            .collect::<Vec<&Vec<Cube>>>();

        if impossible_pulls.is_empty() {
            return true;
        }
        return false;
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cube {
    Red(u32),
    Blue(u32),
    Green(u32),
}

impl Cube {
    fn get_color(&self) -> &str {
        match self {
            Self::Red(_) => "red",
            Self::Blue(_) => "blue",
            Self::Green(_) => "green",
        }
    }

    fn get_amount(&self) -> u32 {
        match self {
            Self::Red(amount) | Self::Blue(amount) | Self::Green(amount) => *amount,
        }
    }

    fn to_hashmap(cubes: &[Cube]) -> HashMap<&str, u32> {
        let mut hash = HashMap::new();
        cubes.iter().for_each(|c| {
            hash.insert(c.get_color(), c.get_amount());
        });
        hash
    }

    fn keep(&self, cubes: &HashMap<&str, u32>) -> bool {
        let h_cube = cubes
            .get(self.get_color())
            .expect("Cube must be in hashmap");
        self.get_amount() <= *h_cube
    }
}

impl TryFrom<(&str, &str)> for Cube {
    type Error = ();
    fn try_from((amount, value): (&str, &str)) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(Self::Red(amount.parse().unwrap())),
            "blue" => Ok(Self::Blue(amount.parse().unwrap())),
            "green" => Ok(Self::Green(amount.parse().unwrap())),
            _ => Err(()),
        }
    }
}

fn cube(input: &str) -> IResult<&str, Cube> {
    map_res(
        separated_pair(
            digit1,
            multispace1,
            alt((tag("blue"), tag("red"), tag("green"))),
        ),
        Cube::try_from,
    )(input)
}

fn pulls(input: &str) -> nom::IResult<&str, Vec<Cube>> {
    separated_list1(tag(", "), cube)(input)
}

fn process_line<'a>(line: &'a str, cubes: &[Cube]) -> IResult<&'a str, Option<u32>> {
    let (line, game_id) = preceded(tag("Game "), digit1)(line)?;
    let (line, pulls) = preceded(tag(": "), separated_list1(tag("; "), pulls))(line)?;
    assert!(line.is_empty());

    let game = Game::new(game_id.parse().expect("Should contain game id"), pulls);
    match game.is_possible(cubes) {
        true => Ok(("", Some(game.id))),
        false => Ok(("", None)),
    }
}

pub fn solve(input: &str, cubes: &[Cube]) -> u32 {
    input
        .lines()
        .filter_map(|line| process_line(line, cubes).expect("Error during parsing").1)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(
            solve(input, &[Cube::Red(12), Cube::Green(13), Cube::Blue(14)]),
            8
        )
    }

    fn prepare_hashmap() -> HashMap<&'static str, u32> {
        let mut map = HashMap::new();
        map.insert("red", 10);
        map.insert("blue", 5);
        map.insert("green", 7);
        map
    }

    #[test]
    fn test_keep_cube() {
        let cube_lt = Cube::Red(8);
        assert!(cube_lt.keep(&prepare_hashmap()));
        let cube_eq = Cube::Red(10);
        assert!(cube_eq.keep(&prepare_hashmap()));

        let cube_gt = Cube::Red(12);
        assert_eq!(cube_gt.keep(&prepare_hashmap()), false);
    }
}
