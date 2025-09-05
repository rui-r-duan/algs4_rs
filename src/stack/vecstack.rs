use std::fmt;

/// The `VecStack` struct represents a last-in-first-out (LIFO) stack of generic items.  It supports
/// the usual `push` and `pop` operations, along with methods for peeking at the top item, testing
/// if the stack is empty, and iterating through the items in LIFO order.
///
/// This implementation uses an `std::vec::Vec`.  `VecStack` is similar to algs4 Java version
/// `ResizingArrayStack`.  See `LinkedStack` for a version that uses a linked list.
///
/// The `push` and `pop` operations take constant amortized time.  The `len`, `peek`, and `is_empty`
/// operations take constant time in the worst case.
///
/// For additional documentation, see <a href="https://algs4.cs.princeton.edu/13stacks">Section
/// 1.3</a> of <i>Algorithms, 4th Edition</i> by Robert Sedgewick and Kevin Wayne.
pub struct VecStack<T> {
    data: Vec<T>,
}

impl<T> VecStack<T> {
    /// Initializes an empty stack.
    pub fn new() -> Self {
        VecStack { data: Vec::new() }
    }

    /// Is this stack empty?
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of items in this stack.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Adds the item to this stack.
    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    /// Removes and returns the item most recently added to this stack, or `None` if the stack is
    /// empty.
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Returns (but does not remove) the item most recently added to this stack.
    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    /// Returns an iterator that iterates over the items in this stack in LIFO order.
    pub fn iter(&self) -> VecStackIter<'_, T> {
        VecStackIter {
            data: &self.data[..],
            cursor: self.data.len(), // points to the next of the top (end) Node
        }
    }
}

impl<T> Default for VecStack<T> {
    fn default() -> Self {
        VecStack::new()
    }
}

pub struct VecStackIter<'a, T> {
    data: &'a [T],
    cursor: usize,
}

impl<'a, T> Iterator for VecStackIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor > 0 {
            let i = self.cursor - 1;
            self.cursor -= 1;
            Some(&self.data[i])
        } else {
            None
        }
    }
}

/// Implementing `std::fmt::Display` will automatically implement the `ToString` trait for
/// `VecStack<T>`, allowing the usage of the `.to_string()` method.
impl<T: fmt::Display> fmt::Display for VecStack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for x in self.data.iter().rev() {
            s.push_str(&x.to_string());
            s.push(' ');
        }
        write!(f, "{}", s)
    }
}
