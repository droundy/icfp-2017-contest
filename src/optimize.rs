use super::*;

use rand::random;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Optimizer {
    Random,
    Greedy(StateRater),
    AllMines(StateRater),
    Parallel(Box<Optimizer>, Box<Optimizer>),
    InitialMine(StateRater),
}
impl std::ops::Add for Optimizer {
    type Output = Optimizer;
    fn add(self, other: Optimizer) -> Optimizer {
        Optimizer::Parallel(Box::new(self), Box::new(other))
    }
}

impl Optimizer {
    pub(crate) fn optimize(&self, state: &State, bestlaidplan: Arc<Mutex<Plan>>) {
        match self {
            &Optimizer::Parallel(ref a, ref b) => {
                let otherplan = Arc::clone(&bestlaidplan);
                let state_copy = state.clone();
                let b = b.clone();
                std::thread::spawn(move || {
                    let state = state_copy;
                    b.optimize(&state, otherplan);
                });
                a.optimize(state, bestlaidplan);
            },
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
            &Optimizer::Greedy(ref rater) => {
                pick_highest_rated(state, rivers_available(state), rater, bestlaidplan);
            },
            &Optimizer::AllMines(ref rater) => {
                let mut minerivers: Vec<RiverId> = new_mines(state).iter()
                    .flat_map(|m| state.rivermap[m].values()
                              .map(|r| &state.riverdata[r.0]))
                    .filter(|r| r.claimed.is_none()).map(|r| r.id).collect();
                SiteRater::Farthest.sort_rivers(state, &mut minerivers);
                if minerivers.len() > 0 {
                    let choice = random::<usize>() % minerivers.len();
                    if let Ok(mut plan) = bestlaidplan.lock() {
                        if plan.value < 0.0 {
                            *plan = Plan {
                                value: 1.0,
                                river: minerivers[choice],
                                why: format!("get to all mines"),
                            };
                        }
                    }
                } else {
                    Optimizer::InitialMine(rater.clone()).optimize(state, bestlaidplan);
                }
            },
            &Optimizer::InitialMine(ref rater) => {
                let mut minerivers: Vec<RiverId> = state.map.mines.iter()
                    .flat_map(|m| state.rivermap[m].values().map(|r| state.riverdata[r.0].clone()))
                    .filter(|r| r.claimed.is_none()).map(|r| r.id).collect();
                SiteRater::Farthest.sort_rivers(state, &mut minerivers);
                if minerivers.len() > 0 {
                    let choice = random::<usize>() % minerivers.len();
                    if let Ok(mut plan) = bestlaidplan.lock() {
                        if plan.value < 0.0 {
                            *plan = Plan {
                                value: 1.0,
                                river: minerivers[choice],
                                why: format!("get to mine"),
                            };
                        }
                    }
                } else {
                    Optimizer::Greedy(rater.clone()).optimize(state, bestlaidplan);
                }
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum StateRater {
    Score,
    BottleNecks,
    Mines,
    AllMines,
    Add(Box<StateRater>, Box<StateRater>),
    Scale(Box<StateRater>,f64),
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
            &StateRater::BottleNecks => {
                let mut totalscore = 0.0;
                for s in state.map.sites.iter().map(|s| s.id).filter(|&s| we_touch(state,s)) {
                    let popularity = state.rivermap[&s].len() as f64;
                    totalscore += 1.0/(popularity*popularity);
                }
                totalscore
            },
            &StateRater::Mines => {
                let mut totalscore = 0.0;
                for s in state.map.mines.iter().cloned().filter(|&s| we_touch(state,s)) {
                    let popularity = state.rivermap[&s].len() as f64;
                    totalscore += 1.0/popularity;
                }
                totalscore
            },
            &StateRater::AllMines => {
                let mut totalscore = 0.0;
                for nn in state.map.mines.iter().map(|&s| num_touch(state,s)) {
                    totalscore += (nn as f64).sqrt();
                }
                totalscore
            },
            &StateRater::Add(ref a, ref b) => {
                a.score(state) + b.score(state)
            },
            &StateRater::Scale(ref r, factor) => {
                factor*r.score(state)
            },
        }
    }
}

impl std::ops::Add for StateRater {
    type Output = StateRater;
    fn add(self, other: StateRater) -> StateRater {
        StateRater::Add(Box::new(self), Box::new(other))
    }
}
impl std::ops::Mul<StateRater> for f64 {
    type Output = StateRater;
    fn mul(self, rater: StateRater) -> StateRater {
        StateRater::Scale(Box::new(rater), self)
    }
}
impl std::ops::Mul<StateRater> for isize {
    type Output = StateRater;
    fn mul(self, rater: StateRater) -> StateRater {
        StateRater::Scale(Box::new(rater), self as f64)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SiteRater {
    /// Rates a given site based on how far it is from our other sites
    /// that we "own".
    Farthest,
    /// Rates a site based on being unowned by us.
    NotOurs,
}

impl SiteRater {
    fn rate(&self, state: &State, site: SiteId) -> f64 {
        match self {
            &SiteRater::Farthest => {
                let mut score = 0.0;
                for (&s, &dist) in &distances(state, site) {
                    if SiteRater::NotOurs.rate(state, s) == 0.0 {
                        score -= 1.0 / (dist as f64 *dist as f64);
                    }
                }
                score
            },
            &SiteRater::NotOurs => {
                if state.rivermap[&site].values()
                    .any(|rid| state.riverdata[rid.0].claimed == Some(state.punter)) {
                        1.0
                    } else {
                        0.0
                    }
            },
        }
    }
    fn sort(&self, state: &State, sites: &mut [SiteId]) {
        let mut rates: HashMap<SiteId, f64> = HashMap::new();
        for &s in sites.iter() {
            rates.insert(s,  self.rate(state, s));
        }
        sites.sort_by(|a, b| compare(rates[b], rates[a]));
    }
    fn sort_rivers(&self, state: &State, rivers: &mut [RiverId]) {
        let mut rates = HashMap::new();
        let mut river_rates = HashMap::new();
        for &rid in rivers.iter() {
            let sites = state.riverdata[rid.0].sites;
            for &site in &sites {
                if !rates.contains_key(&site) {
                    rates.insert(site, self.rate(state, site));
                }
            }
            river_rates.insert(rid, rates[&sites[0]] + rates[&sites[1]]);
        }
        rivers.sort_by(|a, b| compare(river_rates[b], river_rates[a]));
    }

}

fn compare(a: f64, b: f64) -> Ordering {
    match a.partial_cmp(&b) {
        Some(o) => o,
        None => Ordering::Equal,
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

fn pick_highest_rated(state: &State, mut options: Vec<RiverId>, rater: &StateRater,
                      bestlaidplan: Arc<Mutex<Plan>>) {
    let mut state = state.clone();
    let mut bestscore = -1e200;
    if let Ok(plan) = bestlaidplan.lock() {
        bestscore = plan.value;
    }
    SiteRater::Farthest.sort_rivers(&state, &mut options);
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

fn rivers_available(state: &State) -> Vec<RiverId> {
    let mut available: Vec<_> = state.riverdata.iter()
        .filter(|r| r.claimed.is_none()).map(|r| r.id).collect();
    thread_rng().shuffle(&mut available);
    available
}

/// Do we have a river to here?
fn we_touch(state: &State, s: SiteId) -> bool {
    state.rivermap[&s].values()
        .any(|rid| state.riverdata[rid.0].claimed == Some(state.punter))
}

/// How many rivers to here?
fn num_touch(state: &State, s: SiteId) -> usize {
    state.rivermap[&s].values()
        .filter(|rid| state.riverdata[rid.0].claimed == Some(state.punter)).count()
}

/// Mines that we do not yet have rivers to.
fn new_mines(state: &State) -> Vec<SiteId> {
    let mut mines: Vec<_> = state.map.mines.iter().cloned()
        .filter(|&m| !we_touch(state, m)).collect();
    thread_rng().shuffle(&mut mines);
    SiteRater::Farthest.sort(&state, &mut mines);
    mines
}
