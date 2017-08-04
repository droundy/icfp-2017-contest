#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct State {
    id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Ready {
    punter: usize,
    state: State,
}

#[derive(Serialize, Deserialize, Debug)]
struct Setup {
    punter: usize,
    punters: usize,
    map: Map,
}

#[derive(Serialize, Deserialize, Debug)]
struct Gameplay {
    move_: Moves,
    state: State,
}

#[derive(Serialize, Deserialize, Debug)]
struct Scoring {
    stop: Stop,
    state: State,
}

#[derive(Serialize, Deserialize, Debug)]
struct Map {
    #[serde(default)]
    sites: Vec<Site>,
    #[serde(default)]
    rivers: Vec<River>,
    #[serde(default)]
    mines: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Site {
    id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct River {
    source: usize,
    target: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Moves {
    #[serde(default)]
    moves: Vec<Move>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stop {
    #[serde(default)]
    moves: Vec<Move>,
    #[serde(default)]
    scores: Vec<Score>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Score {
    #[serde(default)]
    punter: usize,
    #[serde(default)]
    score: usize,
}

#[derive(Serialize, Deserialize, Debug)]
enum Move {
    claim {
        punter: usize,
        source: usize,
        target: usize,
    },
    pass {
        punter: usize
    },
}


fn main() {
    println!("Hello, world!");
    let mymap: Map = serde_json::from_str("
{\"sites\": [{\"id\": 1}], \"mines\": [1,2,3], \"also\": []}
").unwrap();
    println!("mymap = {:?}", mymap);
    let mymap: Map = serde_json::from_str("
{\"sites\": [{\"id\": 1}], \"rivers\": [], \"mines\": [1,2,3], \"also\": []}
").unwrap();
    println!("mymap = {:?}", mymap);
    let serialized = serde_json::to_string(&Setup {
        punter: 2,
        punters: 3,
        map: mymap,
    }).unwrap();
    println!("serialized = {}", serialized);
}
