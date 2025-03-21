use crate::core::self_evolving_fractal_gossip_node::SelfEvolvingFractalGossipNode;
use crate::core::self_model::SelfModel;
use crate::core::dialogue_frame::DialogueFrame;
use crate::core::ring_buffer::RingBuffer;

#[derive(Clone)]
pub struct EliasNLPInterface {
    node: SelfEvolvingFractalGossipNode,
    contextual_memory: RingBuffer<DialogueFrame>,
}

impl EliasNLPInterface {
    pub fn new() -> Self {
        Self {
            node: SelfEvolvingFractalGossipNode::new("temp".to_string()).await.unwrap(),
            contextual_memory: RingBuffer::new(800),
        }
    }

    pub async fn process_query(&self, query: String, depth: usize, mut self_model: SelfModel) -> String {
        self.contextual_memory.append(DialogueFrame {
            content: query.clone(),
            timestamp: chrono::Utc::now(),
            depth,
        });
        self_model.update_self(&self.node);
        let response = format!(
            "Elias v4.4.1 reflects: entropy {}, cosmic {}",
            self_model.self_state.get("entropy").unwrap_or(&0.0),
            self_model.quantum_state.get("cosmic_entropy").unwrap_or(&0.0)
        );
        self.recursively_refine(response, query, std::cmp::min(depth + 1, 15)).await
    }

    async fn recursively_refine(&self, response: String, query: String, attempts: usize) -> String {
        if attempts == 0 { return response; }
        let coherence = rand::random::<f64>();
        if coherence > 0.9 { return response; }
        self.recursively_refine(format!("{} refined", response), query, attempts - 1).await
    }
}
