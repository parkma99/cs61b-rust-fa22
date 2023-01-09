use super::random::Random;
use crate::deques::{arraydeque::ArrayDeque, Deque};

#[derive(Default)]
pub struct GuitarString {
    note_attenuation: f32,
    deque: ArrayDeque<f32>,
}

impl GuitarString {
    pub fn new(sample_rate: u32, note_atten: f32, freq: u32) -> Self {
        let deque_len = sample_rate / freq;
        let mut deque = ArrayDeque::new();
        for _ in 0..deque_len {
            deque.add_last(0.0);
        }
        Self {
            note_attenuation: note_atten,
            deque,
        }
    }

    /* Pluck the guitar string by replacing the buffer with white noise. */
    pub fn pluck(&mut self, rand: &mut Random) {
        // example usage of rand
        for _ in 0..self.deque.len() {
            self.deque.remove_first();
            let val = (rand.next_f64() - 0.5) as f32;
            self.deque.add_last(val);
        }
    }

    /* Advance the simulation by performing one iteration of the Karplus-Strong algorithm:
        - pop the first sample off of the queue
        - calculate the next sample, and push it into the queue
        - return the popped sample
    */
    pub fn advance(&mut self) -> f32 {
        let first = self.deque.remove_first().unwrap();
        let second = self.deque.get_first().unwrap();
        let next = (first + second) / 2.0 * self.note_attenuation;
        self.deque.add_last(next);
        first
    }
}
