use super::*;

use rand::random;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Optimizer {
    Random,
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
pub enum Rater {
    Score,
}

impl Rater {
    fn score(&self, state: &State, punter: PunterId) -> f64 {
        match self {
            &Rater::Score => {
                0.0
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

// def punter_reaches(nice, mineid, punterid):
//     reached = set([])
//     old_sites = {mineid}
//     while len(old_sites) > 0:
//         new_sites = set([])
//         for site in old_sites:
//             for neighbor in nice['rivermap'][site]:
//                 if neighbor not in reached and nice['riverdata'][nice['rivermap'][site][neighbor]]['claimed'] == punterid:
//                     reached.add(neighbor)
//                     new_sites.add(neighbor)
//         old_sites = new_sites
//     return reached
