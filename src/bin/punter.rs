extern crate punter;
use punter::StateRater;

fn main() {
    let rater = StateRater::Sum(vec![
        (StateRater::Score, 1.0),
        (StateRater::BottleNecks, 1.0),
        (StateRater::AllMines, 100.0),
    ]);
    punter::main_helper(punter::Optimizer::Greedy(rater));
}
