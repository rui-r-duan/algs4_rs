use std::cmp::Reverse;

use crate::MaxPQ;

/// A priority queue of generic keys.  A better alternative is [`std::collections::BinaryHeap`].
///
/// It supports the usual `insert` and `del_min` operations, along with methods for peeking at the
/// minimum key, testing if the priority queue is empty, and iterating through the keys.
///
/// This implementation uses a <em>binary heap</em>.  The `insert` and `del_min` operations take
/// &Theta;(log <em>n</em>) amortized time, where <em>n</em> is the number of elements in the
/// priority queue. This is an amortized bound (and not a worst-case bound) because of array
/// resizing operations.
///
/// The `min`, `len`, `is_empty` operations take &Theta;(1) time in the worst case.
///
/// Construction takes time proportional to the specified capacity or the number of items used to
/// initialize the data structure.
///
/// We use a one-based array to simplify parent and child calculations.
///
/// Can be optimized by replacing full exchanges with half exchanges (aka insertion sort).
///
/// For additional documentation, see <a href="https://algs4.cs.princeton.edu/24pq">Section 2.4</a>
/// of <i>Algorithms, 4th Edition</i> by Robert Sedgewick and Kevin Wayne.
pub struct MinPQ<T> {
    pq: MaxPQ<Reverse<T>>,
}

impl<T> MinPQ<T>
where
    T: Ord + Default,
{
    /// Creates an empty priority queue.
    pub fn new() -> Self {
        MinPQ { pq: MaxPQ::new() }
    }

    /// Creates an empty priority queue with the given initial capacity.
    ///
    /// If capacity is zero, no allocation.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` *bytes*.
    pub fn with_capacity(capacity: usize) -> Self {
        MinPQ {
            pq: MaxPQ::with_capacity(capacity),
        }
    }

    /// Returns true if this priority queue is empty.
    pub fn is_empty(&self) -> bool {
        self.pq.is_empty()
    }

    /// Returns the number of keys on this priority queue.
    pub fn len(&self) -> usize {
        self.pq.len()
    }

    /// Returns a smallest key on this priority queue.
    pub fn min(&self) -> Option<&T> {
        self.pq.max().map(|reversed| &reversed.0)
    }

    /// Adds a new key to this priority queue.
    pub fn insert(&mut self, x: T) {
        self.pq.insert(Reverse(x));
    }

    /// Removes and returns a smallest key on this priority queue.
    pub fn del_min(&mut self) -> Option<T> {
        let result = self.pq.del_max().map(|reversed| reversed.0);
        result
    }
}

impl<T> From<&[T]> for MinPQ<T>
where
    T: Ord + Default + Clone,
{
    fn from(keys: &[T]) -> Self {
        let n = keys.len();
        let cloned_reversed_keys: Vec<Reverse<T>> =
            keys.iter().map(|x| Reverse(x.clone())).collect();
        let mut maxpq = MaxPQ::with_capacity(n + 1);
        for x in cloned_reversed_keys {
            maxpq.insert(x);
        }
        MinPQ { pq: maxpq }
    }
}

impl<T, const N: usize> From<[T; N]> for MinPQ<T>
where
    T: Ord + Default + Clone,
{
    fn from(keys: [T; N]) -> Self {
        let cloned_reversed_keys: Vec<Reverse<T>> =
            keys.iter().map(|x| Reverse(x.clone())).collect();
        let mut maxpq = MaxPQ::with_capacity(N + 1);
        for x in cloned_reversed_keys {
            maxpq.insert(x);
        }
        MinPQ { pq: maxpq }
    }
}

impl<T> Clone for MinPQ<T>
where
    T: Ord + Default + Clone,
{
    fn clone(&self) -> Self {
        MinPQ {
            pq: self.pq.clone(),
        }
    }
}

pub struct MinPQIntoIter<T> {
    moved_pq: MinPQ<T>,
}

impl<T> IntoIterator for MinPQ<T>
where
    T: Ord + Default + Clone,
{
    type Item = T;
    type IntoIter = MinPQIntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        MinPQIntoIter { moved_pq: self }
    }
}

impl<T> Iterator for MinPQIntoIter<T>
where
    T: Ord + Default + Clone,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.moved_pq.is_empty() {
            None
        } else {
            self.moved_pq.del_min()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minpq_basics() {
        let mut pq = MinPQ::new();
        pq.insert(1);
        pq.insert(5);
        pq.insert(2);
        assert_eq!(pq.min(), Some(&1));
        assert_eq!(pq.len(), 3);
        assert_eq!(pq.del_min(), Some(1));
        assert_eq!(pq.del_min(), Some(2));
        assert_eq!(pq.del_min(), Some(5));
        assert_eq!(pq.del_min(), None);
        assert!(pq.is_empty());
    }

    #[test]
    fn minpq_with_capacity() {
        let mut pq = MinPQ::with_capacity(5);
        pq.insert(1);
        pq.insert(5);
        pq.insert(2);
        pq.insert(80);
        pq.insert(4);
        pq.insert(-57);
        assert_eq!(pq.min(), Some(&-57));
        assert_eq!(pq.len(), 6);
        assert_eq!(pq.del_min(), Some(-57));
        assert_eq!(pq.del_min(), Some(1));
        assert_eq!(pq.del_min(), Some(2));
        assert_eq!(pq.del_min(), Some(4));
        assert_eq!(pq.del_min(), Some(5));
        assert_eq!(pq.del_min(), Some(80));
        assert_eq!(pq.del_min(), None);
        assert!(pq.is_empty());
    }

    #[test]
    fn minpq_from_slice() {
        let array = [1, 5, 2, 80, 4, -57];
        let slice = &array[..];
        let mut pq = MinPQ::from(slice);
        assert_eq!(pq.min(), Some(&-57));
        assert_eq!(pq.len(), 6);
        assert_eq!(pq.del_min(), Some(-57));
        assert_eq!(pq.del_min(), Some(1));
        assert_eq!(pq.del_min(), Some(2));
        assert_eq!(pq.del_min(), Some(4));
        assert_eq!(pq.del_min(), Some(5));
        assert_eq!(pq.del_min(), Some(80));
        assert_eq!(pq.del_min(), None);
        assert!(pq.is_empty());
    }

    #[test]
    fn minpq_from_array() {
        let array = [1, 5, 2, 80, 4, -57];
        let mut pq = MinPQ::from(array);
        assert_eq!(pq.min(), Some(&-57));
        assert_eq!(pq.len(), 6);
        assert_eq!(pq.del_min(), Some(-57));
        assert_eq!(pq.del_min(), Some(1));
        assert_eq!(pq.del_min(), Some(2));
        assert_eq!(pq.del_min(), Some(4));
        assert_eq!(pq.del_min(), Some(5));
        assert_eq!(pq.del_min(), Some(80));
        assert_eq!(pq.del_min(), None);
        assert!(pq.is_empty());
    }

    #[test]
    fn minpq_clone_into_iter() {
        let array = [1, 5, 2, 80, 4, -57];
        let pq = MinPQ::from(array);
        let mut itr = pq.clone().into_iter();
        assert_eq!(itr.next(), Some(-57));
        assert_eq!(itr.next(), Some(1));
        assert_eq!(itr.next(), Some(2));
        assert_eq!(itr.next(), Some(4));
        assert_eq!(itr.next(), Some(5));
        assert_eq!(itr.next(), Some(80));
        assert_eq!(itr.next(), None);
    }
}
