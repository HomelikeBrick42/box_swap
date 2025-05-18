#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use core::{
    marker::PhantomData,
    sync::atomic::{AtomicPtr, Ordering},
};

/// An atomic version of [`Option<Box<T>>`]
pub struct BoxSwap<T> {
    ptr: AtomicPtr<T>,
    _data: PhantomData<Box<T>>,
}

unsafe impl<T> Send for BoxSwap<T> where Box<T>: Send {}
unsafe impl<T> Sync for BoxSwap<T> where Box<T>: Send {}

impl<T> BoxSwap<T> {
    /// Constructs a [`BoxSwap`] containing [`None`]
    pub const fn empty() -> Self {
        Self {
            ptr: AtomicPtr::new(core::ptr::null_mut()),
            _data: PhantomData,
        }
    }

    /// Constructs a [`BoxSwap`] containing [`Some(value)`](Some)
    pub fn with_value(value: Box<T>) -> Self {
        Self {
            ptr: AtomicPtr::new(Box::into_raw(value)),
            _data: PhantomData,
        }
    }

    /// Atomically stores `value`, returning the previous value
    pub fn swap(&self, value: Option<Box<T>>) -> Option<Box<T>> {
        let new = value.map(Box::into_raw).unwrap_or_else(core::ptr::null_mut);

        let old = self.ptr.swap(new, Ordering::AcqRel);
        if old.is_null() {
            None
        } else {
            Some(unsafe { Box::from_raw(old) })
        }
    }

    /// Atomically stores [`Some(value)`](Some), dropping the previous value
    pub fn store(&self, value: Box<T>) {
        drop(self.swap(Some(value)));
    }

    /// Atomically takes the value, leaving [`None`] in its place
    pub fn take(&self) -> Option<Box<T>> {
        self.swap(None)
    }

    /// Consumes the [`BoxSwap`], returning the wrapped value
    pub fn into_inner(mut self) -> Option<Box<T>> {
        self.take_mut()
    }

    /// Gets a mutable reference to the value
    pub fn get_mut(&mut self) -> Option<&mut T> {
        let ptr = *self.ptr.get_mut();
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &mut *ptr })
        }
    }

    /// [`BoxSwap::swap`] but without any atomic operations
    pub fn swap_mut(&mut self, value: Option<Box<T>>) -> Option<Box<T>> {
        let new = value.map(Box::into_raw).unwrap_or_else(core::ptr::null_mut);

        let old = core::mem::replace(self.ptr.get_mut(), new);
        if old.is_null() {
            None
        } else {
            Some(unsafe { Box::from_raw(old) })
        }
    }

    /// [`BoxSwap::store`] but without any atomic operations
    pub fn store_mut(&mut self, value: Box<T>) {
        drop(self.swap_mut(Some(value)));
    }

    /// [`BoxSwap::take`] but without any atomic operations
    pub fn take_mut(&mut self) -> Option<Box<T>> {
        self.swap_mut(None)
    }
}

impl<T> Drop for BoxSwap<T> {
    fn drop(&mut self) {
        drop(self.take_mut());
    }
}

impl<T> Default for BoxSwap<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> From<Box<T>> for BoxSwap<T> {
    fn from(value: Box<T>) -> Self {
        Self::with_value(value)
    }
}

impl<T> From<Option<Box<T>>> for BoxSwap<T> {
    fn from(value: Option<Box<T>>) -> Self {
        value.map(Self::with_value).unwrap_or_else(Self::empty)
    }
}
