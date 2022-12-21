use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::Hash,
    sync::{atomic::AtomicUsize, Mutex},
};

use rand::thread_rng;
use rand::seq::SliceRandom;

use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

const MAX_STEPS: usize = 23;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug, PartialOrd, Ord)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

type Inventory = HashMap<Resource, usize>;

#[derive(Debug, Clone)]
struct State {
    robots: Inventory,
    resources: Inventory,
    step: usize,
    // resources: [usize; Roboto::Count as usize],
    // robots: [usize; Roboto::Count as usize],
}

impl State {
    fn tick(&mut self) {
        let robots = self.robots.iter();
        for (resource, production) in robots {
            *self.resources.get_mut(&resource).unwrap() += production;
        }
        self.step += 1;
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            robots: HashMap::from_iter(vec![(Resource::Ore, 1)]),
            resources: HashMap::from_iter(vec![
                (Resource::Ore, 0),
                (Resource::Clay, 0),
                (Resource::Obsidian, 0),
                (Resource::Geode, 0),
            ]),
            step: 0,
        }
    }
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut robots: Vec<_> = self.robots.iter().collect();
        robots.sort_by_key(|(res, _)| **res);
        let mut resources: Vec<_> = self.resources.iter().collect();
        resources.sort_by_key(|(res, _)| **res);
        robots.hash(state);
        resources.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        if self.robots.len() != other.robots.len() {
            false
        } else {
            let mut result = true;
            for (res, count) in self.resources.iter() {
                result = result && (other.resources[res] == *count);
            }
            for (res, count) in self.robots.iter() {
                result = result && (other.robots[res] == *count);
            }
            result
        }
    }
}
impl Eq for State {}

#[derive(Debug)]
struct Recipe {
    costs: Inventory,
    robot_type: Resource,
}


#[derive(Debug)]
struct Blueprint {
    id: usize,
    recipes: Vec<Recipe>,
}

impl Blueprint {
    fn possible(&self, resources: &Inventory) -> Vec<(Resource, Inventory)> {
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

fn parse_blueprint() -> Blueprint {
    Blueprint::from("Blueprint 0: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.")
}

fn maximum_possible_geodes(state: &State) -> usize {
    let remaining_steps: usize = MAX_STEPS - state.step;
    let n = remaining_steps + 1;

    let max_possible = (((n + 1) * n) / 2)
        + (state.robots.get(&Resource::Geode).unwrap_or(&0) * n)
        + state.resources[&Resource::Geode];
    (max_possible)
}

fn traverse_depth_first(
    mut state: State,
    current_max: usize,
    blueprint: &Blueprint,
    metastate: &mut MetaState,
) -> Option<usize> {
    let decisions = blueprint.possible(&state.resources);

    if maximum_possible_geodes(&state) < current_max {
        metastate.pruned_states += 1;
        return None;
    }

    state.tick();

    if let Some(similar_state) = metastate.explored_states.get(&state) {
        if similar_state.step <= state.step {
            metastate.duplicated_states += 1;
            return None;
        }
    }

    if state.step > MAX_STEPS {
        let geodes = state.resources[&Resource::Geode];
        // // println!("{:?}", geodes);
        // if geodes > current_max {
        //     println!("New Max! {:?}", state);
        // }
        metastate.full_traversals += 1;
        return Some(geodes);
    }

    // println!("{:?}", decisions);

    let mut possible_new_states: Vec<_> = decisions
        .iter()
        .map(|(res, costs)| {
            let mut new_robots = state.robots.clone();
            new_robots
                .entry(*res)
                .and_modify(|robot| {
                    *robot += 1;
                })
                .or_insert(1);
            let mut new_inventory = state.resources.clone();
            for (res, cost) in costs {
                *new_inventory.get_mut(res).unwrap() -= cost;
            }
            State {
                robots: new_robots,
                resources: new_inventory,
                step: state.step,
            }
        })
        .collect();
    possible_new_states.push(state.clone());

    // possible_new_states.shuffle(&mut thread_rng());

    let mut new_max = current_max;
    for new_state in possible_new_states {
        let result = traverse_depth_first(new_state, new_max, blueprint, metastate);
        new_max = new_max.max(result.unwrap_or_default());
    }

    if new_max == current_max {
        if metastate.explored_states.contains(&state) {
            {
                let existing = metastate.explored_states.get(&state).unwrap();
                assert!(existing.step > state.step);
            }
            metastate.explored_states.remove(&state);
        } else {
            metastate.explored_states.insert(state);
        }
    }

    Some(new_max)
}

#[derive(Default)]
struct MetaState
{
    explored_states: HashSet<State>,
    duplicated_states: usize,
    pruned_states: usize,
    full_traversals: usize,
}

fn main() {
    println!("Hello elves!");

    let mut final_result: usize = 0;

    for line in INPUT.lines()
    {
        let mut metastate = MetaState::default();

        let blueprint = Blueprint::from(line);
        println!("{:?}", blueprint);
        let result = traverse_depth_first(State::default(), 0, &blueprint, &mut metastate);
        println!("{:?}", result);
        println!(
            "Skipped because duplicate: {}; pruned: {}; full traversals: {}.",
            metastate.duplicated_states,
            metastate.pruned_states,
            metastate.full_traversals
        );
        final_result += result.unwrap() * blueprint.id;
    }

    println!("Final result: {}", final_result);

}
