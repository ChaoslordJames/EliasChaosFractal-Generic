#[tokio::test]
async fn test_field_update() {
    let node = crate::core::self_evolving_fractal_gossip_node::SelfEvolvingFractalGossipNode::new("test".to_string()).await.unwrap();
    let mut tensor = crate::quantum::quantum_fractal_tensor_engine::QuantumFractalTensorEngine::new();
    tensor.update_field(&node);
    assert!(tensor.cosmic_entropy > 0.0);
}
