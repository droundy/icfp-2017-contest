#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct State {
    id: usize,
}

fn main() {
    println!("Hello, world!");
}
