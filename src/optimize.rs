use super::*;

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
                    if let Ok(mut plan) = bestlaidplan.lock() {
                        if plan.value < 0.0 {
                            *plan = Plan {
                                value: 0.0,
                                river: available[0].id,
                                why: format!("randopt with {} choices",
                                             available.len()),
                            };
                        }
                    }
                }
            },
            &Optimizer::InitialMine => {
                let available: Vec<_> = state.riverdata.iter()
                    .filter(|r| r.claimed.is_none()).cloned().collect();
                let minerivers: Vec<RiverData> = state.map.mines.iter()
                    .flat_map(|m| state.rivermap[m].values().map(|r| state.riverdata[r.0].clone()))
                    .filter(|r| r.claimed.is_none())
                    .collect();
                if minerivers.len() > 0 {
                    if let Ok(mut plan) = bestlaidplan.lock() {
                        if plan.value < 0.0 {
                            *plan = Plan {
                                value: 1.0,
                                river: available[0].id,
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


