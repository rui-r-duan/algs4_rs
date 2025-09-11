use std::fmt;
use std::ptr::NonNull;

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
/// This implementation uses a singly linked list.  See [`crate::ResizingQueue`] and
/// [`crate::SVecQue`] for versions that use resizing vectors.
///
/// This implementation uses `Option<NonNull<Node<T>`.
///
/// There are some alternatives.  We do not choose them because of the following issues.
///
/// - mut pointer `*mut T`: `LinkedQueue<T>` is not covariant over type `T`.
///   See `examples/subtyping_variance.rs` and `examples/linkedqueue_mut_pointer.rs`.
///
/// - `Option<Rc<RefCell<Node<T>>>`: very hard (almost impossible) to implement
///   `Iterator` for `LinkedQueueIter<'a, T>`, `peek()`'s return type cannot be `Option<&T>`,
///   it has to be changed to `Option<Ref<T>>`.
///   See <https://rust-unofficial.github.io/too-many-lists/fourth-iteration.html>.
///
/// - `Option<Box<Node<T>>>`: cannot use two `Box`s to point to the same `Node`, which is needed
///   in a linked queue.
///
/// - `Option<Rc<Box<Node<T>>>`: immutable list with sharable sub-lists because it can can have
///   multiple pointers to the same `Node`.  The issue is that we cannot change the `next` smart
///   pointer in a `Rc<Box<Node>>`, the consequence is that `push_back` and `pop_back` (for a
///   double ended queue) needs reconstructing the whole list, which is <em>O</em>(<em>N</em>),
///   where <em>N</em> is the number of elements in the list.
///
/// In Rust, there are *many* ways to implement linked lists to meet various needs (e.g. simplicity,
/// memory efficiency, generic element type, specific element type, type safety), the complexity
/// varies.  The complexity comes from the rules of ownership and borrow checking and type safety.
/// We have the same level of control over the memory usage like that in C programming language,
/// while we get more safety.
///
/// For more discussions about implementing linked lists, see
/// <https://rust-unofficial.github.io/too-many-lists/index.html>.
pub struct LinkedQueue<T> {
    front: Option<NonNull<Node<T>>>, // beginning of queue
    back: Option<NonNull<Node<T>>>,  // end of queue
    n: usize,                        // number of elements on queue
}

struct Node<T> {
    item: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(item: T) -> Self {
        Node { item, next: None }
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
            front: None,
            back: None,
            n: 0,
        };
        debug_assert!(st.check());
        st
    }

    /// Is this queue empty?
    pub fn is_empty(&self) -> bool {
        self.front.is_none()
    }

    /// Returns the number of items on this queue.
    pub fn len(&self) -> usize {
        self.n
    }

    /// Adds the item to this queue.
    pub fn enqueue(&mut self, item: T) {
        // Add to the back.
        let new_back = Some(NonNull::new(Box::into_raw(Box::new(Node::new(item)))).unwrap());
        if self.back.is_none() {
            self.front = new_back;
        } else {
            unsafe {
                self.back.unwrap().as_mut().next = new_back;
            }
        }
        self.back = new_back;
        self.n += 1;
        debug_assert!(self.check());
    }

    /// Removes and returns the item on this queue that was least recently added, or `None` if the
    /// queue is empty.
    pub fn dequeue(&mut self) -> Option<T> {
        // Remove from the front.
        self.front?; // if it is_none(), return None
        let front = unsafe { Box::from_raw(self.front.unwrap().as_ptr()) };
        self.front = front.next;
        if self.front.is_none() {
            self.back = None;
        }
        self.n -= 1;
        debug_assert!(self.check());
        Some(front.item)
    }

    /// Returns (but does not remove) the item least recently added to this queue.
    pub fn peek(&self) -> Option<&T> {
        // Peek the front.
        unsafe { self.front.map(|non_null| &(*non_null.as_ptr()).item) }
    }

    /// Returns an iterator that iterates over the items in this bag in FIFO order.
    pub fn iter(&self) -> LinedQueueIter<'_, T> {
        LinedQueueIter {
            current: unsafe { self.front.map(|non_null| &*non_null.as_ptr()) },
        }
    }

    // Check internal invariants.
    fn check(&self) -> bool {
        if self.n == 0 {
            if self.front.is_some() || self.back.is_some() {
                return false;
            }
        } else if self.n == 1 {
            if self.front.is_none() || self.back.is_none() {
                return false;
            }
            if !std::ptr::eq(self.front.unwrap().as_ptr(), self.back.unwrap().as_ptr()) {
                return false;
            }
            unsafe {
                if (*self.front.unwrap().as_ptr()).next.is_some() {
                    return false;
                }
            }
        } else {
            if self.front.is_none() || self.back.is_none() {
                return false;
            }
            if std::ptr::eq(self.front.unwrap().as_ptr(), self.back.unwrap().as_ptr()) {
                return false;
            }
            unsafe {
                if (*self.front.unwrap().as_ptr()).next.is_none() {
                    return false;
                }
                if (*self.back.unwrap().as_ptr()).next.is_some() {
                    return false;
                }
            }

            // check internal consistency of instance variable n
            let mut count_nodes: usize = 0;
            let mut x = self.front;
            while x.is_some() && count_nodes <= self.n {
                count_nodes += 1;
                x = unsafe { (*x.unwrap().as_ptr()).next };
            }
            if count_nodes != self.n {
                return false;
            }

            // check internal consistency of instance variable back
            let mut y = self.front;
            unsafe {
                while (*y.unwrap().as_ptr()).next.is_some() {
                    y = (*y.unwrap().as_ptr()).next;
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
                self.current = node.next.map(|non_null| &*non_null.as_ptr());
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
