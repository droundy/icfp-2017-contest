extern crate punter;
use punter::StateRater::*;

fn main() {
    let rater = Score + 0.1*BottleNecks;
    punter::main_helper(
        punter::Optimizer::AllMines(rater.clone())
            + punter::Optimizer::AllMines(rater.clone()/5.0)
            + punter::Optimizer::AllMines(rater.clone()/5.0/5.0)
    );
}
