use std::collections::VecDeque;
use std::fmt;

/// A first-in-first-out (FIFO) queue of generic items.
///
/// It supports the usual `enqueue` and `dequeue` operations, along with methods for peeking at the first
/// item, testing if the queue is empty, and iterating through the items in FIFO order.
///
///
/// The `enqueue` and `dequeue` operations take constant amortized time.  The `len`, `peek`, and `is_empty`
/// operations take constant time in the worst case.
///
/// For additional documentation, see <a href="https://algs4.cs.princeton.edu/13queues">Section
/// 1.3</a> of <i>Algorithms, 4th Edition</i> by Robert Sedgewick and Kevin Wayne.
///
/// # Implementation considerations
///
/// This implementation uses an [`std::collections::VecDeque`], thus `ResizingQueue` is similar to
/// algs4 Java version `ResizingArrayQueue`.  However, `VecDeque` is more advanced because it uses a
/// circular buffer (also called ring buffer) to implement a double-ended queue, whereas our queue
/// is single-ended.
///
/// [`crate::SVecQueue`] is simpler than [`std::collections::VecDeque`], and is closer to algs4 Java
/// version `ResizingArrayQueue`.  It uses memory move to fill the "holes" that are left in the
/// front of the queue because of the `dequeue` operations.
///
/// See [`crate::LinkedQueue`] for a version that uses a linked list.
#[derive(Clone)]
pub struct ResizingQueue<T> {
    data: VecDeque<T>,
}

impl<T> ResizingQueue<T> {
    /// Initializes an empty queue.
    pub fn new() -> Self {
        ResizingQueue {
            data: VecDeque::new(),
        }
    }

    /// Is this queue empty?
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of items on this queue.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Adds the item to this queue.
    pub fn enqueue(&mut self, item: T) {
        self.data.push_back(item);
    }

    /// Removes and returns the item on this queue that was least recently added, or `None` if the
    /// queue is empty.
    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    /// Returns (but does not remove) the item least recently added to this queue.
    pub fn peek(&self) -> Option<&T> {
        self.data.back()
    }

    /// Returns an iterator that iterates over the items in this queue in FIFO order.
    pub fn iter(&self) -> ResizingQueueIter<'_, T> {
        ResizingQueueIter {
            data: &self.data,
            cursor: 0, // points to the front Node
        }
    }
}

impl<T> Default for ResizingQueue<T> {
    fn default() -> Self {
        ResizingQueue::new()
    }
}

pub struct ResizingQueueIter<'a, T> {
    data: &'a VecDeque<T>,
    cursor: usize,
}

impl<'a, T> Iterator for ResizingQueueIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < self.data.len() {
            let i = self.cursor;
            self.cursor += 1;
            Some(&self.data[i])
        } else {
            None
        }
    }
}

/// Implementing `std::fmt::Display` will automatically implement the `ToString` trait for
/// `ResizingQueue<T>`, allowing the usage of the `.to_string()` method.
impl<T: fmt::Display> fmt::Display for ResizingQueue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for x in self.data.iter() {
            s.push_str(&x.to_string());
            s.push(' ');
        }
        write!(f, "{}", s)
    }
}
