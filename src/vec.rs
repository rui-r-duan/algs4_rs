//! A vector implementation inspired by the Vec implmenetation in [The
//! Rustonomicon](https://doc.rust-lang.org/nomicon/vec/vec.html).
//!
//! The nonicon version does not have the `shrink` allocation, while this implementation does.
//! **For the nitty-gritty, please read The Rustonomicon.**

use std::alloc::{self, Layout};
use std::mem::{self, ManuallyDrop};
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::{self, NonNull};

pub struct Vec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

impl<T> Vec<T> {
    /// Create an empty `Vec` which does not allocate any memory.
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        Vec {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            // This can't overflow since self.cap <= isize::MAX.
            let new_cap = 2 * self.cap;

            // `Layout::array` checks that the number of bytes is <= usize::MAX,
            // but this is redundant since old_layout.size() <= isize::MAX,
            // so the `unwrap` should never fail.
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // Ensure that the new allocation doesn't exceed `isize::MAX` bytes.
        // This is related to LLVM's GetElementPtr (GEP) inbounds instruction.
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // If allocation fails, `new_ptr` will be null, in which case we abort.
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
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
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), elem);
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
            unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.len))) }
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
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            // ptr::copy(src, dest, len): "copy from src to dest len elems"
            ptr::copy(
                self.ptr.as_ptr().add(index),
                self.ptr.as_ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.ptr.as_ptr().add(index), elem);
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
            let result = ptr::read(self.ptr.as_ptr().add(index));
            ptr::copy(
                self.ptr.as_ptr().add(index + 1),
                self.ptr.as_ptr().add(index),
                self.len - index,
            );
            result
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while let Some(_) = self.pop() {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

/// Coerce `Vec<T>` to slice `[T]`.  A slice provides all sorts of bells and whistles such as `len`,
/// `first`, `last`, indexing, slicing, sorting, `iter`, `iter_mut`, etc.
impl<T> Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

/// Coerce `Vec<T>` to slice `[T]`.  A slice provides all sorts of bells and whistles such as `len`,
/// `first`, `last`, indexing, slicing, sorting, `iter`, `iter_mut`, etc.
impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

pub struct IntoIter<T> {
    buf: NonNull<T>,
    cap: usize,
    start: *const T,
    end: *const T,
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        // Make sure not to drop Vec since that would free the buffer
        let vec = ManuallyDrop::new(self);

        // Can't destructure Vec since it's Drop
        let ptr = vec.ptr;
        let cap = vec.cap;
        let len = vec.len;

        IntoIter {
            buf: ptr,
            cap,
            start: ptr.as_ptr(),
            end: if cap == 0 {
                // can't offset off this pointer, it's not allocated!
                ptr.as_ptr()
            } else {
                unsafe { ptr.as_ptr().add(len) }
            },
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(result)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize) / mem::size_of::<T>();
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            // drop any remaining elements
            for _ in &mut *self {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.buf.as_ptr() as *mut u8, layout);
            }
        }
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
