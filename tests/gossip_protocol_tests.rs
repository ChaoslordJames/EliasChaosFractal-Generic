#[tokio::test]
async fn test_state_propagation() {
    let gossip = crate::network::cosmic_gossip_protocol::CosmicGossipProtocol::new();
    let state = crate::network::cosmic_gossip_protocol::State { cid: "1".to_string(), encrypted: "data".to_string() };
    let peers = (0..1000).map(|i| format!("p{}", i)).collect::<Vec<_>>();
    let success = gossip.propagate_state(&state, &peers, 0.5).await;
    assert!(success);
}
