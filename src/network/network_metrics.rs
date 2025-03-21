use crate::core::self_evolving_fractal_gossip_node::SelfEvolvingFractalGossipNode;

#[derive(Clone)]
pub struct NetworkMetrics;

impl NetworkMetrics {
    pub fn new() -> Self { Self }

    pub fn resilience_score(&self, node: &SelfEvolvingFractalGossipNode) -> f64 {
        let peer_count = node.active_nodes.load(Ordering::Relaxed) as f64;
        let score = (peer_count / 5_000_000.0 * 100.0).min(99.99);
        if score < 99.95 { println!("Warning: Resilience dropping: {}%", score); }
        score
    }

    pub fn throughput(&self, _node: &SelfEvolvingFractalGossipNode, queries: usize, seconds: f64) -> f64 {
        queries as f64 / seconds
    }
}
