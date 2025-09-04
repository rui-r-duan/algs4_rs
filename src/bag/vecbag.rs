/// The `VecBag` struct represents a bag (or multiset) of generic items.  It supports insertion
/// and iterating over the items in arbitrary order.
///
/// This implementation uses an `std::vec::Vec`.  `VecBag` is similar to algs4 Java version
/// `ResizingArrayBag`.  See `LinkedBag` for a version that uses a linked list.
///
/// The `add`, `isEmpty`, and `size` operations take constant time.  Iteration takes time
/// proportional to the number of items.
///
/// For additional documentation, see <a href="https://algs4.cs.princeton.edu/13stacks">Section
/// 1.3</a> of <i>Algorithms, 4th Edition</i> by Robert Sedgewick and Kevin Wayne.
pub struct VecBag<T> {
    data: Vec<T>,
}

impl<T> VecBag<T> {
    /// Initializes an empty bag.
    pub fn new() -> Self {
        VecBag { data: Vec::new() }
    }

    /// Returns true if this bag is empty, returns false otherwise.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of items in this bag.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Adds the item to this bag.
    pub fn add(&mut self, item: T) {
        self.data.push(item);
    }

    /// Returns an iterator that iterates over the items in this bag in arbitrary order.
    pub fn iter(&self) -> VecBagIter<'_, T> {
        VecBagIter {
            data: &self.data[..],
            current: 0,
        }
    }
}

impl<T> Default for VecBag<T> {
    fn default() -> Self {
        VecBag::new()
    }
}

pub struct VecBagIter<'a, T> {
    data: &'a [T],
    current: usize,
}

impl<'a, T> Iterator for VecBagIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.data.len() {
            let i = self.current;
            self.current += 1;
            Some(&self.data[i])
        } else {
            None
        }
    }
}
