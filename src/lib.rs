#[cxx::bridge]
mod ffi {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum PQueueU8Status {
        Success = 0,
        Empty = 1,
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct PQueueU8Value {
        status: PQueueU8Status,
        value: u8,
    }

    extern "Rust" {
        type PQueueU8;

        // Freestanding function
        fn pqueue_u8_new(elements: &[u8]) -> Box<PQueueU8>;

        // Methods
        fn push(&mut self, element: u8);
        fn pop(&mut self) -> PQueueU8Value;
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

use ffi::{PQueueU8Value, PQueueU8Status};

impl PQueueU8 {
    fn push(&mut self, element: u8) {
        self.0.push(element);
    }

    fn pop(&mut self) -> PQueueU8Value {
        if let Some(value) = self.0.pop() {
            PQueueU8Value { status: PQueueU8Status::Success, value }
        } else {
            PQueueU8Value { status: PQueueU8Status::Empty, value: 0 }
        }
    }
}
