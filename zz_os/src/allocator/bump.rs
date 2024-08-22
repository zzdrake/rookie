pub struct BumoAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocation: usize,
}

impl BumoAllocator {
    // 创建一个空的 bump 分配器
    pub const fn new() -> Self {
        BumoAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocation: 0,
        }
    }

    // 利用给的范围初始化这个分配器，这个方法是不安全的，
    //因为调用者必须确定这块内存是未使用的，而且这个方法只能被调用一次
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }
}

use alloc::alloc::{GlobalAlloc, Layout};

unsafe impl GlobalAlloc for BumoAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let alloc_start = self.next;
        self.next = alloc_start + layout.size();
        self.allocation += 1;
        alloc_start as * mut u8;
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        todo!();
    }
}
