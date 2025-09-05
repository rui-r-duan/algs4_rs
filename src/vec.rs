//! A vector implementation inspired by the Vec implmenetation in [The
//! Rustonomicon](https://doc.rust-lang.org/nomicon/vec/vec.html).
//!
//! The nonicon version does not have the `shrink` allocation, while this implementation does.
//! **For the nitty-gritty, please read The Rustonomicon.**

use raw_vec::{RawValIter, RawVec};
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;

mod raw_vec;

pub struct Vec<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> Vec<T> {
    /// Create an empty `Vec` which does not allocate any memory.
    pub fn new() -> Self {
        Vec {
            buf: RawVec::new(),
            len: 0,
        }
    }

    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    fn cap(&self) -> usize {
        self.buf.cap
    }

    /// Appends an element to the back of a collection.  The value of variable `elem` is moved
    /// into this `Vec` so that this `Vec` owns it.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` *bytes*.
    ///
    /// # Time complexity
    ///
    /// Takes amortized *O*(1) time.
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() {
            self.buf.grow();
        }

        unsafe {
            ptr::write(self.ptr().add(self.len), elem);
        }

        // Can't fail, we'll OOM first.
        self.len += 1;
    }

    /// Removes and returns the element most recently added to this `Vec`, or `None` if this `Vec`
    /// is empty.
    ///
    /// # Time complexity
    ///
    /// Takes &Theta;(1) time.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr().add(self.len))) }
        }
    }

    /// Inserts an element at position `index` with the vector, shifting all elements after it to
    /// the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    ///
    /// # Time complexity
    ///
    /// Takes *O*(`Vec::len`) time.  All items after the insertion index must be shifted to the
    /// right.  In the worst case, all elements are shifted when the insertion index is 0.
    pub fn insert(&mut self, index: usize, elem: T) {
        // Note: `<=` because it's valid to insert after everything which would be equivalent to
        // push.
        assert!(index <= self.len, "index out of bounds");
        if self.len == self.cap() {
            self.buf.grow();
        }

        unsafe {
            // ptr::copy(src, dest, len): "copy from src to dest len elems"
            ptr::copy(
                self.ptr().add(index),
                self.ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.ptr().add(index), elem);
        };

        self.len += 1;
    }

    /// Removes and returns the element at position `index` within the vector, shifting all elements
    /// after it to the left.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// # Time complexity
    ///
    /// Takes *O*(*n*) time.  Because of this shifts over the remaining elements, it has a
    /// worst-case performance of *O*(*n*).
    pub fn remove(&mut self, index: usize) -> T {
        // Note: `<` because it's *not* valid to remove after everything
        assert!(index < self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr().add(index));
            ptr::copy(
                self.ptr().add(index + 1),
                self.ptr().add(index),
                self.len - index,
            );
            result
        }
    }

    /// Removes the whole slice of the whole vector, returning a double-ended iterator over the
    /// removed slice.
    ///
    /// If the iterator is dropped before being fully consumed, it drops the remaining removed
    /// elements.
    ///
    /// The returned iterator keeps a mutable borrow on the vector to optimize its implementation.
    ///
    /// # Leaking
    ///
    /// If the returned iterator goes out of scope without being dropped (due to `mem::forget`, for
    /// example), the vector may have lost and leaked elements arbitrarily, including elements
    /// outside the range.
    pub fn drain(&mut self) -> Drain<'_, T> {
        let iter = unsafe { RawValIter::new(&self) };

        // This is mem::forget safety thing.  If Drain is forgotton, we just
        // leak the whole Vec's contents.  Also we need to do this *eventualy*
        // anyway, so why not do it now?
        self.len = 0;

        Drain {
            iter,
            vec: PhantomData,
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
        // deallocation is handled by RawVec
    }
}

/// Coerce `Vec<T>` to slice `[T]`.  A slice provides all sorts of bells and whistles such as `len`,
/// `first`, `last`, indexing, slicing, sorting, `iter`, `iter_mut`, etc.
impl<T> Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.len) }
    }
}

/// Coerce `Vec<T>` to slice `[T]`.  A slice provides all sorts of bells and whistles such as `len`,
/// `first`, `last`, indexing, slicing, sorting, `iter`, `iter_mut`, etc.
impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr(), self.len) }
    }
}

pub struct IntoIter<T> {
    _buf: RawVec<T>, // we don't actually care about this, just need it to live
    iter: RawValIter<T>,
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        let (iter, buf) = unsafe { (RawValIter::new(&self), ptr::read(&self.buf)) };
        mem::forget(self);

        IntoIter { iter, _buf: buf }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        // only need to ensure all our elements are read, and thus their destructors are called;
        // buffer will clean itself up afterwards.
        for _ in &mut *self {}
    }
}

pub struct Drain<'a, T: 'a> {
    vec: PhantomData<&'a mut Vec<T>>,
    iter: RawValIter<T>,
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_basics() {
        let mut v: Vec<&str> = Vec::new();
        v.push("hello");
        v.push("algs4");
        v.push("rs");
        v.push("lib");
        assert_eq!(v.len(), 4);
        assert_eq!(
            v.iter().cloned().collect::<std::vec::Vec<_>>(),
            ["hello", "algs4", "rs", "lib"]
        );
        assert_eq!(v.pop(), Some("lib"));
        assert_eq!(&v[2], &"rs");
        assert_eq!(v.pop(), Some("rs"));
        let mut itr = v.into_iter();
        assert_eq!(itr.next_back(), Some("algs4"));
        assert_eq!(itr.size_hint(), (1, Some(1)));
        assert_eq!(itr.next(), Some("hello"));
        assert_eq!(itr.next_back(), None);
        assert_eq!(itr.next(), None);
    }

    /// Compared to `std::vec::Vec`, our implementation is more strict.
    ///
    /// The following example code will not compile.  But it is OK.
    /// We do not allow such use.
    ///
    /// Making it compile requires relaxing the drop checker's conservative assumption which does
    /// not allow `T` (`&str` in this example) to dangle.
    ///
    /// To relax the borrow checking condition, we need to use the unstable
    /// feature: attribute `#[may_dangle]`, which does not compile in stable
    /// Rust.
    ///
    /// ```compile_fail,E0597
    /// fn test_may_dangle() {
    ///     let mut v: Vec<&str> = Vec::new();
    ///     let s: String = "Short-lived".into();
    ///     v.push(&s); // borrowed value &s does not live long enough
    ///     drop(s);
    /// }
    /// ```
    fn _doc_test() {}
}
