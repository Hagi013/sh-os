#![feature(const_fn)]

use core::ops::Deref;
use core::ptr::NonNull;
use core::alloc::{ AllocRef as Alloc, AllocError as AllocErr, Layout, GlobalAlloc };

use super::spin::mutex::Mutex;

pub mod slab;

use self::slab::Slab;
use core::cell::RefCell;

pub mod linked_list_allocator;
pub mod frame_allocator;

pub const NUM_OF_SLABS: usize = 8;
pub const MIN_SLAB_SIZE: usize = 4096;
pub const MIN_HEAP_SIZE: usize = NUM_OF_SLABS * MIN_SLAB_SIZE;


#[derive(Copy, Clone)]
pub enum HeapAllocator {
    Slab64Bytes,
    Slab128Bytes,
    Slab256Bytes,
    Slab512Bytes,
    Slab1024Bytes,
    Slab2048Bytes,
    Slab4096Bytes,
    LinkedListAllocator,
}

pub struct Heap {
    slab_64_bytes: RefCell<Slab>,
    slab_128_bytes: RefCell<Slab>,
    slab_256_bytes: RefCell<Slab>,
    slab_512_bytes: RefCell<Slab>,
    slab_1024_bytes: RefCell<Slab>,
    slab_2048_bytes: RefCell<Slab>,
    slab_4096_bytes: RefCell<Slab>,
    linked_list_allocator: RefCell<linked_list_allocator::Heap>,
}

impl Heap {
    pub unsafe fn new(heap_start_addr: usize, heap_size: usize) -> Self {
        assert_eq!(
            heap_start_addr % 4096, 0,
            "Start address should be page aligned"
        );
        assert!(
            heap_size >= MIN_HEAP_SIZE,
            "Heap size should be greater or equal to minimum heap size"
        );
        assert_eq!(
            heap_size % MIN_HEAP_SIZE, 0,
            "Heap size should be a multiple of minimum heap size"
        );
        let slab_size: usize = heap_size / NUM_OF_SLABS;
        Heap {
            slab_64_bytes: RefCell::new(Slab::new(heap_start_addr, slab_size, 64)),
            slab_128_bytes: RefCell::new(Slab::new(heap_start_addr + slab_size, slab_size, 128)),
            slab_256_bytes: RefCell::new(Slab::new(heap_start_addr + 2 * slab_size, slab_size, 256)),
            slab_512_bytes: RefCell::new(Slab::new(heap_start_addr + 3 * slab_size, slab_size, 512)),
            slab_1024_bytes: RefCell::new(Slab::new(heap_start_addr + 4 * slab_size, slab_size, 1024)),
            slab_2048_bytes: RefCell::new(Slab::new(heap_start_addr + 5 * slab_size, slab_size, 2048)),
            slab_4096_bytes: RefCell::new(Slab::new(heap_start_addr + 6 * slab_size, slab_size, 4096)),
            linked_list_allocator: RefCell::new(linked_list_allocator::Heap::new(heap_start_addr + 7 * slab_size, slab_size)),
        }
    }

    pub unsafe fn grow(&self, mem_start_addr: usize, mem_size: usize, slab: HeapAllocator) {
        match slab {
            HeapAllocator::Slab64Bytes => self.slab_64_bytes.borrow_mut().grow(mem_start_addr, mem_size),
            HeapAllocator::Slab128Bytes => self.slab_128_bytes.borrow_mut().grow(mem_start_addr, mem_size),
            HeapAllocator::Slab256Bytes => self.slab_256_bytes.borrow_mut().grow(mem_start_addr, mem_size),
            HeapAllocator::Slab512Bytes => self.slab_512_bytes.borrow_mut().grow(mem_start_addr, mem_size),
            HeapAllocator::Slab1024Bytes => self.slab_1024_bytes.borrow_mut().grow(mem_start_addr, mem_size),
            HeapAllocator::Slab2048Bytes => self.slab_2048_bytes.borrow_mut().grow(mem_start_addr, mem_size),
            HeapAllocator::Slab4096Bytes => self.slab_4096_bytes.borrow_mut().grow(mem_start_addr, mem_size),
            HeapAllocator::LinkedListAllocator => self.linked_list_allocator.borrow_mut().extend(mem_size),
        }
    }

    pub fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocErr> {
        match Heap::layout_to_allocator(&layout) {
            HeapAllocator::Slab64Bytes => self.slab_64_bytes.borrow_mut().allocate(layout),
            HeapAllocator::Slab128Bytes => self.slab_128_bytes.borrow_mut().allocate(layout),
            HeapAllocator::Slab256Bytes => self.slab_256_bytes.borrow_mut().allocate(layout),
            HeapAllocator::Slab512Bytes => self.slab_512_bytes.borrow_mut().allocate(layout),
            HeapAllocator::Slab1024Bytes => self.slab_1024_bytes.borrow_mut().allocate(layout),
            HeapAllocator::Slab2048Bytes => self.slab_2048_bytes.borrow_mut().allocate(layout),
            HeapAllocator::Slab4096Bytes => self.slab_4096_bytes.borrow_mut().allocate(layout),
            HeapAllocator::LinkedListAllocator => self.linked_list_allocator.borrow_mut().allocate_first_fit(layout),
        }
    }

    pub unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        match Heap::layout_to_allocator(&layout) {
            HeapAllocator::Slab64Bytes => self.slab_64_bytes.borrow_mut().deallocate(ptr),
            HeapAllocator::Slab128Bytes => self.slab_128_bytes.borrow_mut().deallocate(ptr),
            HeapAllocator::Slab256Bytes => self.slab_256_bytes.borrow_mut().deallocate(ptr),
            HeapAllocator::Slab512Bytes => self.slab_512_bytes.borrow_mut().deallocate(ptr),
            HeapAllocator::Slab1024Bytes => self.slab_1024_bytes.borrow_mut().deallocate(ptr),
            HeapAllocator::Slab2048Bytes => self.slab_2048_bytes.borrow_mut().deallocate(ptr),
            HeapAllocator::Slab4096Bytes => self.slab_4096_bytes.borrow_mut().deallocate(ptr),
            HeapAllocator::LinkedListAllocator => self.linked_list_allocator.borrow_mut().deallocate(ptr, layout),
        }
    }

    pub fn usable_size(&self, layout: &Layout) -> (usize, usize) {
        match Heap::layout_to_allocator(&layout) {
            HeapAllocator::Slab64Bytes => (layout.size(), 64),
            HeapAllocator::Slab128Bytes => (layout.size(), 128),
            HeapAllocator::Slab256Bytes => (layout.size(), 256),
            HeapAllocator::Slab512Bytes => (layout.size(), 512),
            HeapAllocator::Slab1024Bytes => (layout.size(), 1024),
            HeapAllocator::Slab2048Bytes => (layout.size(), 2048),
            HeapAllocator::Slab4096Bytes => (layout.size(), 4096),
            HeapAllocator::LinkedListAllocator => (layout.size(), layout.size()),
        }
    }

    pub fn layout_to_allocator(layout: &Layout) -> HeapAllocator {
        if layout.size() > 4096 {
            HeapAllocator::LinkedListAllocator
        } else if layout.size() <= 64 && layout.align() <= 64 {
            HeapAllocator::Slab64Bytes
        } else if layout.size() <= 128 && layout.align() <= 128 {
            HeapAllocator::Slab128Bytes
        } else if layout.size() <= 256 && layout.align() <= 256 {
            HeapAllocator::Slab128Bytes
        } else if layout.size() <= 512 && layout.align() <= 512 {
            HeapAllocator::Slab512Bytes
        } else if layout.size() <= 1024 && layout.align() <= 1024 {
            HeapAllocator::Slab1024Bytes
        } else if layout.size() <= 2048 && layout.align() <= 2048 {
            HeapAllocator::Slab2048Bytes
        } else if layout.size() <= 4096 && layout.align() <= 4096 {
            HeapAllocator::Slab4096Bytes
        } else {
            HeapAllocator::Slab4096Bytes
        }
    }
}

unsafe impl Alloc for Heap {
    fn alloc(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocErr> {
        self.allocate(layout)
    }

    unsafe fn dealloc(&self, ptr: NonNull<u8>, layout: Layout) {
        self.deallocate(ptr, layout);
    }

//    fn oom(&mut self, err: AllocErr) -> ! {
//        panic!("Out of memory: {:?}", err);
//    }

    // fn usable_size(&self, layout: &Layout) -> (usize, usize) {
    //     self.usable_size(layout)
    // }
}

static HEAP: Mutex<Option<Heap>> = Mutex::new(None);
// static HEAP: Mutex<Option<Heap>> = LockedHeap::empty();

pub struct LockedHeap;

impl LockedHeap {
//    #[cfg(feature = "min_const_fn")]
//    pub const fn empty() -> Mutex<Option<Heap>> {
////        LockedHeap(Mutex::new(None))
//        Mutex::new(None)
//    }

    pub fn init(&self, heap_addr_start: usize, size: usize) {
        *HEAP.lock() = unsafe { Some(Heap::new(heap_addr_start, size)) };
    }

//    pub unsafe fn new(heap_addr_start: usize, heap_size: usize) -> Self {
//        LockedHeap(Mutex::new(Some(Heap::new(heap_addr_start, heap_size))))
//    }
}

//impl Deref for LockedHeap {
//    type Target = Mutex<Option<Heap>>;
//    fn deref(&self) -> &Mutex<Option<Heap>> {
//        // &self.0
//        *HEAP
//    }
//}

unsafe impl<'a> Alloc for &'a LockedHeap {
    fn alloc(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocErr> {
//        if let Some(ref mut heap) = *self.0.lock() {
//    unsafe fn alloc(&self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
        if let Some(ref mut heap) = *HEAP.lock() {
            heap.allocate(layout)
        } else {
            panic!("allocate: heap not initialized");
        }
    }

    unsafe fn dealloc(&self, ptr: NonNull<u8>, layout: Layout) {
//        if let Some(ref mut heap) = *self.0.lock() {
//    unsafe fn dealloc(&self, ptr: NonNull<u8>, layout: Layout) {
        if let Some(ref mut heap) = *HEAP.lock() {
            heap.deallocate(ptr, layout)
        } else {
            panic!("deallocate: heap not initialized");
        }
    }

//     fn usable_size(&self, layout: &Layout) -> (usize, usize) {
// //        if let Some(ref mut heap) = *self.0.lock() {
//         if let Some(ref mut heap) = *HEAP.lock() {
//             heap.usable_size(layout)
//         } else {
//             panic!("usable_size: heap not initialized");
//         }
//     }
}

unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//        if let Some(ref mut heap) = *self.0.lock() {
        if let Some(ref mut heap) = *HEAP.lock() {
            if let Ok(ref mut nnptr) = heap.allocate(layout) {
                return nnptr.as_ptr() as *mut u8;
            } else {
                panic!("allocate: failed");
            }
        } else {
            panic!("allocate: heap not initialized");
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
//        if let Some(ref mut heap) = *self.0.lock() {
        if let Some(ref mut heap) = *HEAP.lock() {
            if let Some(p) = NonNull::new(ptr) {
                heap.deallocate(p, layout)
            }
        } else {
            panic!("dealloc: heap not initialized");
        }
    }
}

