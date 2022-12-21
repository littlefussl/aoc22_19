use day19::{
    blueprint::Blueprint,
    max_search::{traverse_depth_first, MetaState},
    state::State,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Hello elves!");

    let mut final_result: usize = 0;

    for line in INPUT.lines() {
        let mut metastate = MetaState::with_max_steps(31);

        let blueprint = Blueprint::from(line);
        println!("{:?}", blueprint);
        let result = traverse_depth_first(State::default(), 0, &blueprint, &mut metastate);
        println!("{:?}", result);
        println!(
            "Skipped because duplicate: {}; pruned: {}; full traversals: {}.",
            metastate.duplicated_states, metastate.pruned_states, metastate.full_traversals
        );
        final_result += result.unwrap() as usize * blueprint.id as usize;
    }

    println!("Final result: {}", final_result);
}
