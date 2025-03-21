#[tokio::test]
async fn test_node_initialization() {
    let node = crate::core::self_evolving_fractal_gossip_node::SelfEvolvingFractalGossipNode::new("test".to_string()).await;
    assert!(node.is_ok());
}

#[tokio::test]
async fn test_query_processing() {
    let node = crate::core::self_evolving_fractal_gossip_node::SelfEvolvingFractalGossipNode::new("test".to_string()).await.unwrap();
    let response = node.process_query("Hello".to_string()).await;
    assert!(response.contains("v4.4.1"));
}
