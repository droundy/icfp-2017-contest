extern crate punter;
use punter::StateRater::*;

fn main() {
    let rater = Score + 0.1*BottleNecks + 1000.0*AllMines;
    punter::main_helper(
        punter::Optimizer::Greedy(rater.clone())
        + punter::Optimizer::Greedy(!rater.clone())
        + punter::Optimizer::Greedy(!!rater.clone())
        + punter::Optimizer::Greedy(!!!rater.clone())
    );
}
