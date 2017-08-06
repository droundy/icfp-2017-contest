extern crate punter;
use punter::StateRater::*;

fn main() {
    punter::main_helper(
        punter::Optimizer::Greedy(Score + 0.0001*BottleNecks + 1000.0*AllMines)
            + punter::Optimizer::Greedy((Score + 0.0001*BottleNecks + 1000.0*AllMines)/5.0)
            + punter::Optimizer::Greedy((Score + 0.0001*BottleNecks + 1000.0*AllMines)/5.0/5.0)
    );
}

