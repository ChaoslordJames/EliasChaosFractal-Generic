#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct State {
    pub cid: String,
    pub encrypted: String,
}

#[derive(Clone)]
pub struct CosmicGossipProtocol;

impl CosmicGossipProtocol {
    pub fn new() -> Self { Self }

    pub async fn propagate_state(&self, state: &State, peers: &[String], churn: f64) -> bool {
        let replication_factor = 16 + (churn * 100.0 * 0.09) as usize;
        let successful_peers = peers.iter().filter(|_| rand::random::<f64>() > churn).count();
        successful_peers >= (replication_factor as f64 * 0.8) as usize
    }
}
