use core::ptr::NonNull;
use core::alloc::Layout;

pub mod bump_alloc;
pub mod local_alloc;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    OutOfMemory,
}

/// Safety: If an instace of `Allocator` is moved, the currently allocated memory section pointers
/// can not get invalidated.
/// So it is invalid to implement `Allocator` on a type like `struct MyAlloc([u8; 1024])`
pub unsafe trait Allocator {
    /// This function should never panic.
    ///
    /// Layout size cannot be zero.
    ///
    /// Layout alignment cannot be larger than 4KB.
    fn alloc(&self, layout: Layout) -> Result<NonNull<u8>, Error>;

    /// This function should never panic.
    ///
    /// Safety: `ptr` must be a currently valid pointer obtained by calling `alloc` on the same `Allocator`.
    /// Layout argument passed to that `alloc` call or the last `resize` call made with the same
    /// `ptr`.
    unsafe fn free(&self, ptr: NonNull<u8>, layout: Layout) -> Result<(), Error>;

    /// size of `new_layout` cannot be zero.
    ///
    /// This function should never panic.
    ///
    /// Returns true if resizing succeeded, false otherwise.
    ///
    /// Safety: `ptr` must be a currently valid pointer obtained by calling `alloc` on the same `Allocator`.
    /// Layout argument passed to that `alloc` call or the last `resize` call made with the same
    /// `ptr`.
    unsafe fn resize(&self, ptr: NonNull<u8>, layout: Layout, new_layout: Layout) -> Result<bool, Error>;
}

/// Safety: If an instace of `PageAllocator` is moved, the currently allocated page pointers
/// can not get invalidated.
/// So it is invalid to implement `PageAllocator` on a type like `struct MyAlloc([u8; 1024])`
pub unsafe trait PageAllocator {
    /// This function should never panic.
    ///
    /// Might return a bigger sized allocation than size given in argument.
    ///
    /// Alignment of the returned page is at least 4KB.
    fn alloc_page(&self, size: usize) -> Result<*mut [u8], Error>;
    
    /// This function should never panic.
    ///
    /// Safety: `ptr` must have been returned by `alloc_page` on the same object.
    /// user can only call this function once on a pointer.
    unsafe fn free_page(&self, ptr: *mut [u8]) -> Result<(), Error>;
}
