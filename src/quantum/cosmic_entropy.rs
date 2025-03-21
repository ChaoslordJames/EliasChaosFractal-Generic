use crate::core::self_evolving_fractal_gossip_node::SelfEvolvingFractalGossipNode;

pub struct CosmicEntropy;

impl CosmicEntropy {
    pub fn calculate(node: &SelfEvolvingFractalGossipNode) -> f64 {
        let chaos_sum = node.chaos_history.last().map_or(0.0, |v| v.iter().sum());
        let peer_factor = (node.active_nodes.load(Ordering::Relaxed) as f64 + 1.0).log2();
        chaos_sum * peer_factor
    }
}
