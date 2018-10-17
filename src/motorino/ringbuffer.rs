use std::collections::VecDeque;

pub struct RingBuffer<A> {
    deque: VecDeque<A>
}

impl<A> RingBuffer<A> {

    pub fn new<B>(capacity: usize) -> RingBuffer<B> {
        RingBuffer {
            deque: VecDeque::with_capacity(capacity)
        }
    }

    pub fn push(&mut self, value: A) {
        if self.deque.len() == self.deque.capacity() {
            self.deque.pop_front();
        }
        self.deque.push_back(value);
    }

    pub fn deque(&self) -> &VecDeque<A> {
        &self.deque
    }
}

impl<A> Default for RingBuffer<A> {
    fn default() -> Self {
        RingBuffer::<A>::new(10)
    }
}