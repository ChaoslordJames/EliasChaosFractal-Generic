#[tokio::test]
async fn test_3d_rendering() {
    let engine = crate::rendering::cross_modal_cosmic_engine::CrossModalCosmicEngine::new();
    let tensor = crate::quantum::quantum_fractal_tensor_engine::QuantumFractalTensorEngine::new();
    let (xy, xz, yz) = engine.render_live_fractal(&tensor).await;
    assert!(xy.width() > 0 && xz.width() > 0 && yz.width() > 0);
}
