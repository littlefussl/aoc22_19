use std::collections::HashMap;
use std::hash::Hash;

use crate::resource::{self, Inventory, ResourceType};

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
        let robots = self.robots.iter().enumerate();
        for (resource_type, quantity) in robots {
            self.resources[resource_type] += quantity;
        }
        self.step += 1;
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            robots: [1, 0, 0, 0],
            resources: Default::default(),
            step: Default::default(),
        }
    }
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.robots.hash(state);
        self.resources.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.robots == other.robots && self.resources == other.resources
    }
}
impl Eq for State {}
