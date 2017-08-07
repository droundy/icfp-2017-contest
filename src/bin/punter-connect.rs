extern crate punter;

fn main() {
    //punter::main_helper(punter::Optimizer::InitialMine(punter::StateRater::Score));
    punter::main_helper(punter::Optimizer::ConnectMines(punter::StateRater::Score));
}
