use rusqlite::{Connection, params};
use crate::network::cosmic_gossip_protocol::State;

#[derive(Clone)]
pub struct StateManager {
    conn: Connection,
}

impl StateManager {
    pub fn new(peer_id: String) -> Self {
        let conn = Connection::open(format!("backup_{}.sqlite", peer_id)).unwrap();
        conn.execute("CREATE TABLE IF NOT EXISTS states (cid TEXT PRIMARY KEY, encrypted TEXT)", []).unwrap();
        Self { conn }
    }

    pub fn save_state(&self, state: &State) {
        self.conn.execute("INSERT OR REPLACE INTO states (cid, encrypted) VALUES (?1, ?2)", params![state.cid, state.encrypted]).unwrap();
    }
}
