use crate::alloc::Allocator;
use core::ptr::NonNull;

pub struct Vec<'alloc, T> {
    alloc: &'alloc dyn Allocator,
    data: NonNull<T>,
}

impl<'alloc, T> Vec<'alloc, T> {

}
