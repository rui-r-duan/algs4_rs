use crate::SVec;

/// A priority queue of generic keys.  A better alternative is [`std::collections::BinaryHeap`].
///
/// It supports the usual `insert` and `del_max` operations, along with methods for peeking at the
/// maximum key, testing if the priority queue is empty, and iterating through the keys.
///
/// This implementation uses a <em>binary heap</em>.  The `insert` and `del_max` operations take
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
pub struct MaxPQ<T> {
    pq: SVec<T>, // store items at indices 1 to n
    len: usize,  // number of items on priority queue
}

impl<T> MaxPQ<T>
where
    T: Ord + Default,
{
    /// Creates an empty priority queue.
    pub fn new() -> Self {
        let mut data = SVec::new();
        data.push(T::default());
        MaxPQ { pq: data, len: 0 }
    }

    /// Creates an empty priority queue with the given initial capacity.
    ///
    /// If capacity is zero, no allocation.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` *bytes*.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut data = SVec::with_capacity(capacity);
        data.push(T::default());
        MaxPQ { pq: data, len: 0 }
    }

    /// Returns true if this priority queue is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of keys on this priority queue.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns a largest key on this priority queue.
    pub fn max(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.pq[1])
        }
    }

    /// Adds a new key to this priority queue.
    pub fn insert(&mut self, x: T) {
        self.len += 1;
        self.pq.push(x);
        self.swim(self.len);
        debug_assert!(self.is_max_heap());
    }

    /// Removes and returns a largest key on this priority queue.
    pub fn del_max(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.exch(1, self.len);
        let max = self.pq.pop().unwrap();
        self.len -= 1;
        self.sink(1);
        debug_assert!(self.is_max_heap());
        Some(max)
    }

    fn swim(&mut self, mut k: usize) {
        while k > 1 && self.less(k / 2, k) {
            self.exch(k / 2, k);
            k = k / 2;
        }
    }

    fn sink(&mut self, mut k: usize) {
        while 2 * k <= self.len {
            let mut j = 2 * k;
            if j < self.len && self.less(j, j + 1) {
                j += 1;
            }
            if !self.less(k, j) {
                break;
            }
            self.exch(k, j);
            k = j;
        }
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self.pq[i].cmp(&self.pq[j]).is_lt()
    }

    fn exch(&mut self, i: usize, j: usize) {
        self.pq.swap(i, j)
    }

    // is pq[1..=n] a max heap?
    fn is_max_heap(&self) -> bool {
        self.is_max_heap_ordered(1)
    }

    // is subtree of pq[1..=n] rooted at k a max heap?
    fn is_max_heap_ordered(&self, k: usize) -> bool {
        if k > self.len {
            return true;
        }
        let left = 2 * k;
        let right = 2 * k + 1;
        if left <= self.len && self.less(k, left) {
            false
        } else if right <= self.len && self.less(k, right) {
            false
        } else {
            self.is_max_heap_ordered(left) && self.is_max_heap_ordered(right)
        }
    }
}

impl<T> From<&[T]> for MaxPQ<T>
where
    T: Ord + Default + Clone,
{
    fn from(keys: &[T]) -> Self {
        let n = keys.len();
        let mut maxpq = MaxPQ::with_capacity(n + 1);
        for x in keys {
            maxpq.pq.push(x.clone());
            maxpq.len += 1;
        }
        let mut k = n / 2;
        while k >= 1 {
            maxpq.sink(k);
            k -= 1;
        }
        debug_assert!(maxpq.is_max_heap());
        maxpq
    }
}

impl<T, const N: usize> From<[T; N]> for MaxPQ<T>
where
    T: Ord + Default + Clone,
{
    fn from(keys: [T; N]) -> Self {
        let mut maxpq = MaxPQ::with_capacity(N + 1);
        for x in keys {
            maxpq.pq.push(x.clone());
            maxpq.len += 1;
        }
        let mut k = N / 2;
        while k >= 1 {
            maxpq.sink(k);
            k -= 1;
        }
        debug_assert!(maxpq.is_max_heap());
        maxpq
    }
}

impl<T> Clone for MaxPQ<T>
where
    T: Ord + Default + Clone,
{
    fn clone(&self) -> Self {
        MaxPQ {
            pq: self.pq.clone(),
            len: self.len,
        }
    }
}

pub struct MaxPQIntoIter<T> {
    moved_pq: MaxPQ<T>,
}

impl<T> IntoIterator for MaxPQ<T>
where
    T: Ord + Default + Clone,
{
    type Item = T;
    type IntoIter = MaxPQIntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        MaxPQIntoIter { moved_pq: self }
    }
}

impl<T> Iterator for MaxPQIntoIter<T>
where
    T: Ord + Default + Clone,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.moved_pq.is_empty() {
            None
        } else {
            self.moved_pq.del_max()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maxpq_basics() {
        let mut pq = MaxPQ::new();
        pq.insert(1);
        pq.insert(5);
        pq.insert(2);
        assert_eq!(pq.max(), Some(&5));
        assert_eq!(pq.len(), 3);
        assert_eq!(pq.del_max(), Some(5));
        assert_eq!(pq.del_max(), Some(2));
        assert_eq!(pq.del_max(), Some(1));
        assert_eq!(pq.del_max(), None);
        assert!(pq.is_empty());
    }

    #[test]
    fn maxpq_with_capacity() {
        let mut pq = MaxPQ::with_capacity(5);
        pq.insert(1);
        pq.insert(5);
        pq.insert(2);
        pq.insert(80);
        pq.insert(4);
        pq.insert(-57);
        assert_eq!(pq.max(), Some(&80));
        assert_eq!(pq.len(), 6);
        assert_eq!(pq.del_max(), Some(80));
        assert_eq!(pq.del_max(), Some(5));
        assert_eq!(pq.del_max(), Some(4));
        assert_eq!(pq.del_max(), Some(2));
        assert_eq!(pq.del_max(), Some(1));
        assert_eq!(pq.del_max(), Some(-57));
        assert_eq!(pq.del_max(), None);
        assert!(pq.is_empty());
    }

    #[test]
    fn maxpq_from_slice() {
        let array = [1, 5, 2, 80, 4, -57];
        let slice = &array[..];
        let mut pq = MaxPQ::from(slice);
        assert_eq!(pq.max(), Some(&80));
        assert_eq!(pq.len(), 6);
        assert_eq!(pq.del_max(), Some(80));
        assert_eq!(pq.del_max(), Some(5));
        assert_eq!(pq.del_max(), Some(4));
        assert_eq!(pq.del_max(), Some(2));
        assert_eq!(pq.del_max(), Some(1));
        assert_eq!(pq.del_max(), Some(-57));
        assert_eq!(pq.del_max(), None);
        assert!(pq.is_empty());
    }

    #[test]
    fn maxpq_from_array() {
        let array = [1, 5, 2, 80, 4, -57];
        let mut pq = MaxPQ::from(array);
        assert_eq!(pq.max(), Some(&80));
        assert_eq!(pq.len(), 6);
        assert_eq!(pq.del_max(), Some(80));
        assert_eq!(pq.del_max(), Some(5));
        assert_eq!(pq.del_max(), Some(4));
        assert_eq!(pq.del_max(), Some(2));
        assert_eq!(pq.del_max(), Some(1));
        assert_eq!(pq.del_max(), Some(-57));
        assert_eq!(pq.del_max(), None);
        assert!(pq.is_empty());
    }

    #[test]
    fn maxpq_clone_into_iter() {
        let array = [1, 5, 2, 80, 4, -57];
        let pq = MaxPQ::from(array);
        let mut itr = pq.clone().into_iter();
        assert_eq!(itr.next(), Some(80));
        assert_eq!(itr.next(), Some(5));
        assert_eq!(itr.next(), Some(4));
        assert_eq!(itr.next(), Some(2));
        assert_eq!(itr.next(), Some(1));
        assert_eq!(itr.next(), Some(-57));
        assert_eq!(itr.next(), None);
    }
}
