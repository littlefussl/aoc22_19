#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug, PartialOrd, Ord)]
pub enum ResourceType {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

pub type Inventory = [u8; 4];
