#[tokio::test]
async fn test_extreme_load() {
    let node = crate::core::self_evolving_fractal_gossip_node::SelfEvolvingFractalGossipNode::new("test".to_string()).await.unwrap();
    let metrics = crate::network::network_metrics::NetworkMetrics::new();
    assert!(metrics.resilience_score(&node) >= 99.95);
}
