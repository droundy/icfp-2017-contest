extern crate punter;
use punter::StateRater::*;

fn main() {
    punter::main_helper(
        punter::Optimizer::Greedy(Score + 0.1*BottleNecks + 100.0*AllMines)
            + punter::Optimizer::Greedy((Score + 0.1*BottleNecks + 100.0*AllMines)/5.0)
            + punter::Optimizer::Greedy((Score + 0.1*BottleNecks + 100.0*AllMines)/5.0/5.0)
    );
}

