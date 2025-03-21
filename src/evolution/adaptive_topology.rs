#[derive(Clone)]
pub struct AdaptiveTopology {
    k_buckets: Vec<KBucket>,
}

impl AdaptiveTopology {
    pub fn new() -> Self { Self { k_buckets: Vec::new() } }

    pub fn optimize_for_peers(&mut self, count: usize) -> Vec<KBucket> {
        let bucket_count = std::cmp::max(count / 2000, 2500);
        self.k_buckets = (0..bucket_count).map(|i| KBucket { distance: i, k: 4000, peers: Vec::new() }).collect();
        self.k_buckets.clone()
    }

    pub fn get_buckets(&self) -> &[KBucket] { &self.k_buckets }
}

#[derive(Clone)]
pub struct KBucket {
    distance: usize,
    k: usize,
    peers: Vec<String>,
}
