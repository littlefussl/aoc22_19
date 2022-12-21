use std::collections::HashMap;

const MAX_STEPS: usize = 23;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

type Inventory = HashMap<Resource, usize>;

#[derive(Debug)]
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

struct Recipe {
    costs: Inventory,
    robot_type: Resource,
}

struct Blueprint {
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

fn parse_blueprint() -> Blueprint {
    // Each ore robot costs 4 ore.
    // Each clay robot costs 2 ore.
    // Each obsidian robot costs 3 ore and 14 clay.
    // Each geode robot costs 2 ore and 7 obsidian.
    Blueprint {
        recipes: vec![
            Recipe {
                costs: HashMap::from_iter(vec![(Resource::Ore, 4)]),
                robot_type: Resource::Ore,
            },
            Recipe {
                costs: HashMap::from_iter(vec![(Resource::Ore, 2)]),
                robot_type: Resource::Clay,
            },
            Recipe {
                costs: HashMap::from_iter(vec![(Resource::Ore, 3), (Resource::Clay, 14)]),
                robot_type: Resource::Obsidian,
            },
            Recipe {
                costs: HashMap::from_iter(vec![(Resource::Ore, 2), (Resource::Obsidian, 7)]),
                robot_type: Resource::Geode,
            },
        ],
    }
}

fn maximum_possible_geodes(state: &State) -> usize {
    let remaining_steps: usize = MAX_STEPS - state.step;
    let n = remaining_steps;

    let max_possible = (((n + 1) * n) / 2)
        + (state.robots.get(&Resource::Geode).unwrap_or(&0) * n)
        + state.resources[&Resource::Geode];
    (max_possible)
}

fn traverse_depth_first(
    mut state: State,
    current_max: usize,
    blueprint: &Blueprint,
) -> Option<usize> {
    let decisions = blueprint.possible(&state.resources);

    // println!("{:?}", state);

    if maximum_possible_geodes(&state) < current_max {
        return None;
    }

    state.tick();

    if state.step > MAX_STEPS {
        let geodes = state.resources[&Resource::Geode];
        println!("{:?}", geodes);
        if geodes > current_max {
            println!("New Max! {:?}", state);
        }
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
    possible_new_states.push(state);

    let mut new_max = current_max;
    for mut new_state in possible_new_states {
        let result = traverse_depth_first(new_state, new_max, blueprint);
        new_max = new_max.max(result.unwrap_or_default());
    }
    Some(new_max)
}

fn main() {
    println!("Hello elves!");
    let blueprint = parse_blueprint();

    let result = traverse_depth_first(State::default(), 0, &blueprint);
    println!("{:?}", result);
}
