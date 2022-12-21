use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug, PartialOrd, Ord)]
pub enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

pub type Inventory = HashMap<Resource, usize>;
