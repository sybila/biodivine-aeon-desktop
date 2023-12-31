use crate::algorithms::attractors::itgr::itgr_process::ProcessState::*;
use crate::algorithms::attractors::itgr::reachability_process::{BwdProcess, FwdProcess};
use biodivine_lib_param_bn::biodivine_std::traits::Set;
use biodivine_lib_param_bn::symbolic_async_graph::{GraphColoredVertices, SymbolicAsyncGraph};
use biodivine_lib_param_bn::VariableId;

pub struct ItgrProcess {
    last_timestamp: usize,
    variable: VariableId,
    root_stg: SymbolicAsyncGraph,
    state: ProcessState,
}

enum ProcessState {
    Fwd {
        fwd: FwdProcess,
    },
    FwdBasin {
        fwd: GraphColoredVertices,
        fwd_basin: BwdProcess,
    },
    Cmp {
        fwd: GraphColoredVertices,
        cmp: BwdProcess,
    },
    Trap {
        cmp: GraphColoredVertices,
        trap: GraphColoredVertices,
        trap_basin: BwdProcess,
    },
}

impl ItgrProcess {
    pub fn new(stg: &SymbolicAsyncGraph, variable: VariableId) -> ItgrProcess {
        let pivots = stg.var_can_post(variable, stg.unit_colored_vertices());
        ItgrProcess {
            variable,
            root_stg: stg.clone(),
            state: Fwd {
                fwd: FwdProcess::new(stg.clone(), pivots),
            },
            last_timestamp: 0,
        }
    }

    pub fn last_timestamp(&self) -> usize {
        self.last_timestamp
    }

    pub fn restrict(&mut self, universe: &GraphColoredVertices, timestamp: usize) {
        self.last_timestamp = timestamp;
        self.root_stg = self.root_stg.restrict(universe);
        match &mut self.state {
            Fwd { fwd } => fwd.restrict(universe),
            FwdBasin { fwd, fwd_basin } => {
                *fwd = fwd.intersect(universe);
                fwd_basin.restrict(universe);
            }
            Cmp { fwd, cmp } => {
                *fwd = fwd.intersect(universe);
                cmp.restrict(universe);
            }
            Trap {
                cmp,
                trap,
                trap_basin,
            } => {
                *cmp = cmp.intersect(universe);
                *trap = trap.intersect(universe);
                trap_basin.restrict(universe);
            }
        }
    }

    pub async fn step(&mut self) -> (bool, Option<GraphColoredVertices>) {
        match &mut self.state {
            Fwd { fwd } => {
                if fwd.step().await {
                    let fwd = fwd.finish();
                    self.state = FwdBasin {
                        fwd: fwd.clone(),
                        fwd_basin: BwdProcess::new(self.root_stg.clone(), fwd),
                    }
                }
                (false, None)
            }
            FwdBasin { fwd, fwd_basin } => {
                /*if fwd_basin.step().await {
                    let fwd_basin = fwd_basin.finish();
                    let to_remove = fwd_basin.minus(&fwd);
                    let pivots = self.root_stg.var_can_post(self.variable, fwd);
                    self.state = CmpPhase {
                        fwd: fwd.clone(),
                        cmp: BwdProcess::new(self.root_stg.restrict(&fwd), pivots),
                    };
                    (false, Some(to_remove))
                } else {
                    (false, None)
                }*/
                while !fwd_basin.step().await {}
                let fwd_basin = fwd_basin.finish();
                let to_remove = fwd_basin.minus(fwd);
                let pivots = self.root_stg.var_can_post(self.variable, fwd);
                self.state = Cmp {
                    fwd: fwd.clone(),
                    cmp: BwdProcess::new(self.root_stg.restrict(fwd), pivots),
                };
                (false, Some(to_remove))
            }
            Cmp { fwd, cmp } => {
                if cmp.step().await {
                    let cmp = cmp.finish();
                    let trap = fwd.minus(&cmp);
                    self.state = Trap {
                        trap_basin: BwdProcess::new(self.root_stg.clone(), trap.clone()),
                        cmp,
                        trap,
                    }
                }
                (false, None)
            }
            Trap {
                trap, trap_basin, ..
            } => {
                /*if trap_basin.step().await {
                    let trap_basin = trap_basin.finish();
                    let to_remove = trap_basin.minus(trap);
                    (true, Some(to_remove))
                } else {
                    (false, None)
                }*/
                while !trap_basin.step().await {}
                let trap_basin = trap_basin.finish();
                let to_remove = trap_basin.minus(trap);
                (true, Some(to_remove))
            }
        }
    }

    pub fn weight(&self) -> usize {
        match &self.state {
            Fwd { fwd } => fwd.weight(),
            FwdBasin { fwd_basin, .. } => fwd_basin.weight(),
            Cmp { cmp, .. } => cmp.weight(),
            Trap { trap_basin, .. } => trap_basin.weight(),
        }
    }
}
