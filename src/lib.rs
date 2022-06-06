#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type PQueueU8;

        // Freestanding function
        fn pqueue_u8_new(elements: &[u8]) -> Box<PQueueU8>;

        // Methods
        fn push(self: &mut PQueueU8, element: u8);
        fn pop(self: &mut PQueueU8) -> Result<u8>;
    }
}

pub struct PQueueU8(std::collections::BinaryHeap<u8>);

fn pqueue_u8_new(elements: &[u8]) -> Box<PQueueU8> {
    let mut pqueue = PQueueU8(std::collections::BinaryHeap::new());

    for &element in elements {
        pqueue.0.push(element);
    }

    Box::new(pqueue)
}

impl PQueueU8 {
    fn pop(&mut self) -> Result<u8, &'static str> {
        self.0.pop().ok_or("queue is empty")
    }

    fn push(&mut self, element: u8) {
        self.0.push(element);
    }
}
