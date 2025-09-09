use std::fmt;

/// The `LinkedStack` struct represents a last-in-first-out (LIFO) stack of generic items.  It
/// supports the usual `push` and `pop` operations, along with methods for peeking at the top item,
/// testing if the stack is empty, and iterating through the items in LIFO order.
///
/// This implementation uses a singly linked list.  See `VecStack` for a version that uses a
/// resizing Vec.
///
/// The `push`, `pop`, `peek`, `len`, and `is_empty` operations all take constant time in the worst case.
///
/// For additional documentation, see <a href="https://algs4.cs.princeton.edu/13stacks">Section
/// 1.3</a> of <i>Algorithms, 4th Edition</i> by Robert Sedgewick and Kevin Wayne.
#[derive(Clone)]
pub struct LinkedStack<T> {
    first: Option<Box<Node<T>>>, // top of stack
    n: usize,                    // size of the stack
}

#[derive(Clone)]
struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
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

impl<T> LinkedStack<T> {
    /// Initializes an empty stack.
    pub fn new() -> Self {
        let st = LinkedStack { first: None, n: 0 };
        debug_assert!(st.check());
        st
    }

    /// Is this stack empty?
    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    /// Returns the number of items in this stack.
    pub fn len(&self) -> usize {
        self.n
    }

    /// Adds the item to this stack.
    pub fn push(&mut self, item: T) {
        let mut new_node = Node::new(item);
        new_node.next = self.first.take();
        self.first = Some(Box::new(new_node));
        self.n += 1;
        debug_assert!(self.check());
    }

    /// Removes and returns the item most recently added to this stack, or `None` if the stack is
    /// empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let top = *self.first.take().unwrap();
        self.first = top.next;
        self.n -= 1;
        debug_assert!(self.check());
        Some(top.item)
    }

    /// Returns (but does not remove) the item most recently added to this stack.
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        Some(&self.first.as_ref().unwrap().item)
    }

    /// Returns an iterator that iterates over the items in this bag in LIFO order.
    pub fn iter(&self) -> LinedStackIter<'_, T> {
        LinedStackIter {
            current: self.first.as_ref().map(|b| b.as_ref()),
        }
    }

    // Check internal invariants.
    fn check(&self) -> bool {
        if self.n == 0 {
            if self.first.is_some() {
                return false;
            }
        } else if self.n == 1 {
            if self.first.is_none() {
                return false;
            }
            if self.first.as_ref().unwrap().next.is_some() {
                return false;
            }
        } else {
            if self.first.is_none() {
                return false;
            }
            if self.first.as_ref().unwrap().next.is_none() {
                return false;
            }
        }

        // check internal consistency of `self.n`
        let mut node_cnt = 0usize;
        let mut x = self.first.as_ref();
        while x.is_some() && node_cnt <= self.n {
            node_cnt += 1;
            x = x.unwrap().next.as_ref();
        }
        if node_cnt != self.n {
            return false;
        }

        true
    }
}

impl<T> Default for LinkedStack<T> {
    fn default() -> Self {
        LinkedStack::new()
    }
}

/// Without this iterative `drop`, the compiler generated `drop` will be a recursive procedure.  It
/// drops the Boxed nodes recursively.  It will not be a tail recursive call, because dropping
/// `Box<Node<T>>` is not a tail call, after dropping `Node<T>`, it has an extra operation: dropping
/// Box.
///
/// The recursive call can overflow the process's stack!
impl<T> Drop for LinkedStack<T> {
    fn drop(&mut self) {
        while self.first.is_some() {
            let first = *self.first.take().unwrap();
            self.first = first.next;
        }
    }
}

pub struct LinedStackIter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for LinedStackIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_ref().map(|b| b.as_ref());
            &node.item
        })
    }
}

/// Implementing `std::fmt::Display` will automatically implement the `ToString` trait for
/// `LinkedStack<T>`, allowing the usage of the `.to_string()` method.
impl<T: fmt::Display> fmt::Display for LinkedStack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for x in self.iter() {
            s.push_str(&x.to_string());
            s.push(' ');
        }
        write!(f, "{}", s)
    }
}
