/// A bag (or multiset) of generic items.
///
/// It supports insertion and iterating over the items in arbitrary order.
///
/// This implementation uses a singly linked list.  See [`crate::ResizingBag`] for a version that
/// uses a resizing Vec.
///
/// The `add`, `isEmpty`, and `size` operations take constant time.  Iteration takes time
/// proportional to the number of items.
///
/// For additional documentation, see <a href="https://algs4.cs.princeton.edu/13stacks">Section
/// 1.3</a> of <i>Algorithms, 4th Edition</i> by Robert Sedgewick and Kevin Wayne.
#[derive(Clone)]
pub struct LinkedBag<T> {
    first: Option<Box<Node<T>>>, // beginning of bag
    n: usize,                    // number of elements in bag
}

#[derive(Clone)]
struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(item: T) -> Self {
        Node {
            item,
            next: None,
        }
    }
}

impl<T: Default> Default for Node<T> {
    fn default() -> Self {
        Node::new(T::default())
    }
}

impl<T> LinkedBag<T> {
    /// Initializes an empty bag.
    pub fn new() -> Self {
        LinkedBag {
            first: None,
            n: 0,
        }
    }

    /// Returns true if this bag is empty, returns false otherwise.
    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    /// Returns the number of items in this bag.
    pub fn len(&self) -> usize {
        self.n
    }

    /// Adds the item to this bag.
    pub fn add(&mut self, item: T) {
        let mut new_node = Node::new(item);
        new_node.next = self.first.take();
        self.first = Some(Box::new(new_node));
        self.n += 1;
    }

    /// Returns an iterator that iterates over the items in this bag in arbitrary order.
    pub fn iter(&self) -> LinkedBagIter<'_, T> {
        LinkedBagIter {
            current: self.first.as_ref().map(|b| b.as_ref()),
        }
    }
}

impl<T> Default for LinkedBag<T> {
    fn default() -> Self {
        LinkedBag::new()
    }
}

/// Without this iterative `drop`, the compiler generated `drop` will be a recursive procedure.  It
/// drops the Boxed nodes recursively.  It will not be a tail recursive call, because dropping
/// `Box<Node<T>>` is not a tail call, after dropping `Node<T>`, it has an extra operation: dropping
/// Box.
///
/// The recursive call can overflow the process's stack!
impl<T> Drop for LinkedBag<T> {
    fn drop(&mut self) {
        while self.first.is_some() {
            let first = *self.first.take().unwrap();
            self.first = first.next;
        }
    }
}

pub struct LinkedBagIter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for LinkedBagIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_ref().map(|b| b.as_ref());
            &node.item
        })
    }
}
