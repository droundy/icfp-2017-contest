#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::io::{Read,Write};
use std::collections::hash_map::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
struct PunterId(pub usize);
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
struct SiteId(pub usize);

/// RiverId is our private way of identifying a river.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
struct RiverId(pub usize);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct State {
    punter: PunterId,
    punters: usize,
    map: Map,
    #[serde(default)]
    rivers_from: HashMap<SiteId,HashMap<SiteId,RiverId>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Ready {
    punter: PunterId,
    state: State,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Setup {
    punter: PunterId,
    punters: usize,
    map: Map,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Gameplay {
    #[serde(rename = "move")]
    move_: Moves,
    state: State,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Scoring {
    stop: Stop,
    state: State,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Map {
    #[serde(default)]
    sites: Vec<Site>,
    #[serde(default)]
    rivers: Vec<River>,
    #[serde(default)]
    mines: Vec<SiteId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Site {
    id: SiteId,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct River {
    source: SiteId,
    target: SiteId,
    #[serde(default)]
    claimed_by: Option<PunterId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Moves {
    #[serde(default)]
    moves: Vec<Move>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Stop {
    #[serde(default)]
    moves: Vec<Move>,
    #[serde(default)]
    scores: Vec<Score>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Score {
    #[serde(default)]
    punter: PunterId,
    #[serde(default)]
    score: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Move {
    claim {
        punter: PunterId,
        source: SiteId,
        target: SiteId,
    },
    pass {
        punter: PunterId
    },
}

impl State {
    fn new(s: Setup) -> State {
        let mut rivers_from: HashMap<SiteId,HashMap<SiteId,RiverId>> = HashMap::new();
        for r in s.map.rivers.iter() {
            for &(site,other) in &[(r.source, r.target), (r.target, r.source)] {
                let mut had_it = false;
                if let Some(child) = rivers_from.get_mut(&site) {
                    child.insert(other, RiverId(0));
                    had_it = true;
                }
                if !had_it {
                    let mut child = HashMap::new();
                    child.insert(other, RiverId(0));
                    rivers_from.insert(site, child);
                }
            }
        }
        // FIXME eventually we want some AI in here, to make the most
        // of our 10 seconds! This also means we need a place in State
        // to store the plan we come up with.
        State {
            punter: s.punter,
            punters: s.punters,
            map: s.map,
            rivers_from: rivers_from,
        }
    }
    /// Here we use the AI to decide what to do.
    fn play(&mut self) -> Move {
        Move::pass {
            punter: self.punter,
        }
    }
    /// Here we adjust the State based on the moves that we were told
    /// about by the server.
    fn apply_moves(&mut self, moves: Moves) {
        for m in moves.moves.iter() {
            match m {
                &Move::pass {punter: _} => (),
                &Move::claim { punter, source, target } => {
                    //eprintln!("punter {:?} claims {:?}->{:?}", punter, source, target);
                    for r in self.map.rivers.iter_mut().filter(|r| r.claimed_by.is_none()) {
                        if r.source == source && r.target == target {
                            r.claimed_by = Some(punter);
                        }
                    }
                },
            }
        }
    }
}

fn main() {
    // First send our greeting (and we always call ourselves "Xiphon"
    // for now)
    let mut greeting: HashMap<String,String> = HashMap::new();
    greeting.insert(String::from("me"), String::from("Xiphon"));
    print_string_with_length(&serde_json::to_string(&greeting).unwrap());

    // This is just the "you" response, which is unimportant, but
    // triggers the timer.
    let length = read_integer_to_colon();
    let mut input = vec![b'x'; length];
    match std::io::stdin().read_exact(input.as_mut_slice()) {
        Ok(()) => {
            //eprintln!("{}", String::from_utf8_lossy(&input));
        },
        Err(error) => println!("error: {}", error),
    }

    // Now we read the real thing!
    let length = read_integer_to_colon();
    let mut input = vec![b'x'; length];
    match std::io::stdin().read_exact(input.as_mut_slice()) {
        Ok(()) => {
            //eprintln!("{}", String::from_utf8_lossy(&input));
        },
        Err(error) => println!("error: {}", error),
    }

    // Now we see what we have, and act on it.
    if let Ok(s) = serde_json::from_slice::<Setup>(&input) {
        //eprintln!("It is a setup!\n");
        let state = State::new(s);
        print_string_with_length(&serde_json::to_string(&Ready {
            punter: state.punter,
            state: state,
        }).unwrap());
    } else if let Ok(play) = serde_json::from_slice::<Gameplay>(&input) {
        //println!("It is a play!");
        let mut state = play.state;
        state.apply_moves(play.move_);
        let mv = state.play();
        let mut movestr = serde_json::to_string(&mv).unwrap();
        let movelen = movestr.len();
        movestr.truncate(movelen-1);
        let statestr = serde_json::to_string(&state).unwrap();
        let totalstring = format!("{}, \"state\": {}}}", movestr, statestr);
        print_string_with_length(&totalstring);
    } else {
        println!("It is neither");
        serde_json::from_slice::<Gameplay>(&input).unwrap();
    }
}

fn print_string_with_length(s: &str) {
    print!("{}:{}", s.len(), s);
    std::io::stdout().flush().ok();
}

fn read_integer_to_colon() -> usize {
    let mut byte: [u8;1] = [0;1];
    let mut length = 0;
    loop {
        std::io::stdin().read_exact(&mut byte)
            .expect("there should be an integer followed by a colon");
        length *= 10;
        match byte[0] {
            b':' => return length/10,
            b'0' => (),
            b'1' => {
                length += 1;
            },
            b'2' => {
                length += 2;
            },
            b'3' => {
                length += 3;
            },
            b'4' => {
                length += 4;
            },
            b'5' => {
                length += 5;
            },
            b'6' => {
                length += 6;
            },
            b'7' => {
                length += 7;
            },
            b'8' => {
                length += 8;
            },
            b'9' => {
                length += 9;
            },
            _ => {
                panic!("You gave me a bad byte! {}", String::from_utf8_lossy(&byte));
            },
        }
    }
}
