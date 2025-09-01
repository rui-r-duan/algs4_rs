//! A generic bag or multiset, implemented using a singly linked list.

/// The `Bag` struct represents a bag (or multiset) of generic items.  It supports insertion and
/// iterating over the items in arbitrary order.
///
/// This implementation uses a singly linked list.  See `ResizingArrayBag` for a version that uses a
/// resizing array.
///
/// The <em>add</em>, <em>isEmpty</em>, and <em>size</em> operations take constant time. Iteration
/// takes time proportional to the number of items.
///
/// For additional documentation, see <a href="https://algs4.cs.princeton.edu/13stacks">Section
/// 1.3</a> of <i>Algorithms, 4th Edition</i> by Robert Sedgewick and Kevin Wayne.
pub struct Bag<T> {
    first: Option<Box<Node<T>>>, // beginning of bag
    n: usize,                    // number of elements in bag
}

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

impl<T> Bag<T> {
    /// Initializes an empty bag.
    pub fn new() -> Self {
        Bag {
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
    pub fn iter(&self) -> BagIter<'_, T> {
        BagIter {
            current: self.first.as_ref().map(|b| b.as_ref()),
        }
    }
}

impl<T> Default for Bag<T> {
    fn default() -> Self {
        Bag::new()
    }
}

pub struct BagIter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for BagIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_ref().map(|b| b.as_ref());
            &node.item
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn bag_of_string() {
        let mut bag = Bag::new();
        let list = ["to", "be", "or", "not", "to", "-", "be", "-", "-", "that", "-", "-", "-", "is"];
        for s in list {
            bag.add(s);
        }

        println!("size of bag = {}", bag.len());
        assert_eq!(bag.len(), 14);
        assert_eq!(bag.iter().map(|s| *s).collect::<HashSet<&str>>(), HashSet::from(list));
    }
}
