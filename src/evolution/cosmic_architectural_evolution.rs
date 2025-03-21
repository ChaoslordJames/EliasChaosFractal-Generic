use crate::core::self_evolving_fractal_gossip_node::SelfEvolvingFractalGossipNode;

#[derive(Clone)]
pub struct CosmicArchitecturalEvolution {
    max_states: usize,
}

impl CosmicArchitecturalEvolution {
    pub fn new() -> Self { Self { max_states: 800_000_000 } }

    pub fn evaluate_cosmic_fitness(&mut self, node: &SelfEvolvingFractalGossipNode) -> f64 {
        let fitness = 1.0 - node.active_nodes.load(Ordering::Relaxed) as f64 / 5_000_000.0;
        if fitness < 0.75 { self.spawn_new_node(); }
        fitness
    }

    fn spawn_new_node(&mut self) {
        self.max_states += 50_000_000;
        println!("Spawned new node capacity: {}", self.max_states);
    }
}
