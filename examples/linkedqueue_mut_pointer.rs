//! This is an alternative implmentation for `LinkedQueue`.  It functions well as a queue, but it is
//! NOT covariant over the generic parameter `T`.
//!
//! Use it in `examples/subtyping_variance.rs` or copying the utility from
//! `examples/subtyping_variance.rs` to this file, and see that this LinkedQueue is not covariant
//! over T.

use algs4_rs::StdIn;
use std::fmt;
use std::ptr;

/// A first-in-first-out (FIFO) queue of generic items.
///
/// It supports the usual `enqueue` and `dequeue` operations, along with methods for peeking at the first
/// item, testing if the queue is empty, and iterating through the items in FIFO order.
///
/// This implementation uses a singly linked list.  See [`crate::ResizingQueue`] for a version that
/// uses a resizing Vec.
///
/// The `enqueue`, `dequeue`, `peek`, `len`, and `is_empty` operations all take constant time in the
/// worst case.
///
/// For additional documentation, see <a href="https://algs4.cs.princeton.edu/13stacks">Section
/// 1.3</a> of <i>Algorithms, 4th Edition</i> by Robert Sedgewick and Kevin Wayne.
pub struct LinkedQueue<T> {
    front: *mut Node<T>, // beginning of queue
    back: *mut Node<T>,  // end of queue
    n: usize,            // number of elements on queue
}

struct Node<T> {
    item: T,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(item: T) -> Self {
        Node {
            item,
            next: ptr::null_mut(),
        }
    }
}

impl<T: Default> Default for Node<T> {
    fn default() -> Self {
        Node::new(T::default())
    }
}

impl<T> LinkedQueue<T> {
    /// Initializes an empty queue.
    pub fn new() -> Self {
        let st = LinkedQueue {
            front: ptr::null_mut(),
            back: ptr::null_mut(),
            n: 0,
        };
        debug_assert!(st.check());
        st
    }

    /// Is this queue empty?
    pub fn is_empty(&self) -> bool {
        self.front.is_null()
    }

    /// Returns the number of items in this queue.
    pub fn len(&self) -> usize {
        self.n
    }

    /// Adds the item to this queue.
    pub fn enqueue(&mut self, item: T) {
        // Add to the back.
        let new_back = Box::into_raw(Box::new(Node::new(item)));
        if self.back.is_null() {
            self.front = new_back;
        } else {
            unsafe {
                (*self.back).next = new_back;
            }
        }
        self.back = new_back;
        self.n += 1;
        debug_assert!(self.check());
    }

    /// Removes and returns the itemon this queue that was least recently added, or `None` if the
    /// queue is empty.
    pub fn dequeue(&mut self) -> Option<T> {
        // Remove from the front.
        if self.front.is_null() {
            return None;
        }
        let front = unsafe { Box::from_raw(self.front) };
        self.front = front.next;
        if self.front.is_null() {
            self.back = ptr::null_mut();
        }
        self.n -= 1;
        debug_assert!(self.check());
        Some(front.item)
    }

    /// Returns (but does not remove) the item least recently added to this queue.
    pub fn peek(&self) -> Option<&T> {
        // Peek the front.
        if self.front.is_null() {
            None
        } else {
            unsafe { Some(&(*self.front).item) }
        }
    }

    /// Returns an iterator that iterates over the items in this bag in LIFO order.
    pub fn iter(&self) -> LinedQueueIter<'_, T> {
        LinedQueueIter {
            current: if self.front.is_null() {
                None
            } else {
                unsafe { Some(&*self.front) }
            },
        }
    }

    // Check internal invariants.
    fn check(&self) -> bool {
        if self.n == 0 {
            if !self.front.is_null() || !self.back.is_null() {
                return false;
            }
        } else if self.n == 1 {
            if self.front.is_null() || self.back.is_null() {
                return false;
            }
            if !std::ptr::eq(self.front, self.back) {
                return false;
            }
            unsafe {
                if !(*self.front).next.is_null() {
                    return false;
                }
            }
        } else {
            if self.front.is_null() || self.back.is_null() {
                return false;
            }
            if std::ptr::eq(self.front, self.back) {
                return false;
            }
            unsafe {
                if (*self.front).next.is_null() {
                    return false;
                }
                if !(*self.back).next.is_null() {
                    return false;
                }
            }

            // check internal consistency of instance variable n
            let mut count_nodes: usize = 0;
            let mut x = self.front;
            while !x.is_null() && count_nodes <= self.n {
                count_nodes += 1;
                x = unsafe { (*x).next };
            }
            if count_nodes != self.n {
                return false;
            }

            // check internal consistency of instance variable back
            let mut y = self.front;
            unsafe {
                while !(*y).next.is_null() {
                    y = (*y).next;
                }
            }
            if y != self.back {
                return false;
            }
        }

        true
    }
}

impl<T> Default for LinkedQueue<T> {
    fn default() -> Self {
        LinkedQueue::new()
    }
}

impl<T> Drop for LinkedQueue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
    }
}

pub struct LinedQueueIter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for LinedQueueIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            unsafe {
                self.current = if node.next.is_null() {
                    None
                } else {
                    Some(&*node.next)
                };
            }
            &node.item
        })
    }
}

impl<T: Clone> Clone for LinkedQueue<T> {
    fn clone(&self) -> Self {
        let mut newq = LinkedQueue::new();
        for x in self.iter() {
            newq.enqueue(x.clone());
        }
        newq
    }
}

/// Implementing `std::fmt::Display` will automatically implement the `ToString` trait for
/// `LinkedQueue<T>`, allowing the usage of the `.to_string()` method.
impl<T: fmt::Display> fmt::Display for LinkedQueue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for x in self.iter() {
            s.push_str(&x.to_string());
            s.push(' ');
        }
        write!(f, "{}", s)
    }
}

////////////////////////////////////////////////////////////////////////////////
// This LinkedQueue<T> is not covariant over T.
// The following code snippet does not compile.
/*
type F<T> = LinkedQueue<T>

fn _two_refs<'short, 'long: 'short>(a: F<&'short str>, b: F<&'long str>) {
   _take_two(a, b);
}
fn _take_two<T>(_val1: T, _val2: T) {}
*/
////////////////////////////////////////////////////////////////////////////////

fn main() -> std::io::Result<()> {
    let mut qu = LinkedQueue::new();
    let mut stdin = StdIn::new();
    while !stdin.is_empty() {
        let item = stdin.read_string()?;
        if item != "-" {
            qu.enqueue(item);
        } else if !qu.is_empty() {
            print!("{} ", qu.dequeue().unwrap());
        }
    }
    println!("({} left on queue)", qu.len());
    Ok(())
}
