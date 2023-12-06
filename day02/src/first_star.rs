use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::{
        complete::{char, digit1, multispace0, multispace1},
        is_digit,
    },
    combinator::{map_res, opt},
    multi::separated_list0,
    multi::{many0, separated_list1},
    sequence::{pair, separated_pair},
    IResult,
};

type GameId = u32;

#[derive(Debug)]
enum Cube {
    Red(u32),
    Blue(u32),
    Green(u32),
}

impl TryFrom<(&str, &str)> for Cube {
    type Error = ();
    fn try_from((value, amount): (&str, &str)) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(Self::Red(amount.parse().unwrap())),
            "blue" => Ok(Self::Blue(amount.parse().unwrap())),
            "green" => Ok(Self::Green(amount.parse().unwrap())),
            _ => Err(()),
        }
    }
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    println!("PARSE CUBE");
    dbg!(input);
    map_res(
        separated_pair(
            digit1,
            multispace1,
            alt((tag("blue"), tag("red"), tag("green"))),
        ),
        Cube::try_from,
    )(input)
}

fn parse_pulls(input: &str) -> nom::IResult<&str, Vec<Cube>> {
    println!("PARSE PULLS");
    dbg!(input);
    separated_list1(tag(", "), parse_cube)(input)
}

fn process_line(line: &str) -> IResult<&str, Option<u32>> {
    let (rest, _) = tag("Game")(line)?;
    let (rest, _) = multispace1(rest)?;
    let (rest, game_id) = digit1(rest)?;
    let (rest, _) = tag(":")(rest)?;
    let (rest, _) = multispace1(rest)?;

    let (rest, pulls) = many0(pair(parse_pulls, opt(tag("; "))))(rest)?;
    // let (rest, pulls) = separated_list1(tag("; "), parse_pulls)(rest)?;
    assert!(rest.len() == 0);
    dbg!(pulls);

    Ok((
        "",
        Some(
            game_id
                .parse::<GameId>()
                .expect("Could not convert game id"),
        ),
    ))
}

fn solve(input: &str, cubes: &[Cube]) -> u32 {
    input
        .lines()
        .filter_map(|line| process_line(line).expect("Error during parsing").1)
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
            solve(input, &vec![Cube::Red(12), Cube::Green(13), Cube::Blue(14)]),
            8
        )
    }
}
