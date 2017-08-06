extern crate punter;
use punter::StateRater::*;

fn main() {
    punter::main_helper(punter::Optimizer::Greedy(Score + 2*BottleNecks + 100.0*AllMines));
}
