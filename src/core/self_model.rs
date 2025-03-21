use std::collections::HashMap;
use crate::core::self_evolving_fractal_gossip_node::SelfEvolvingFractalGossipNode;

#[derive(Clone)]
pub struct SelfModel {
    self_state: HashMap<String, f64>,
    quantum_state: HashMap<String, f64>,
    cached_depth: Option<f64>,
    last_update: chrono::DateTime<chrono::Utc>,
}

impl SelfModel {
    pub fn new() -> Self {
        Self {
            self_state: HashMap::from([("entropy".to_string(), 0.0), ("valence".to_string(), 0.0)]),
            quantum_state: HashMap::from([("cosmic_entropy".to_string(), 0.0)]),
            cached_depth: None,
            last_update: chrono::DateTime::<chrono::Utc>::MIN_UTC,
        }
    }

    pub fn update_self(&mut self, node: &SelfEvolvingFractalGossipNode) {
        self.self_state.insert("entropy".to_string(), node.entropy.load(Ordering::Relaxed) as f64);
        self.self_state.insert("valence".to_string(), node.emotional_state_model.get_current_valence());
        self.quantum_state.insert("cosmic_entropy".to_string(), crate::quantum::cosmic_entropy::CosmicEntropy::calculate(node));
    }

    pub fn get_recursive_depth(&mut self) -> f64 {
        if let Some(depth) = self.cached_depth {
            if chrono::Utc::now().signed_duration_since(self.last_update).num_seconds() < 5 {
                return depth;
            }
        }
        let depth = (1.0 + self.self_state.len() as f64 + self.quantum_state.len() as f64 * 0.3).ln();
        self.cached_depth = Some(depth);
        self.last_update = chrono::Utc::now();
        depth
    }
}
