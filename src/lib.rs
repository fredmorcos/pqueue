pub struct PQueueU8(std::collections::BinaryHeap<u8>);

#[no_mangle]
pub extern "C"
fn pqueue_u8_new(elements: Option<&u8>, len: usize) -> Option<Box<PQueueU8>> {
    let mut pqueue = PQueueU8(std::collections::BinaryHeap::new());

    let elements: &u8 = elements?;
    let elements: &[u8] = unsafe { std::slice::from_raw_parts(elements, len) };
    for &element in elements {
        pqueue.0.push(element);
    }

    Some(Box::new(pqueue))
}

#[repr(C)]
pub enum PQueueU8Status {
    Success = 0,
    Empty = 1,
    InvalidArgument = -1,
}

#[repr(C)]
pub struct PQueueU8Value {
    status: PQueueU8Status,
    value: u8,
}

#[no_mangle]
pub extern "C"
fn pqueue_u8_pop(pqueue: Option<&mut PQueueU8>) -> PQueueU8Value {
    if let Some(pqueue) = pqueue {
        if let Some(value) = pqueue.0.pop() {
            PQueueU8Value { status: PQueueU8Status::Success, value }
        } else {
            PQueueU8Value { status: PQueueU8Status::Empty, value: 0 }
        }
    } else {
        PQueueU8Value { status: PQueueU8Status::InvalidArgument, value: 0 }
    }
}

#[no_mangle]
pub extern "C"
fn pqueue_u8_push(pqueue: Option<&mut PQueueU8>, element: u8) -> PQueueU8Status {
    if let Some(pqueue) = pqueue {
        pqueue.0.push(element);
        PQueueU8Status::Success
    } else {
        PQueueU8Status::InvalidArgument
    }
}

#[no_mangle]
pub extern "C"
fn pqueue_u8_free(_pqueue: Option<Box<PQueueU8>>) {}
