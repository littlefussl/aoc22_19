use std::collections::HashMap;

use regex::Regex;

use crate::resource::{Inventory, Resource};

#[derive(Debug)]
struct Recipe {
    costs: Inventory,
    robot_type: Resource,
}

#[derive(Debug)]
pub struct Blueprint {
    pub id: usize,
    recipes: Vec<Recipe>,
}

impl Blueprint {
    pub fn possible(&self, resources: &Inventory) -> Vec<(Resource, Inventory)> {
        self.recipes
            .iter()
            .filter_map(|Recipe { costs, robot_type }| {
                let new_inventory: Vec<_> = resources
                    .iter()
                    .map(|(&res, &count)| {
                        let cost = *costs.get(&res).unwrap_or(&0);
                        if count >= cost {
                            Some((res, cost))
                        } else {
                            None
                        }
                    })
                    .collect();

                let affordable = new_inventory.iter().all(Option::is_some);
                affordable.then(|| {
                    (
                        *robot_type,
                        HashMap::from_iter(new_inventory.iter().map(|&x| x.unwrap())),
                    )
                })
            })
            .collect()
    }

    pub fn obsidian_ratio(&self) -> f64 {
        let obsidian_recipe = self
            .recipes
            .iter()
            .find(|Recipe { robot_type, .. }| *robot_type == Resource::Obsidian)
            .unwrap();
        obsidian_recipe.costs[&Resource::Clay] as f64 / obsidian_recipe.costs[&Resource::Ore] as f64
    }
}

impl From<&str> for Blueprint {
    fn from(input: &str) -> Self {
        // Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 10 clay. Each geode robot costs 2 ore and 7 obsidian.
        let re = Regex::new("Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.").expect("Invalid regex");
        let captures: Vec<usize> = re
            .captures(input)
            .expect("Failed to parse with regex")
            .iter()
            .skip(1) // skip capture group 0
            .map(|capture| {
                capture
                    .expect("Missing capture")
                    .as_str()
                    .parse::<usize>()
                    .unwrap()
            })
            .collect();

        Blueprint {
            id: captures[0],
            recipes: vec![
                Recipe {
                    costs: HashMap::from_iter(vec![(Resource::Ore, captures[1])]),
                    robot_type: Resource::Ore,
                },
                Recipe {
                    costs: HashMap::from_iter(vec![(Resource::Ore, captures[2])]),
                    robot_type: Resource::Clay,
                },
                Recipe {
                    costs: HashMap::from_iter(vec![
                        (Resource::Ore, captures[3]),
                        (Resource::Clay, captures[4]),
                    ]),
                    robot_type: Resource::Obsidian,
                },
                Recipe {
                    costs: HashMap::from_iter(vec![
                        (Resource::Ore, captures[5]),
                        (Resource::Obsidian, captures[6]),
                    ]),
                    robot_type: Resource::Geode,
                },
            ],
        }
    }
}
