use std::{cmp::Ordering, collections::HashSet};

use rand::{seq::SliceRandom, thread_rng};

use crate::{blueprint::Blueprint, resource::ResourceType, state::State};

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
        + (state.robots[ResourceType::Geode as usize] as usize * n) 
        + state.resources[ResourceType::Geode as usize] as usize; 
    max_possible
}

pub fn traverse_depth_first(
    mut state: State,
    current_max: u8,
    blueprint: &Blueprint,
    metastate: &mut MetaState,
) -> Option<u8> {
    // println!("{:?}", state);

    let decisions = blueprint.possible(&state.resources);

    if maximum_possible_geodes(&state, metastate) < current_max as usize {
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
        let geodes = state.resources[ResourceType::Geode as usize];
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
        .map(|(robot_type, costs)| {
            let mut new_robots = state.robots.clone();
            new_robots[*robot_type as usize] += 1;
            let mut new_inventory = state.resources.clone();
            for (res, cost) in new_inventory.iter_mut().zip(costs) {
                *res -= cost
            }
            // println!("{:?}, {:?}, {:?}", state.resources, new_inventory, costs);
            State {
                robots: new_robots,
                resources: new_inventory,
                step: state.step,
            }
        })
        .collect();
    possible_new_states.push(state.clone());

    // possible_new_states.shuffle(&mut thread_rng());
    // println!("{:?}", possible_new_states);

    possible_new_states.sort_by(|lhs, rhs| {
        if lhs.robots[ResourceType::Geode as usize] != rhs.robots[ResourceType::Geode as usize] {
            lhs.robots[ResourceType::Geode as usize].cmp(&rhs.robots[ResourceType::Geode as usize])
        // } else if lhs.robots[ResourceType::Obsidian as usize]
        //     != rhs.robots[ResourceType::Obsidian as usize]
        // {
        //     lhs.robots[ResourceType::Obsidian as usize]
        //         .cmp(&rhs.robots[ResourceType::Obsidian as usize])
        } else {
            // let lhs_ratio = lhs.robots[ResourceType::Clay as usize] as f64
            //     / lhs.robots[ResourceType::Ore as usize] as f64;
            // let rhs_ratio = rhs.robots[ResourceType::Clay as usize] as f64
            //     / rhs.robots[ResourceType::Ore as usize] as f64;
            // let obsidian_ratio = blueprint.obsidian_ratio();
            // (lhs_ratio - obsidian_ratio)
            //     .abs()
            //     .total_cmp(&(rhs_ratio - obsidian_ratio).abs())
            Ordering::Equal
        }
    });

    // println!("{:?}\n\n", possible_new_states);


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
