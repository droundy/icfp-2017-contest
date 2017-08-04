#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct State {
    punter: usize,
    punters: usize,
    map: Map,
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

impl State {
    fn new(s: Setup) -> State {
        State {
            punter: s.punter,
            punters: s.punters,
            map: s.map,
        }
    }
    fn play(&mut self) -> Move {
        Move::pass {
            punter: self.punter,
        }
    }
    fn apply_moves(&mut self, moves: Moves) {
    }
}

fn main() {
    let mut input = String::new();
    eprintln!("hello world");
    match std::io::stdin().read_line(&mut input) {
        Ok(n) => {
            eprintln!("{} bytes read", n);
            eprintln!("{}", input);
        }
        Err(error) => eprintln!("error: {}", error),
    }
    let input = input.replace("\"move\":", "\"move_\":");
    if let Ok(s) = serde_json::from_str::<Setup>(&input) {
        eprintln!("It is a setup!\n");
        let state = State::new(s);
        println!("{}", serde_json::to_string(&Ready {
            punter: state.punter,
            state: state,
        }).unwrap());
    } else if let Ok(play) = serde_json::from_str::<Gameplay>(&input) {
        println!("It is a play!");
        let mut state = play.state;
        state.apply_moves(play.move_);
        let mv = state.play();
        let mut movestr = serde_json::to_string(&mv).unwrap();
        let movelen = movestr.len();
        movestr.truncate(movelen-1);
        let statestr = serde_json::to_string(&state).unwrap();
        println!("{}, \"state\": {}}}", movestr, statestr);
    } else {
        eprintln!("It is neither");
        serde_json::from_str::<Gameplay>(&input).unwrap();
    }
}
