use std::{collections::HashSet};

use crate::{blueprint::Blueprint, resource::Resource, state::State};

#[derive(Default)]
pub struct MetaState {
    pub explored_states: HashSet<State>,
    pub duplicated_states: usize,
    pub pruned_states: usize,
    pub full_traversals: usize,
    pub max_steps: usize,
}

impl MetaState {
    pub fn with_max_steps(max_steps: usize) -> Self {
        Self {
            max_steps: max_steps,
            ..Default::default()
        }
    }
}

fn maximum_possible_geodes(state: &State, metastate: &MetaState) -> usize {
    let remaining_steps: usize = metastate.max_steps - state.step;
    let n = remaining_steps + 1;

    let max_possible = (((n + 1) * n) / 2)
        + (state.robots.get(&Resource::Geode).unwrap_or(&0) * n)
        + state.resources[&Resource::Geode];
    max_possible
}

pub fn traverse_depth_first(
    mut state: State,
    current_max: usize,
    blueprint: &Blueprint,
    metastate: &mut MetaState,
) -> Option<usize> {
    let decisions = blueprint.possible(&state.resources);

    if maximum_possible_geodes(&state, metastate) < current_max {
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

    if state.step > metastate.max_steps {
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

    // println!("{:?}", possible_new_states);

    // possible_new_states.sort_by(|lhs, rhs| {
    //     if lhs.robots.get(&Resource::Geode).unwrap_or(&0)
    //         > rhs.robots.get(&Resource::Geode).unwrap_or(&0)
    //     {
    //         Ordering::Less
    //     } else if lhs.robots.get(&Resource::Obsidian).unwrap_or(&0)
    //         > rhs.robots.get(&Resource::Obsidian).unwrap_or(&0)
    //     {
    //         Ordering::Less
    //     }
    //     else {
    //         let lhs_ratio = *lhs.robots.get(&Resource::Clay).unwrap_or(&0) as f64 / *lhs.robots.get(&Resource::Ore).unwrap_or(&0) as f64;
    //         let rhs_ratio = *rhs.robots.get(&Resource::Clay).unwrap_or(&0) as f64 / *rhs.robots.get(&Resource::Ore).unwrap_or(&0) as f64;
    //         let obsidian_ratio = blueprint.obsidian_ratio();
    //         (lhs_ratio - obsidian_ratio).abs().total_cmp(&(rhs_ratio - obsidian_ratio).abs())
    //     }
    // });

    // println!("{:?}\n\n", possible_new_states);

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
