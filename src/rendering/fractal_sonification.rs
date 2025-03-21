use std::collections::VecDeque;
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct FractalSonification {
    queue: VecDeque<String>,
}

impl FractalSonification {
    pub fn new() -> Self { Self { queue: VecDeque::new() } }

    pub fn sonify(&self, coherence: f64) {
        if self.queue.len() < 2 {
            let message = format!("Cosmic hum: {}", coherence);
            self.queue.push_back(message.clone());
            println!("{}", message); // Simulated audio
            let queue = self.queue.clone();
            tokio::spawn(async move {
                sleep(Duration::from_millis(500)).await;
                queue.pop_front();
            });
        }
    }
}
