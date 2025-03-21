#[derive(Clone)]
pub struct PeerDiscovery;

impl PeerDiscovery {
    thread_local! {
        static LOCAL_NODES: std::cell::RefCell<Vec<String>> = std::cell::RefCell::new(Vec::new());
        static GLOBAL_PEERS: std::cell::RefCell<Vec<String>> = std::cell::RefCell::new(Vec::new());
    }

    pub fn register_node(peer_id: String) {
        Self::LOCAL_NODES.with(|nodes| nodes.borrow_mut().push(peer_id));
    }

    pub fn connect_global() {
        Self::GLOBAL_PEERS.with(|peers| {
            let mut peers = peers.borrow_mut();
            if peers.is_empty() {
                *peers = (0..5_000_000).map(|i| format!("global_{}", i)).collect();
            }
        });
    }

    pub async fn get_peers(node: &super::self_evolving_fractal_gossip_node::SelfEvolvingFractalGossipNode) -> Vec<String> {
        let local_count = std::cmp::min(node.active_nodes.load(Ordering::Relaxed) / 2, 500);
        let global_count = std::cmp::min(node.active_nodes.load(Ordering::Relaxed) / 2, 500);
        Self::LOCAL_NODES.with(|local| {
            let local_peers: Vec<_> = local.borrow().iter().filter(|p| *p != &node.peer_id).cloned().collect();
            Self::GLOBAL_PEERS.with(|global| {
                let mut peers = local_peers.into_iter().take(local_count).collect::<Vec<_>>();
                peers.extend(global.borrow().iter().take(global_count).cloned());
                peers
            })
        })
    }
}
