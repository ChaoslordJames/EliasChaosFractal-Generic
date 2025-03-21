#[tokio::test]
async fn test_recursive_depth() {
    let node = crate::core::self_evolving_fractal_gossip_node::SelfEvolvingFractalGossipNode::new("test".to_string()).await.unwrap();
    let mut model = crate::core::self_model::SelfModel::new();
    model.update_self(&node);
    assert!(model.get_recursive_depth() > 0.0);
}
