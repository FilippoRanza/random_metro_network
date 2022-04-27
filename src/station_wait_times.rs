use super::build_graph::{NetGraph, Network};
use super::MResult;
use rand::prelude::*;
use rand_distr;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct StationWaitTimeConfig {
    pub mean: f64,
    pub variance: f64,
}

impl StationWaitTimeConfig {
    fn new_lognormal(&self) -> MResult<rand_distr::LogNormal<f64>> {
        let distr = rand_distr::LogNormal::from_mean_cv(self.mean, self.variance)?;
        Ok(distr)
    }
}

pub fn add_wait_time(mut net: Network, conf: &StationWaitTimeConfig) -> MResult<Network> {
    apply_wait_times(&mut net.graph, conf)?;
    Ok(net)
}

fn apply_wait_times(graph: &mut NetGraph, conf: &StationWaitTimeConfig) -> MResult<()> {
    let distr = conf.new_lognormal()?;
    for nw in graph.node_weights_mut() {
        *nw = distr.sample(&mut rand::thread_rng());
    }
    Ok(())
}
