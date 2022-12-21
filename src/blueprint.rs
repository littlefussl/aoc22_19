use std::collections::HashMap;

use regex::Regex;

use crate::resource::{Inventory, ResourceType};

#[derive(Debug)]
struct Recipe {
    costs: Inventory,
    robot_type: ResourceType,
}

#[derive(Debug)]
pub struct Blueprint {
    pub id: u8,
    recipes: Vec<Recipe>,
}

impl Blueprint {
    pub fn possible(&self, resources: &Inventory) -> Vec<(ResourceType, Inventory)> {
        self.recipes
            .iter()
            .filter_map(|Recipe { costs, robot_type }| {
                let new_inventory: Vec<_> = resources
                    .iter()
                    .zip(costs)
                    .map(|(&res, &cost)| if res >= cost { Some(cost) } else { None })
                    .collect();

                let affordable = new_inventory.iter().all(Option::is_some);

                if affordable {
                    let mut x = new_inventory.iter().map(|&x| x.unwrap());
                    let return_value: Inventory = [
                        x.next().unwrap(),
                        x.next().unwrap(),
                        x.next().unwrap(),
                        x.next().unwrap(),
                    ];
                    Some((*robot_type, return_value))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn obsidian_ratio(&self) -> f64 {
        let obsidian_recipe = self
            .recipes
            .iter()
            .find(|Recipe { robot_type, .. }| *robot_type == ResourceType::Obsidian)
            .unwrap();
        obsidian_recipe.costs[ResourceType::Clay as usize] as f64
            / obsidian_recipe.costs[ResourceType::Ore as usize] as f64
    }
}

impl From<&str> for Blueprint {
    fn from(input: &str) -> Self {
        // Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 10 clay. Each geode robot costs 2 ore and 7 obsidian.
        let re = Regex::new("Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.").expect("Invalid regex");
        let captures: Vec<u8> = re
            .captures(input)
            .expect("Failed to parse with regex")
            .iter()
            .skip(1) // skip capture group 0
            .map(|capture| {
                capture
                    .expect("Missing capture")
                    .as_str()
                    .parse::<u8>()
                    .unwrap()
            })
            .collect();

        Blueprint {
            id: captures[0],
            recipes: vec![
                Recipe {
                    costs: [captures[1], 0, 0, 0],
                    robot_type: ResourceType::Ore,
                },
                Recipe {
                    costs: [captures[2], 0, 0, 0],
                    robot_type: ResourceType::Clay,
                },
                Recipe {
                    costs: [captures[3], captures[4], 0, 0],
                    robot_type: ResourceType::Obsidian,
                },
                Recipe {
                    costs: [captures[5], 0, captures[6], 0],
                    robot_type: ResourceType::Geode,
                },
            ],
        }
    }
}
