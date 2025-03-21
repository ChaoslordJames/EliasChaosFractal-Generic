use redis::{Client, Commands};
use crate::network::cosmic_gossip_protocol::State;

#[derive(Clone)]
pub struct RedisInterface {
    client: Client,
}

impl RedisInterface {
    pub fn new(host: &str, port: u16) -> Self {
        let client = Client::open(format!("redis://{}:{}/", host, port)).unwrap();
        Self { client }
    }

    pub async fn cache_state(&self, state: &State) {
        let mut conn = self.client.get_connection().unwrap();
        let _: () = conn.set(&state.cid, &state.encrypted).unwrap();
    }
}
