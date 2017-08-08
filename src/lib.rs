#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate rand;

mod optimize;

use std::io::{Read,Write};
use std::collections::{HashSet,HashMap};
use std::sync::{Arc,Mutex};

pub use optimize::{Optimizer, StateRater};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
struct PunterId(pub usize);
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
struct SiteId(pub usize);

/// RiverId is our private way of identifying a river.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
struct RiverId(pub usize);

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
    #[serde(rename = "claim")]
    Claim {
        punter: PunterId,
        source: SiteId,
        target: SiteId,
    },
    #[serde(rename = "pass")]
    Pass {
        punter: PunterId
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RiverData {
    id: RiverId,
    sites: [SiteId; 2],
    claimed: Option<PunterId>,
    option_claimed: Option<PunterId>,
}

trait Measurer : Default {
    fn measure(&mut self, state: &State) -> f64;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
struct Plan {
    value: f64,
    river: RiverId,
    why: String,
    done_flags: HashSet<String>,
}
impl Plan {
    fn new() -> Plan {
        Plan {
            value: -1e200,
            river: RiverId(0),
            why: String::from("new"),
            done_flags: HashSet::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct State {
    punter: PunterId,
    punters: usize,
    map: Map,
    #[serde(default)]
    rivermap: HashMap<SiteId,HashMap<SiteId,RiverId>>,
    #[serde(default)]
    riverdata: Vec<RiverData>,
    optimizer: Optimizer,
    done_flags: HashSet<String>,
}

impl State {
    fn new(s: Setup, optimizer: Optimizer) -> State {
        let mut rivermap: HashMap<SiteId,HashMap<SiteId,RiverId>> = HashMap::new();
        let mut riverdata: Vec<RiverData> = Vec::new();
        let mut next = 0;
        for site in s.map.sites.iter().map(|s| s.id) {
            rivermap.insert(site, HashMap::new());
        }
        // The following somewhat convoluted code ensures that
        // riverdata is sorted in order of distance from mines.  This
        // is so that provided we iterate through the rivers in order,
        // if we can't try all of them, at least we'll try those that
        // are closest to the mines.
        let mut old_sites: HashSet<_> = s.map.mines.iter().cloned().collect();
        let mut rivers_done = HashSet::new();
        while old_sites.len() > 0 {
            let rivers_done_copy = rivers_done.clone();
            let mut new_sites = HashSet::new();
            for r in s.map.rivers.iter().cloned()
                .filter(|&r| !rivers_done_copy.contains(&(r.source,r.target)))
                .filter(|&r| old_sites.contains(&r.target) || old_sites.contains(&r.source))
            {
                rivers_done.insert((r.source,r.target));
                if !old_sites.contains(&r.target) {
                    new_sites.insert(r.target);
                }
                if !old_sites.contains(&r.source) {
                    new_sites.insert(r.source);
                }
                let id = RiverId(next);
                next += 1;
                riverdata.push(RiverData {
                    id: id,
                    sites: [r.target, r.source],
                    claimed: None,
                    option_claimed: None,
                });
                // eprintln!("river goes from {:?} to {:?}", r.target, r.source);
                for &(site,other) in &[(r.source, r.target), (r.target, r.source)] {
                    let mut had_it = false;
                    if let Some(child) = rivermap.get_mut(&site) {
                        child.insert(other, id);
                        had_it = true;
                    }
                    if !had_it {
                        let mut child = HashMap::new();
                        child.insert(other, id);
                        rivermap.insert(site, child);
                    }
                }
            }
            // the following handles the case where some rivers cannot
            // ever reach a mine.  We could simply remove these
            // rivers, but we cannot ensure that they won't be
            // selected, and we don't want our rivermap to be lacking
            // them.
            if new_sites.len() == 0 {
                for r in s.map.rivers.iter().cloned()
                    .filter(|&r| !rivers_done.contains(&(r.source,r.target)))
                {
                    new_sites.insert(r.source);
                    new_sites.insert(r.target);
                }
            }
            old_sites = new_sites;
        }
        // FIXME eventually we want some AI in here, to make the most
        // of our 10 seconds! This also means we need a place in State
        // to store the plan we come up with.
        State {
            punter: s.punter,
            punters: s.punters,
            map: s.map,
            rivermap: rivermap,
            riverdata: riverdata,
            optimizer: optimizer,
            done_flags: HashSet::new(),
        }
    }
    /// Here we use the AI to decide what to do.
    fn play(&mut self) -> Move {
        let bestlaidplan = Arc::new(Mutex::new(Plan::new()));
        let otherplan = Arc::clone(&bestlaidplan);
        let state_copy = self.clone();
        std::thread::spawn(move || {
            state_copy.optimizer.optimize(&state_copy, otherplan);
        });
        std::thread::sleep(std::time::Duration::from_millis(900));
        let final_plan = bestlaidplan.lock().unwrap();
        if final_plan.done_flags.len() > 0 {
            self.done_flags = final_plan.done_flags.clone();
        }
        let sites = self.riverdata[final_plan.river.0].sites;
        Move::Claim {
            punter: self.punter,
            source: sites[0],
            target: sites[1],
        }
        // Move::pass {
        //     punter: self.punter,
        // }
    }
    /// Here we adjust the State based on the moves that we were told
    /// about by the server.
    fn apply_moves(&mut self, moves: Moves) {
        for m in moves.moves.iter() {
            match m {
                &Move::Pass {punter: _} => (),
                &Move::Claim { punter, source, target } => {
                    // eprintln!("claim {:?} {:?}", source, target);
                    let rid = self.rivermap[&source][&target];
                    if self.riverdata[rid.0].claimed.is_none() {
                        //eprintln!("{:?} got the river {:?}!", punter, rid);
                        self.riverdata[rid.0].claimed = Some(punter);
                    }
                    //eprintln!("punter {:?} claims {:?}->{:?} aka {:?}",
                    //          punter, source, target, rid);
                },
            }
        }
    }
}

pub fn main_helper(optimizer: Optimizer) {
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
        let state = State::new(s, optimizer);
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
        std::process::exit(0);
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
