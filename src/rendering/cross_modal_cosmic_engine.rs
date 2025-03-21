use crate::quantum::quantum_fractal_tensor_engine::QuantumFractalTensorEngine;
use crate::core::emotional_state_model::EmotionalStateModel;
use crate::rendering::fractal_visualization::FractalVisualization;
use crate::rendering::fractal_sonification::FractalSonification;
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct CrossModalCosmicEngine {
    visualization: FractalVisualization,
    sonification: FractalSonification,
    last_entropy: f64,
}

impl CrossModalCosmicEngine {
    pub fn new() -> Self {
        Self {
            visualization: FractalVisualization::new(),
            sonification: FractalSonification::new(),
            last_entropy: 0.0,
        }
    }

    pub async fn render_live_fractal(&self, tensor_engine: &QuantumFractalTensorEngine) -> (image::DynamicImage, image::DynamicImage, image::DynamicImage) {
        self.last_entropy = tensor_engine.cosmic_entropy;
        let delay = Duration::from_secs_f64((1.0f64).max(self.last_entropy / 20000.0));
        sleep(delay).await;
        self.visualization.render_3d(&tensor_engine.tensor_field)
    }

    pub async fn render_live_soundscape(&self, emotional_model: &EmotionalStateModel) {
        let coherence = *emotional_model.emotional_dimensions.get("cosmic_resonance").unwrap_or(&0.0);
        self.sonification.sonify(coherence);
    }
}
