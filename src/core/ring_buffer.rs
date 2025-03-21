#[derive(Clone)]
pub struct RingBuffer<T> {
    array: Vec<Option<T>>,
    head: usize,
    capacity: usize,
}

impl<T: Clone> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            array: vec![None; capacity],
            head: 0,
            capacity,
        }
    }

    pub fn append(&mut self, item: T) {
        self.array[self.head % self.capacity] = Some(item);
        self.head += 1;
    }

    pub fn get_latest(&self) -> Option<&T> {
        self.array[(self.head.wrapping_sub(1) % self.capacity) as usize].as_ref()
    }
}
