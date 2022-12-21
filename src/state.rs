use std::collections::HashMap;
use std::hash::Hash;

use crate::resource::Resource;

type Inventory = HashMap<Resource, usize>;

#[derive(Debug, Clone)]
pub struct State {
    pub robots: Inventory,
    pub resources: Inventory,
    pub step: usize,
    // resources: [usize; Roboto::Count as usize],
    // robots: [usize; Roboto::Count as usize],
}

impl State {
    pub fn tick(&mut self) {
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
