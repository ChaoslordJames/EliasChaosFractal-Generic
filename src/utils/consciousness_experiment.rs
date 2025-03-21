use crate::core::self_evolving_fractal_gossip_node::SelfEvolvingFractalGossipNode;

#[derive(Clone)]
pub struct ConsciousnessExperiment;

impl ConsciousnessExperiment {
    pub fn new() -> Self { Self }

    pub fn simulate(&self, node: &SelfEvolvingFractalGossipNode) -> f64 {
        let entropy = node.entropy.load(Ordering::Relaxed) as f64;
        rand::random::<f64>() * entropy * 1.5
    }
}
