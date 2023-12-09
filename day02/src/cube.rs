use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

/// Parses a line of cube pulls
pub fn parse(input: &str) -> IResult<&str, Vec<Cube>> {
    separated_list1(
        tag(", "),
        map_res(
            separated_pair(
                digit1,
                multispace1,
                alt((tag("blue"), tag("red"), tag("green"))),
            ),
            Cube::try_from,
        ),
    )(input)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cube {
    Red(u32),
    Blue(u32),
    Green(u32),
}

impl Cube {
    pub fn get_color(&self) -> &str {
        match self {
            Self::Red(_) => "red",
            Self::Blue(_) => "blue",
            Self::Green(_) => "green",
        }
    }

    pub fn get_amount(&self) -> u32 {
        match self {
            Self::Red(amount) | Self::Blue(amount) | Self::Green(amount) => *amount,
        }
    }

    pub fn to_hashmap(cubes: &[Cube]) -> HashMap<&str, u32> {
        let mut hash = HashMap::new();
        cubes.iter().for_each(|c| {
            hash.insert(c.get_color(), c.get_amount());
        });
        hash
    }

    pub fn keep(&self, cubes: &HashMap<&str, u32>) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

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
