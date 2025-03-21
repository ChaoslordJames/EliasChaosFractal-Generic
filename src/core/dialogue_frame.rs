#[derive(Clone)]
pub struct DialogueFrame {
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub depth: usize,
}
