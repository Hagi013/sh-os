use core::ops::Deref;
use alloc::alloc::{ Alloc, AllocErr, Layout };

pub mod slab;

use slab::Slab;
pub const NUM_OF_SLABS: usize = 8;
pub const MIN_SLAB_SIZE: usize = 4096;
pub const MIN_HEAP_SIZE: usize = NUM_OF_SLABS * MIN_SLAB_SIZE;

pub enum HeapAllocator {
    Slab64Bytes,
    Slab128Bytes,
    Slab256Bytes,
    Slab512Bytes,
    Slab1024Bytes,
    Slab2048Bytes,
    Slab4096Bytes,
}

pub struct Heap {
    slab_64_bytes: Slab,
    slab_128_bytes: Slab,
    slab_256_bytes: Slab,
    slab_512_bytes: Slab,
    slab_1024_bytes: Slab,
    slab_2048_bytes: Slab,
    slab_4096_bytes: Slab,
}

impl Heap {
    pub unsafe fn new(heap_start_addr: usize, heap_size: usize) -> Self {
//        assert!(
//            heap_start_addr % 4096 == 0,
//            "Start address should be page aligned"
//        );
        assert_eq!(
            heap_start_addr % 4096, 0,
            "Start address should be page aligned"
        );
        assert!(
            heap_size >= MIN_HEAP_SIZE,
            "Heap size should be greater or equal to minimum heap size"
        );
//        assert!(
//            heap_size % MIN_HEAP_SIZE == 0,
//            "Heap size should be a multiple of minimum heap size"
//        );
        assert_eq!(
            heap_size % MIN_HEAP_SIZE, 0,
            "Heap size should be a multiple of minimum heap size"
        );

        let slab_size: usize = heap_size / NUM_OF_SLABS;
        Heap {
            slab_64_bytes: Slab::new(heap_start_addr, slab_size, 64),
            slab_128_bytes: Slab::new(heap_start_addr + slab_size, slab_size, 128),
            slab_256_bytes: Slab::new(heap_start_addr + 2 * slab_size, slab_size, 256),
            slab_512_bytes: Slab::new(heap_start_addr + 3 * slab_size, slab_size, 512),
            slab_1024_bytes: Slab::new(heap_start_addr + 4 * slab_size, slab_size, 1024),
            slab_2048_bytes: Slab::new(heap_start_addr + 5 * slab_size, slab_size, 2048),
            slab_4096_bytes: Slab::new(heap_start_addr + 6 * slab_size, slab_size, 4096),
        }
    }

    pub unsafe fn grow(&mut self, mem_start_addr: usize, mem_size: usize, slab: HeapAllocator) {
        match slab {
            HeapAllocator::Slab64Bytes => self.slab_64_bytes.grow(mem_start_addr. mem_size),
            HeapAllocator::Slab128Bytes => self.slab_128_bytes.grow(mem_start_addr. mem_size),
            HeapAllocator::Slab256Bytes => self.slab_256_bytes.grow(mem_start_addr. mem_size),
            HeapAllocator::Slab512Bytes => self.slab_512_bytes.grow(mem_start_addr. mem_size),
            HeapAllocator::Slab1024Bytes => self.slab_1024_bytes.grow(mem_start_addr. mem_size),
            HeapAllocator::Slab2048Bytes => self.slab_2048_bytes.grow(mem_start_addr. mem_size),
            HeapAllocator::Slab4096Bytes => self.slab_4096_bytes.grow(mem_start_addr. mem_size),
        }
    }

    pub fn allocate(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        match Heap::layout_to_allocator(&layout) {
            HeapAllocator::Slab64Bytes => self.slab_64_bytes.allocate(layout),
            HeapAllocator::Slab128Bytes => self.slab_128_bytes.allocate(layout),
            HeapAllocator::Slab256Bytes => self.slab_256_bytes.allocate(layout),
            HeapAllocator::Slab512Bytes => self.slab_512_bytes.allocate(layout),
        }
    }

    pub unsafe fn deallocate(&mut self, ptr: *mut u8, layout: Layout) {
        match Heap::layout_to_allocator(&layout) {
            HeapAllocator::Slab64Bytes => self.slab_64_bytes.deallocate(ptr),
            HeapAllocator::Slab128Bytes => self.slab_128_bytes.deallocate(ptr),
            HeapAllocator::Slab256Bytes => self.slab_256_bytes.deallocate(ptr),
            HeapAllocator::Slab512Bytes => self.slab_512_bytes.deallocate(ptr),
            HeapAllocator::Slab1024Bytes => self.slab_1024_bytes.deallocate(ptr),
            HeapAllocator::Slab2048Bytes => self.slab_2048_bytes.deallocate(ptr),
            HeapAllocator::Slab4096Bytes => self.slab_4096_bytes.deallocate(ptr),
        }
    }

    pub fn usable_size(&mut self, layout: &Layout) -> (usize, usize) {
        match Heap::layout_to_allocator(&layout) {
            HeapAllocator::Slab64Bytes => (layout.size(), 64),
            HeapAllocator::Slab128Bytes => (layout.size(), 128),
            HeapAllocator::Slab256Bytes => (layout.size(), 256),
            HeapAllocator::Slab512Bytes => (layout.size(), 512),
            HeapAllocator::Slab1024Bytes => (layout.size(), 1024),
            HeapAllocator::Slab2048Bytes => (layout.size(), 2048),
            HeapAllocator::Slab4096Bytes => (layout.size(), 4096),
        }
    }

    pub fn layout_to_allocator(layout: &Layout) -> HeapAllocator {
        if layout.size() > 4096 {
            HeapAllocator::Slab4096Bytes
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
        }
    }
}

unsafe impl Alloc for Heap {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        self.allocate(layout)
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        self.deallocate(ptr, layout);
    }

    fn oom(&mut self, err: AllocErr) -> ! {
        panic!("Out of memory: {:?}", err);
    }

    fn usable_size(&mut self, lauout: &Layout) -> (usize, usize) {
        self.usable_size(layout)
    }
}

