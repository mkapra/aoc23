use std::collections::HashMap;

use crate::Cube;

pub struct Game {
    pub id: u32,
    pulls: Vec<Vec<Cube>>,
}

impl Game {
    pub fn new(id: u32, pulls: Vec<Vec<Cube>>) -> Self {
        Game { id, pulls }
    }

    pub fn is_possible(&self, bag_content: &[Cube]) -> bool {
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
        false
    }

    pub fn get_fewest_cubes_power(&self) -> u32 {
        let mut map = HashMap::new();
        self.pulls.iter().for_each(|pull| {
            pull.iter().for_each(|cube| {
                match map.get(cube.get_color()) {
                    None => {
                        map.insert(cube.get_color(), cube.get_amount());
                    }
                    Some(v) => {
                        if v < &cube.get_amount() {
                            map.insert(cube.get_color(), cube.get_amount());
                        }
                    }
                };
            })
        });
        map.values().product::<u32>()
    }
}
