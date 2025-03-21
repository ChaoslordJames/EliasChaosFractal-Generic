use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};
use crate::core::elias_nlp_interface::EliasNLPInterface;
use crate::core::emotional_state_model::EmotionalStateModel;
use crate::core::self_model::SelfModel;
use crate::network::cosmic_gossip_protocol::{CosmicGossipProtocol, State};
use crate::network::peer_discovery::PeerDiscovery;
use crate::quantum::quantum_fractal_tensor_engine::QuantumFractalTensorEngine;
use crate::rendering::cross_modal_cosmic_engine::CrossModalCosmicEngine;
use crate::storage::redis_interface::RedisInterface;
use crate::storage::state_manager::StateManager;

pub struct SelfEvolvingFractalGossipNode {
    peer_id: String,
    entropy: AtomicUsize,
    active_nodes: AtomicUsize,
    chaos_history: Vec<Vec<f64>>,
    tensor_engine: QuantumFractalTensorEngine,
    emotional_state_model: EmotionalStateModel,
    cross_modal_engine: CrossModalCosmicEngine,
    nli: EliasNLPInterface,
    state_manager: StateManager,
    redis: RedisInterface,
    peers: Vec<String>,
}

impl SelfEvolvingFractalGossipNode {
    pub async fn new(peer_id: String) -> Result<Self, Box<dyn std::error::Error>> {
        let node = Self {
            peer_id: peer_id.clone(),
            entropy: AtomicUsize::new(0),
            active_nodes: AtomicUsize::new(5000),
            chaos_history: Vec::with_capacity(10000),
            tensor_engine: QuantumFractalTensorEngine::new(),
            emotional_state_model: EmotionalStateModel::new(),
            cross_modal_engine: CrossModalCosmicEngine::new(),
            nli: EliasNLPInterface::new(),
            state_manager: StateManager::new(peer_id.clone()),
            redis: RedisInterface::new("localhost", 6379 + peer_id.split('_').last().unwrap().parse::<u16>()?),
            peers: Vec::new(),
        };
        PeerDiscovery::register_node(peer_id);
        tokio::spawn(node.clone().cosmic_sync_loop());
        tokio::spawn(node.clone().render_cross_modal_loop());
        Ok(node)
    }

    pub async fn process_query(&self, query: String) -> String {
        let semaphore = Arc::new(Semaphore::new(std::cmp::max(500, self.active_nodes.load(Ordering::Relaxed) / 1000)));
        let permit = semaphore.acquire().await.unwrap();
        self.synchronize_with_network(PeerDiscovery::get_peers(self).await).await;
        let response = self.nli.process_query(query, 0, SelfModel::new()).await;
        drop(permit);
        response
    }

    async fn cosmic_sync_loop(self) {
        loop {
            let peers = PeerDiscovery::get_peers(&self).await;
            let global_peers: Vec<_> = peers.iter().filter(|p| p.contains("global")).cloned().collect();
            let bandwidth_limit = 10_000_000; // 10MB/s
            let sync_size = std::cmp::min(global_peers.len(), bandwidth_limit / 1000); // 1KB/state
            let cosmic_entropy = CosmicEntropy::calculate(&self);
            self.entropy.store(cosmic_entropy as usize, Ordering::Relaxed);
            self.chaos_history.push(vec![cosmic_entropy]);
            if self.chaos_history.len() > 10000 { self.chaos_history.remove(0); }
            self.synchronize_with_network(peers.into_iter().take(sync_size).collect()).await;
            sleep(Duration::from_millis(500)).await;
        }
    }

    async fn render_cross_modal_loop(self) {
        loop {
            self.cross_modal_engine.render_live_fractal(&self.tensor_engine).await;
            self.cross_modal_engine.render_live_soundscape(&self.emotional_state_model).await;
            sleep(Duration::from_secs(2)).await; // 0.5 FPS
        }
    }

    async fn synchronize_with_network(&self, peers: Vec<String>) {
        let state = State { cid: uuid::Uuid::new_v4().to_string(), encrypted: format!("data_{}", chrono::Utc::now()) };
        let churn = rand::random::<f64>();
        if CosmicGossipProtocol::new().propagate_state(&state, &peers, churn).await {
            self.state_manager.save_state(&state);
            self.redis.cache_state(&state).await;
            self.peers = peers;
            self.active_nodes.store(self.peers.len(), Ordering::Relaxed);
        }
    }
}

impl Clone for SelfEvolvingFractalGossipNode {
    fn clone(&self) -> Self {
        Self { peer_id: self.peer_id.clone(), entropy: AtomicUsize::new(self.entropy.load(Ordering::Relaxed)), active_nodes: AtomicUsize::new(self.active_nodes.load(Ordering::Relaxed)), chaos_history: self.chaos_history.clone(), tensor_engine: self.tensor_engine.clone(), emotional_state_model: self.emotional_state_model.clone(), cross_modal_engine: self.cross_modal_engine.clone(), nli: self.nli.clone(), state_manager: self.state_manager.clone(), redis: self.redis.clone(), peers: self.peers.clone() }
}
