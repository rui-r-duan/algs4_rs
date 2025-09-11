use crate::vec::raw_vec::RawVec;
use std::fmt;
use std::ptr;

/// A first-in-first-out (FIFO) queue of generic items.
///
/// It supports the usual `enqueue` and `dequeue` operations, along with methods for peeking at the first
/// item, testing if the queue is empty, and iterating through the items in FIFO order.
///
/// The `enqueue`, `dequeue`, `peek`, `len`, and `is_empty` operations all take constant time in the
/// worst case.
///
/// For additional documentation, see <a href="https://algs4.cs.princeton.edu/13stacks">Section
/// 1.3</a> of <i>Algorithms, 4th Edition</i> by Robert Sedgewick and Kevin Wayne.
///
/// # Implementation considerations
///
/// This implementation uses a resizing vector, thus `SVecQue` is similar to algs4 Java version
/// `ResizingArrayQueue`.
///
/// Compared to [`crate::ResizingQueue`], `SVecQue` is closer to the Java `ResizingArrayQueue`.
/// Because `SVecQue` does not use a ring buffer, instead it uses a buffer without any circle, and
/// it uses memory move to fill the "holes" that are left in the front of the queue because of the
/// `dequeue` operations, which is what the Java version does.  See the documentation of
/// [`crate::ResizingQueue`] for more information.
///
/// See [`crate::LinkedQueue`] for a version that uses a linked list.
pub struct SVecQue<T> {
    buf: RawVec<T>,
    // valid index range is open range: front..back (include front, exclude back)
    front: usize, // point to the front element (least recently added)
    back: usize,  // point to the next slot after the back element (most recently added)
}

impl<T> SVecQue<T> {
    /// Create an empty `SVecDeque` which does not allocate any memory.
    pub fn new() -> Self {
        SVecQue {
            buf: RawVec::new(),
            front: 0,
            back: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.back == self.front
    }

    pub fn len(&self) -> usize {
        self.back - self.front
    }

    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    fn cap(&self) -> usize {
        self.buf.cap
    }

    pub fn enqueue(&mut self, elem: T) {
        if self.back == self.cap() && self.front > 0 {
            self.move_to_front();
        }
        if self.len() == self.cap() {
            self.buf.grow();
        }

        unsafe {
            ptr::write(self.ptr().add(self.back), elem);
        }

        self.back += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let elem = unsafe { Some(ptr::read(self.ptr().add(self.front))) };
            self.front += 1;
            if self.len() == self.buf.cap / 4 {
                self.move_to_front();
                self.buf.shrink();
            }
            elem
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { Some(&*self.ptr().add(self.front)) }
    }

    fn move_to_front(&mut self) {
        let diff = self.front;
        unsafe {
            ptr::copy(self.ptr().add(self.front), self.ptr(), self.len());
        }
        self.front -= diff;
        self.back -= diff;
    }

    pub fn iter(&self) -> SVecQueIter<'_, T> {
        SVecQueIter {
            buf: &self.buf,
            front: self.front,
            back: self.back,
        }
    }
}

pub struct SVecQueIter<'a, T> {
    buf: &'a RawVec<T>,
    front: usize,
    back: usize,
}

impl<'a, T> Iterator for SVecQueIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.front < self.back {
            let i = self.front;
            self.front += 1;
            let elem = unsafe { &*self.buf.ptr.as_ptr().add(i) };
            Some(elem)
        } else {
            None
        }
    }
}

/// Implementing `std::fmt::Display` will automatically implement the `ToString` trait for
/// `SVecQue<T>`, allowing the usage of the `.to_string()` method.
impl<T: fmt::Display> fmt::Display for SVecQue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for x in self.iter() {
            s.push_str(&x.to_string());
            s.push(' ');
        }
        write!(f, "{}", s)
    }
}

impl<T: Clone> Clone for SVecQue<T> {
    fn clone(&self) -> Self {
        let mut q = SVecQue {
            buf: self.buf.clone(),
            front: 0,
            back: 0,
        };
        for elem in self.iter() {
            q.enqueue(elem.clone());
        }
        q
    }
}

impl<T> Default for SVecQue<T> {
    fn default() -> Self {
        Self::new()
    }
}
