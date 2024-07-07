use std::alloc::{alloc, dealloc, realloc, Layout};
use std::mem::size_of;

struct Stack {
    layout: Layout,
    ptr: *mut u8,
}

impl Stack {
    pub fn new() -> Option<Stack> {
        let layout = Layout::new::<usize>();
        let ptr = unsafe {
            let ptr = alloc(layout);
            if ptr.is_null() {
                return None;
            }
            *(ptr as *mut usize) = size_of::<usize>();
            ptr
        };
        Some(Stack { layout, ptr })
    }
    pub fn push<T: Sized>(&mut self, item: T) -> bool {
        let layout = Layout::new::<T>();
        let res = self.layout.extend(layout);
        if let Err(_) = res { return false; }
        let len = {
            let (layout, len) = res.unwrap();
            self.layout = layout;
            len
        };
        unsafe {
            self.ptr = realloc(self.ptr, self.layout, len);
            if self.ptr.is_null() {
                return false;
            }
            let ptr = self.ptr as *mut usize;
            *ptr = len;
            *(ptr.offset(1) as *mut T) = item;
        };
        return true;
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        unsafe { dealloc(self.ptr, self.layout) }
    }
}

