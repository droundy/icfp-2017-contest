extern crate punter;
use punter::StateRater::*;

fn main() {
    punter::main_helper(punter::Optimizer::Greedy(Score + 0.1*BottleNecks + 1000.0*Mines));
}

