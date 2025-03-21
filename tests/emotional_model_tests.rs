#[test]
fn test_cosmic_feedback() {
    let mut model = crate::core::emotional_state_model::EmotionalStateModel::new();
    let tensor = crate::quantum::quantum_fractal_tensor_engine::QuantumFractalTensorEngine::new();
    model.adjust_with_cosmic_feedback(&tensor);
    assert!(*model.emotional_dimensions.get("cosmic_resonance").unwrap() > 0.0);
}
