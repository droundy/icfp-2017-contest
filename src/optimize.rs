use super::*;

use rand::random;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Optimizer {
    Random,
    Greedy,
    InitialMine,

}

impl Optimizer {
    pub(crate) fn optimize(&self, state: &State, bestlaidplan: Arc<Mutex<Plan>>) {
        match self {
            &Optimizer::Random => {
                let available: Vec<_> = state.riverdata.iter()
                    .filter(|r| r.claimed.is_none()).cloned().collect();
                if available.len() > 0 {
                    //eprintln!("\navailable: {:?}", &available);
                    let choice = random::<usize>() % available.len();
                    if let Ok(mut plan) = bestlaidplan.lock() {
                        if plan.value < 0.0 {
                            *plan = Plan {
                                value: 0.0,
                                river: available[choice].id,
                                why: format!("random with {} choices",
                                             available.len()),
                            };
                        }
                    }
                }
            },
            &Optimizer::Greedy => {
                let mut available: Vec<_> = state.riverdata.iter()
                    .filter(|r| r.claimed.is_none()).map(|r| r.id).collect();
                thread_rng().shuffle(&mut available);
                pick_highest_rated(state, available, StateRater::Score, bestlaidplan);
            },
            &Optimizer::InitialMine => {
                let minerivers: Vec<RiverData> = state.map.mines.iter()
                    .flat_map(|m| state.rivermap[m].values().map(|r| state.riverdata[r.0].clone()))
                    .filter(|r| r.claimed.is_none())
                    .collect();
                if minerivers.len() > 0 {
                    let choice = random::<usize>() % minerivers.len();
                    if let Ok(mut plan) = bestlaidplan.lock() {
                        if plan.value < 0.0 {
                            *plan = Plan {
                                value: 1.0,
                                river: minerivers[choice].id,
                                why: format!("get to mine"),
                            };
                        }
                    }
                } else {
                    Optimizer::Random.optimize(state, bestlaidplan);
                }        
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StateRater {
    Score,
}

impl StateRater {
    fn score(&self, state: &State) -> f64 {
        match self {
            &StateRater::Score => {
                let mut totalscore = 0;
                for &m in state.map.mines.iter() {
                    let d = distances(state, m);
                    for r in punter_reaches(state, m) {
                        totalscore += d[&r]*d[&r];
                    }
                }
                totalscore as f64
            },
        }
    }
}

fn distances(state: &State, mineid: SiteId) -> HashMap<SiteId, usize> {
    let mut distances = HashMap::new();
    distances.insert(mineid, 0);
    let mut current_distance = 0;
    let mut old_sites = HashSet::new();
    old_sites.insert(mineid);
    while distances.len() < state.map.sites.len() {
        current_distance += 1;
        let mut new_sites = HashSet::new();
        for &site in old_sites.iter() {
            for &neighbor in state.rivermap[&site].keys() {
                if !distances.contains_key(&neighbor) {
                    distances.insert(neighbor, current_distance+1);
                    new_sites.insert(neighbor);
                }
            }
        }
        old_sites = new_sites;
    }
    distances
}

fn punter_reaches(state: &State, mineid: SiteId) -> HashSet<SiteId> {
    let mut reached = HashSet::new();
    let mut old_sites = HashSet::new();
    old_sites.insert(mineid);
    while old_sites.len() > 0 {
        let mut new_sites = HashSet::new();
        for &site in old_sites.iter() {
            for &neighbor in state.rivermap[&site].keys() {
                if !reached.contains(&neighbor) && state.riverdata[state.rivermap[&site][&neighbor].0].claimed == Some(state.punter) {
                    reached.insert(neighbor);
                    new_sites.insert(neighbor);
                }
            }
        }
        old_sites = new_sites;
    }
    reached
}

fn pick_highest_rated(state: &State, options: Vec<RiverId>, rater: StateRater,
                      bestlaidplan: Arc<Mutex<Plan>>) {
    let mut state = state.clone();
    let mut bestscore = -1e200;
    if let Ok(plan) = bestlaidplan.lock() {
        bestscore = plan.value;
    }
    for rid in options {
        // experiment with claiming this river for ourselves.
        state.riverdata[rid.0].claimed = Some(state.punter);
        let score = rater.score(&state);
        if score > bestscore {
            bestscore = score;
            if let Ok(mut plan) = bestlaidplan.lock() {
                if score > plan.value {
                    *plan = Plan {
                        value: score,
                        river: rid,
                        why: format!("rated {:?} with score {}", rater, score),
                    };
                } else {
                    bestscore = plan.value;
                }
            }
        }
        // return to actual current state.
        state.riverdata[rid.0].claimed = None;
    }
}
