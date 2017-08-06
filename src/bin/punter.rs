extern crate punter;

fn main() {
    punter::main_helper(punter::Optimizer::AllMines(punter::StateRater::Score));
}
