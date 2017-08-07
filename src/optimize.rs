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
    ConnectMines(StateRater),
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
            &Optimizer::ConnectMines(ref rater) => {
                let mut state = state.clone();
                let mut importance = vec![0.0; state.riverdata.len()];
                let mut best = 0.0;
                let mut mines_left = HashSet::new();
                for m1 in state.map.mines.iter().cloned() {
                    let d_old = my_distances(&state, m1);
                    for m2 in state.map.mines.iter().filter(|m2| m2.0 > m1.0)
                        .filter(|m2| d_old.contains_key(m2) && d_old[m2] > 0)
                    {
                        mines_left.insert(*m2);
                        mines_left.insert(m1);
                    }
                    for r in 0..state.riverdata.len() {
                        if state.riverdata[r].claimed.is_none() {
                            state.riverdata[r].claimed = Some(PunterId(500));
                            let d_new = my_distances(&state, m1);
                            for m2 in state.map.mines.iter().cloned()
                                .filter(|m2| m2.0 > m1.0)
                                .filter(|m2| d_old.contains_key(m2))
                                .filter(|m2| d_old[&m2] > 0)
                            {
                                let dist = if !d_new.contains_key(&m2) {
                                    state.riverdata.len()
                                } else {
                                    d_new[&m2] - d_old[&m2]
                                } as f64;
                                importance[r] += dist/d_old[&m2] as f64;
                                if importance[r] > best {
                                    best = importance[r];
                                    if let Ok(mut plan) = bestlaidplan.lock() {
                                        if plan.value < importance[r] {
                                            *plan = Plan {
                                                value: importance[r],
                                                river: RiverId(r),
                                                why: format!("connect mines"),
                                            };
                                        }
                                    }
                                }

                            }
                            state.riverdata[r].claimed = None;
                        }
                    }
                }
                if best > 0.0 {
                    //eprintln!("made a nice connection! {}  :)", best);
                    return;
                }
                for m1 in mines_left.iter().cloned() {
                    let d1 = my_distances(&state, m1);
                    for m2 in mines_left.iter().filter(|m2| m2.0 > m1.0)
                        .filter(|m2| d1.contains_key(m2) && d1[m2] > 0)
                        .cloned()
                    {
                        let d12 = d1[&m2];
                        let d2 = my_distances(&state, m2);
                        for r in d1.iter().filter(|&(_,&dd)| dd == 0)
                            .map(|(s1,_)| s1)
                            .filter(|s1| d2[s1] == d12)
                            .flat_map(|s1| state.rivermap[s1].iter())
                            .filter(|&(s2,_)| d2[s2] == d12-1)
                            .map(|(_,&r)| r)
                        {
                            if let Ok(mut plan) = bestlaidplan.lock() {
                                //eprintln!("connecting with ambiguity...");
                                *plan = Plan {
                                    value: 100.0,
                                    river: r,
                                    why: format!("connect mines with ambiguity"),
                                };
                            }
                            return;
                        }
                    }
                }
                //eprintln!("no connections needed?");
                Optimizer::Greedy(rater.clone()).optimize(&state, bestlaidplan);
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
    EnemyScore,
    NetScore,
    BottleNecks,
    Desperate,
    Mines,
    AllMines,
    Add(Box<StateRater>, Box<StateRater>),
    Scale(Box<StateRater>,f64),
    /// checks two moves ahead
    NextMove(Box<StateRater>, f64),
    /// checks two moves ahead
    LookAhead(Box<StateRater>, usize),
}

impl StateRater {
    fn score(&self, state: &State) -> f64 {
        match self {
            &StateRater::NetScore => {
                StateRater::Score.score(state) + 0.5*StateRater::EnemyScore.score(state)
            },
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
            &StateRater::EnemyScore => {
                let mut totalscore = 0;
                let mut tempstate = state.clone();
                for i in 0 .. state.punters {
                    if i != state.punter.0 {
                        tempstate.punter = PunterId(i);
                        for &m in state.map.mines.iter() {
                            let d = distances(&tempstate, m);
                            for r in punter_reaches(&tempstate, m) {
                                totalscore += d[&r]*d[&r];
                            }
                        }
                    }
                }
                (totalscore as f64)/(-(state.punters as f64) + 1.0)
            },
            &StateRater::BottleNecks => {
                let mut totalscore = 0.0;
                for s in state.map.sites.iter().map(|s| s.id).filter(|&s| we_touch(state,s)) {
                    let popularity = state.rivermap[&s].len() as f64;
                    totalscore += 1.0/(popularity*popularity);
                }
                totalscore
            },
            &StateRater::Desperate => {
                let mut totalscore = 0.0;
                for s in state.map.sites.iter().map(|s| s.id).filter(|&s| !we_touch(state,s)) {
                    let available = state.rivermap[&s].values()
                        .filter(|&s| state.riverdata[s.0].claimed == None).count() as f64;
                    totalscore += -1.0/(available*available);
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
            &StateRater::NextMove(ref rater, scaledown) => {
                let mut state = state.clone();
                let enemy = if state.punter.0 == 0 {
                    PunterId(1)
                } else {
                    PunterId(0)
                };
                let initial_score = rater.score(&state);
                let mut worstbest = 1e200;
                let mut secondworstbest = 1e200;
                for &enemyrid in &rivers_available(&state) {
                    // experiment with enemy claiming this river for themselves.
                    state.riverdata[enemyrid.0].claimed = Some(enemy);
                    let mut best = -1e200;
                    for rid in &rivers_available(&state) {
                        // experiment with claiming this river for ourselves.
                        state.riverdata[rid.0].claimed = Some(state.punter);
                        let score = rater.score(&state);
                        if score > best {
                            best = score;
                        }
                        state.riverdata[rid.0].claimed = None;
                    }
                    if best < worstbest {
                        secondworstbest = worstbest;
                        worstbest = best;
                        if secondworstbest == 1e200 {
                            secondworstbest = worstbest;
                        }
                    }
                    state.riverdata[enemyrid.0].claimed = None;
                }
                // eprintln!("secondworst: {} worst: {}", secondworstbest, worstbest);
                initial_score + (secondworstbest - initial_score)/scaledown
            },
            &StateRater::LookAhead(ref rater, lookahead) => {
                let mut state = state.clone();
                let initial_score = rater.score(&state);
                let mut best = -1e200;
                let avail = rivers_available(&state);
                let options = plans_of_length(&avail, lookahead);
                for plan in &options {
                    // experiment with claiming this river for ourselves.
                    for &rid in plan.iter() {
                        state.riverdata[rid.0].claimed = Some(state.punter);
                    }
                    let score = if lookahead <= 1 {
                        rater.score(&state)
                    } else {
                        StateRater::LookAhead(rater.clone(), lookahead-1).score(&state)
                    };
                    if score > best {
                        best = score;
                    }
                    for &rid in plan.iter() {
                        state.riverdata[rid.0].claimed = None;
                    }
                }
                initial_score + (best - initial_score)/(lookahead as f64 + 1.0)
            },
            &StateRater::Add(ref a, ref b) => {
                let ascore = a.score(state);
                let bscore = b.score(state);
                //eprintln!("{} ({:?}) + {} ({:?}) = {}", ascore, a, bscore, b, ascore+bscore);
                ascore+bscore
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
impl std::ops::Div<f64> for StateRater {
    type Output = StateRater;
    fn div(self, scaledown: f64) -> StateRater {
        StateRater::NextMove(Box::new(self), scaledown)
    }
}

impl std::ops::Not for StateRater {
    type Output = StateRater;
    fn not(self) -> StateRater {
        match self {
            StateRater::LookAhead(r,n) => StateRater::LookAhead(r,n+1),
            r => StateRater::LookAhead(Box::new(r), 1),
        }
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

fn my_distances(state: &State, mineid: SiteId) -> HashMap<SiteId, usize> {
    let mut distances = HashMap::new();
    distances.insert(mineid, 0);
    let mut current_distance = 0;
    let mut old_sites = HashSet::new();
    old_sites.insert(mineid);
    while distances.len() < state.map.sites.len() && old_sites.len() > 0 {
        let mut new_sites = HashSet::new();
        for &site in old_sites.iter() {
            for &neighbor in state.rivermap[&site].iter()
                .map(|(k,r)| (k,state.riverdata[r.0].clone()))
                .filter(|&(_,ref r)| r.claimed == Some(state.punter))
                .map(|(k,_)| k)
            {
                if !distances.contains_key(&neighbor) && !old_sites.contains(&neighbor) {
                    distances.insert(neighbor, current_distance);
                    new_sites.insert(neighbor);
                }
            }
        }
        if new_sites.len() > 0 {
            old_sites.extend(new_sites);
            continue;
        }
        current_distance += 1;
        for &site in old_sites.iter() {
            for &neighbor in state.rivermap[&site].iter()
                .map(|(k,r)| (k,state.riverdata[r.0].clone()))
                .filter(|&(_,ref r)| r.claimed.is_none())
                .map(|(k,_)| k)
            {
                if !distances.contains_key(&neighbor) {
                    distances.insert(neighbor, current_distance);
                    new_sites.insert(neighbor);
                }
            }
        }
        old_sites = new_sites;
    }
    distances
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

const MAX_ALLOWED_SIZE: usize = 100*1024;

fn plans_of_length(rivers: &[RiverId], len: usize) -> Vec<Vec<RiverId>> {
    if len == 1 {
        return rivers.iter().map(|&r| vec![r]).collect();
    }
    let shorter_plans = plans_of_length(rivers, len-1);
    if shorter_plans.len()*rivers.len() > MAX_ALLOWED_SIZE {
        return Vec::new();
    }
    let mut output = Vec::with_capacity(rivers.len()*shorter_plans.len());
    for r in rivers {
        for plan in shorter_plans.iter() {
            if plan[0].0 > r.0 { // enforce sorted plans!
                let mut v = Vec::with_capacity(len);
                v.push(*r);
                v.extend(plan);
                output.push(v);
            }
        }
    }
    output
}
