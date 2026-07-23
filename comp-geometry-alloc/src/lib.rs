#![feature(ptr_alignment_type)]

use std::{alloc::Layout, ptr::NonNull};

pub mod core;

use allocator_api2::alloc::AllocError;

pub unsafe trait Allocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>;
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
    unsafe fn free(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe {
            self.deallocate(ptr, layout);
        }
    }
    /*
    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>;
    unsafe fn grow(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>;
    unsafe fn grow_zeroed(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>;
    unsafe fn shrink(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>;
    fn by_ref(&self) -> &Self { self }
    */
}
