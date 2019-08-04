use alloc::collections::vec_deque::VecDeque;

use super::super::spin::mutex::Mutex;
//use alloc::string::String;
//use alloc::borrow::ToOwned;

// static HEAP: Mutex<Option<Heap>> = LockedHeap::empty();
//pub struct KQueue<T: ?Sized> {
//    k_queue: Mutex<Queue<T>>,
//}
//
//impl<T> KQueue<T> {
//    pub fn new() -> KQueue<T> {
//        Self {
//            k_queue: Mutex::new(Queue::new())
//        }
//        // *HEAP.lock() = unsafe { Some(Heap::new(heap_addr_start, size)) };
//    }
//}

//#[derive(Debug)]
//pub struct KQueue<T> {
//    k_queue: Mutex<VecDeque<T>>,
//}
//
//impl<T> KQueue<T> {
//    pub fn new() -> KQueue<T> {
//        KQueue {
//            k_queue: Mutex::new(VecDeque::new()),
//        }
//    }
//
//    pub fn enqueue(&mut self, data: T) {
//        self.k_queue.lock().push_back(data);
//    }
//
//    pub fn dequeue(&mut self) -> Option<T> {
//        self.k_queue.lock().pop_front()
//    }
//
//    pub fn len(&self) -> usize {
//        self.k_queue.lock().len()
//    }
//
//    pub fn is_empty(&self) -> bool {
//        self.k_queue.lock().is_empty()
//    }
//
//    pub fn is_existing(&self) -> bool {
//        !self.is_empty()
//    }
//
//    pub fn clear(&mut self) {
//        self.k_queue.lock().clear()
//    }
//}

const CAPACITY: usize = 30;
const INITIAL_INDEX: usize = 0;
const LIMIT: usize = CAPACITY - INITIAL_INDEX;

#[derive(Debug)]
pub struct SimpleQueue<T: Copy> {
    head: usize,
    tail: usize,
    count: usize,
    queue: [T; LIMIT],
}

impl<T: Copy> SimpleQueue<T> {
    pub fn new() -> Self {
        SimpleQueue {
            head: INITIAL_INDEX,
            tail: INITIAL_INDEX,
            count: 0,
            queue: [unsafe { *(0 as *mut T) }; LIMIT],
        }
    }

    pub fn enqueue(&mut self, data: T) {
        if self.len() == LIMIT { return; }

        self.count += 1;
        self.queue[self.tail] = data;

        if SimpleQueue::<T>::check_index_limit(self.tail) {
            self.tail += 1;
        } else {
            self.tail = INITIAL_INDEX;
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.len() <= 0 { return None }

        self.count -= 1;
        let data = Some(self.queue[self.head]);

        if SimpleQueue::<T>::check_index_limit(self.head) {
            self.head += 1;
        } else {
            self.head = INITIAL_INDEX;
        }
        data
    }

    pub fn len(&self) -> usize {
        self.count
    }

    fn check_index_limit(index: usize) -> bool {
        index < LIMIT - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_existing(&self) -> bool {
        self.len() > 0
    }

    pub fn clear(&mut self) {
        self.queue = [unsafe { *(0 as *mut T) }; LIMIT];
        self.head = INITIAL_INDEX;
        self.tail = INITIAL_INDEX;
        self.count = 0;
    }
}
