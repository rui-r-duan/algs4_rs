use crate::error::InvalidArgument;
use std::cmp::Ordering;
use std::collections::VecDeque;

type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
struct Node<K, V> {
    key: K,            // sorted by key
    val: V,            // associated data
    left: Link<K, V>,  // left subtree
    right: Link<K, V>, // right subtree
    size: usize,       // number of nodes in subtree
}

impl<K, V> Node<K, V> {
    fn new(key: K, val: V, size: usize) -> Self {
        Node {
            key,
            val,
            left: None,
            right: None,
            size,
        }
    }
}

/// An ordered symbol table of generic key-value pairs, implemented with a binary search tree.
///
/// It supports the usual `put`, `get`, `contains`, `delete`, `size`, and `is-empty` methods.  It
/// also provides ordered methods for finding the `minimum`, `maximum`, `floor`, `select`,
/// `ceiling`.  It also provides a `keys` method for iterating over all of the keys.
///
/// A symbol table implements the *associative array* abstraction: when associating a value with a
/// key that is already in the symbol table, the convention is to replace the old value with the new
/// value.
///
/// This implementation uses an (unbalanced) *binary search tree*.
///
/// The `put`, `contains`, `remove`, `minimum`, `maximum`, `ceiling`, `floor`, `select`, and `rank`
/// operations each take &Theta;(<em>n</em>) time in the worst case, where `n` is the number of
/// key-value pairs.
///
/// The `size` and `is-empty` operations take &Theta;(1) time.
///
/// The `keys` method takes &Theta;(<em>n</em>) time in the worst case.
///
/// Construction takes &Theta;(1) time.
///
/// For alternative implementations of the symbol table API, see {@link ST}, {@link BinarySearchST},
/// {@link SequentialSearchST}, {@link RedBlackBST}, {@link SeparateChainingHashST}, and {@link
/// LinearProbingHashST}, For additional documentation, see <a
/// href="https://algs4.cs.princeton.edu/32bst">Section 3.2</a> of <i>Algorithms, 4th Edition</i> by
/// Robert Sedgewick and Kevin Wayne.
#[derive(Debug)]
pub struct BST<K, V> {
    root: Link<K, V>,
}

impl<K, V> BST<K, V>
where
    K: Ord,
{
    /// Initialize an empty symbol table.
    pub fn new() -> Self {
        BST { root: None }
    }

    /// Returns true if this symbol table is empty, returns false otherwise.
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// Returns the number of key-value pairs in this symbol table.
    pub fn size(&self) -> usize {
        size(self.root.as_ref())
    }

    /// Does this symbol table contain the given key?
    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Returns the value associated with the given key.
    pub fn get(&self, key: &K) -> Option<&V> {
        get(self.root.as_ref(), key)
    }

    /// Inserts the specified key-value pair into the symbol table, overwriting the old value with
    /// the new value if the symbol table already contains the specified key.
    pub fn put(&mut self, key: K, val: V) {
        self.root = put(self.root.take(), key, val);
        debug_assert!(self.check());
    }

    /// Removes the smallest key and associated value from the symbol table.
    pub fn delete_min(&mut self) -> Result<(), InvalidArgument> {
        if self.is_empty() {
            return Err(InvalidArgument("symbol table underflow".to_string()));
        }
        let (t, _deleted) = delete_min(self.root.take().unwrap());
        self.root = t;
        debug_assert!(self.check());
        Ok(())
    }

    /// Removes the largest key and associated value from the symbol table.
    pub fn delete_max(&mut self) -> Result<(), InvalidArgument> {
        if self.is_empty() {
            return Err(InvalidArgument("symbol table underflow".to_string()));
        }
        let (t, _deleted) = delete_max(self.root.take().unwrap());
        self.root = t;
        debug_assert!(self.check());
        Ok(())
    }

    /// Removes the specified key and its associated value from this symbol table (if the key is in
    /// this symbol table).
    pub fn delete(&mut self, key: &K) {
        self.root = delete(self.root.take(), key);
        debug_assert!(self.check());
    }

    /// Returns the smallest key in the symbol table.
    pub fn min(&self) -> Option<&K> {
        if self.is_empty() {
            None
        } else {
            Some(&min(self.root.as_ref().unwrap()).key)
        }
    }

    /// Returns the largest key in the symbol table.
    pub fn max(&self) -> Option<&K> {
        if self.is_empty() {
            None
        } else {
            Some(&max(self.root.as_ref().unwrap()).key)
        }
    }

    /// Returns the largest key in the symbol table less than or equal to `key`.
    pub fn floor(&self, key: &K) -> Option<&K> {
        floor(self.root.as_ref(), key).map(|x| &x.key)
    }

    pub fn floor2(&self, key: &K) -> Option<&K> {
        floor2(self.root.as_ref(), key, None)
    }

    /// Returns the smallest key in the symbol table greater than or equal to `key`.
    pub fn ceiling(&self, key: &K) -> Option<&K> {
        ceiling(self.root.as_ref(), key).map(|x| &x.key)
    }

    /// Returns the key in the symbol table of a given `rank`.
    ///
    /// This key has the property that there are `rank` keys in the symbol table that are smaller.
    /// In other words, this key is the (`rank+1`)st smallest key in the symbol table.
    ///
    /// If `rank >= n` where `n` is the size of this BST, return `InvalidArgument`.
    pub fn select(&self, rank: usize) -> Result<Option<&K>, InvalidArgument> {
        if rank >= self.size() {
            return Err(InvalidArgument(format!(
                "argument to select() is invalid: {}",
                rank
            )));
        }
        Ok(select(self.root.as_ref(), rank))
    }

    /// Returns the number of keys in the symbol table strictly less than `key`.
    pub fn rank(&self, key: &K) -> usize {
        rank(key, self.root.as_ref())
    }

    /// Returns an iterator over the keys in the symbol table in ascending order.
    ///
    /// Note: this iterator is lazy but not pure lazy.  See [Keys].
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys::new(&self.root)
    }

    /// Returns an iterator over all keys in the symbol table in the given range.
    /// `lo` and `hi` are inclusive.
    ///
    /// The iterator implements DoubleEndedIterator.
    ///
    /// Note: this iterator is eager (not lazy at all).  See [KeysRange].
    pub fn keys_range(&self, lo: &K, hi: &K) -> KeysRange<'_, K> {
        KeysRange::new(&self.root, lo, hi)
    }

    /// Returns the number of keys in the symbol table in the given range.
    pub fn size_range(&self, lo: &K, hi: &K) -> usize {
        if lo.cmp(hi) == Ordering::Greater {
            0
        } else if self.contains(hi) {
            self.rank(hi) - self.rank(lo) + 1
        } else {
            self.rank(hi) - self.rank(lo)
        }
    }

    /// Returns the height of the BST (for debugging).
    /// A 1-node tree has height 0.
    pub fn height(&self) -> isize {
        height(&self.root)
    }

    /// Returns an iterator over the keys in the BST in level order (for debugging).
    ///
    /// Note: this iterator is eager (not lazy at all).  See [KeysLevelOrder].
    pub fn keys_level_order(&self) -> KeysLevelOrder<'_, K> {
        KeysLevelOrder::new(&self.root)
    }

    fn check(&self) -> bool {
        let a = self.is_bst();
        if !a {
            eprintln!("Not in symmetric order");
        }
        let b = self.is_size_consistent();
        if !b {
            eprintln!("Subtree counts not consistent");
        }
        let c = self.is_rank_consistent();
        if !c {
            eprintln!("Ranks not consistent");
        }
        a && b
    }

    fn is_bst(&self) -> bool {
        is_bst(&self.root, None, None)
    }

    fn is_size_consistent(&self) -> bool {
        is_size_consistent(&self.root)
    }

    fn is_rank_consistent(&self) -> bool {
        for i in 0..self.size() {
            let rk = self.rank(
                self.select(i)
                    .expect("cannot fail")
                    .expect("cannot be None"),
            );
            if i != rk {
                return false;
            }
        }
        for k in self.keys() {
            let k2 = self
                .select(self.rank(k))
                .expect("cannot fail")
                .expect("cannot be None");
            if k.cmp(k2).is_ne() {
                return false;
            }
        }
        true
    }
}

impl<K, V> Default for BST<K, V>
where
    K: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

fn size<K, V>(x: Option<&Box<Node<K, V>>>) -> usize {
    x.map_or(0, |n| n.size)
}

fn get<'a, K: Ord, V>(x: Option<&'a Box<Node<K, V>>>, key: &K) -> Option<&'a V> {
    match x {
        None => None,
        Some(node) => match key.cmp(&node.key) {
            Ordering::Equal => Some(&node.val),
            Ordering::Less => get(node.left.as_ref(), key),
            Ordering::Greater => get(node.right.as_ref(), key),
        },
    }
}

fn put<K: Ord, V>(x: Link<K, V>, key: K, val: V) -> Link<K, V> {
    match x {
        None => Some(Box::new(Node::new(key, val, 1))),
        Some(mut n) => {
            match key.cmp(&n.key) {
                Ordering::Less => {
                    n.left = put(n.left, key, val);
                }
                Ordering::Greater => {
                    n.right = put(n.right, key, val);
                }
                Ordering::Equal => {
                    n.val = val;
                }
            }
            n.size = 1 + size(n.left.as_ref()) + size(n.right.as_ref());
            Some(n)
        }
    }
}

// Returns: (new_root, deleted_node)
fn delete_min<K: Ord, V>(mut x: Box<Node<K, V>>) -> (Link<K, V>, Box<Node<K, V>>) {
    match x.left {
        None => (x.right.take(), x),
        Some(left) => {
            let (t, deleted) = delete_min(left);
            x.left = t;
            x.size = size(x.left.as_ref()) + size(x.right.as_ref()) + 1;
            (Some(x), deleted)
        }
    }
}

// Returns: (new_root, deleted_node)
fn delete_max<K: Ord, V>(mut x: Box<Node<K, V>>) -> (Link<K, V>, Box<Node<K, V>>) {
    match x.right {
        None => (x.left.take(), x),
        Some(right) => {
            let (t, deleted) = delete_max(right);
            x.right = t;
            x.size = size(x.left.as_ref()) + size(x.right.as_ref()) + 1;
            (Some(x), deleted)
        }
    }
}

// Returns new_root
fn delete<K: Ord, V>(x: Link<K, V>, key: &K) -> Link<K, V> {
    match x {
        None => None,
        Some(mut node) => {
            match key.cmp(&node.key) {
                Ordering::Less => {
                    node.left = delete(node.left, key);
                }
                Ordering::Greater => {
                    node.right = delete(node.right, key);
                }
                Ordering::Equal => {
                    if node.right.is_none() {
                        return node.left;
                    }
                    if node.left.is_none() {
                        return node.right;
                    }
                    let t = node; // node `t` is to be deleted
                    let (right, right_min) = delete_min(t.right.unwrap());
                    node = right_min; // right_min is the successor of node `t`
                    node.right = right;
                    node.left = t.left;
                }
            };
            node.size = size(node.left.as_ref()) + size(node.right.as_ref()) + 1;
            Some(node)
        }
    }
}

fn min<K, V>(x: &Box<Node<K, V>>) -> &Box<Node<K, V>> {
    if x.left.is_none() {
        x
    } else {
        min(x.left.as_ref().unwrap())
    }
}

fn max<K, V>(x: &Box<Node<K, V>>) -> &Box<Node<K, V>> {
    if x.right.is_none() {
        x
    } else {
        max(x.right.as_ref().unwrap())
    }
}

fn floor<'a, K: Ord, V>(x: Option<&'a Box<Node<K, V>>>, key: &K) -> Option<&'a Box<Node<K, V>>> {
    x?;
    let y = x.unwrap();
    match key.cmp(&y.key) {
        Ordering::Equal => Some(y),
        Ordering::Less => floor(y.left.as_ref(), key),
        Ordering::Greater => {
            let t = floor(y.right.as_ref(), key);
            if t.is_some() { t } else { Some(y) }
        }
    }
}

fn floor2<'a, K: Ord, V>(
    x: Option<&'a Box<Node<K, V>>>,
    key: &K,
    best: Option<&'a K>,
) -> Option<&'a K> {
    if x.is_none() {
        return best;
    }
    let y = x.unwrap();
    match key.cmp(&y.key) {
        Ordering::Equal => Some(&y.key),
        Ordering::Less => floor2(y.left.as_ref(), key, best),
        Ordering::Greater => floor2(y.right.as_ref(), key, Some(&y.key)),
    }
}

fn ceiling<'a, K: Ord, V>(x: Option<&'a Box<Node<K, V>>>, key: &K) -> Option<&'a Box<Node<K, V>>> {
    x?;
    let y = x.unwrap();
    match key.cmp(&y.key) {
        Ordering::Equal => Some(y),
        Ordering::Greater => ceiling(y.right.as_ref(), key),
        Ordering::Less => {
            let t = ceiling(y.left.as_ref(), key);
            if t.is_some() { t } else { Some(y) }
        }
    }
}

// Returns key in BST rooted at x of given rank.
// Precondition: rank is in legal range.
fn select<K, V>(x: Option<&Box<Node<K, V>>>, rank: usize) -> Option<&K> {
    x?;
    let y = x.unwrap();
    let left_size = size(y.left.as_ref());
    if left_size > rank {
        select(y.left.as_ref(), rank)
    } else if left_size < rank {
        select(y.right.as_ref(), rank - left_size - 1)
    } else {
        Some(&y.key)
    }
}

// Number of keys in the subtree less than key.
fn rank<K: Ord, V>(key: &K, x: Option<&Box<Node<K, V>>>) -> usize {
    if x.is_none() {
        return 0;
    }
    let y = x.unwrap();
    match key.cmp(&y.key) {
        Ordering::Equal => size(y.left.as_ref()),
        Ordering::Less => rank(key, y.left.as_ref()),
        Ordering::Greater => 1 + size(y.left.as_ref()) + rank(key, y.right.as_ref()),
    }
}

/// Iterator over all the keys of the given BST.
///
/// This iterator is lazy but not pure lazy.  It consumes part of the tree nodes initially, and then
/// as more `next` are called, it consumes more tree nodes group by group.  "Consume" means it
/// allocates memory to store the consumed keys.  In some implementations of other programming
/// languages, for example, Java, the Iterable is eager, which means that **all** the keys are
/// consumed when the iterator is created, that is, the iterator allocates memory to store all the
/// keys.  As a comparison, [`std::collections::BTreeMap`] in Rust standard library has a pure lazy
/// implementation of `keys` method, which means the iterator does nothing unless consumed.
pub struct Keys<'a, K, V> {
    stack: Vec<&'a Node<K, V>>,
}

impl<'a, K: Ord, V> Keys<'a, K, V> {
    fn new(root: &'a Link<K, V>) -> Self {
        let mut iter = Keys { stack: Vec::new() };
        iter.push_left_branch(root);
        iter
    }

    fn push_left_branch(&mut self, mut node: &'a Link<K, V>) {
        while let Some(n) = node {
            self.stack.push(n.as_ref());
            node = &n.left;
        }
    }
}

impl<'a, K: Ord, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    // in-order traversal
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        let key = &node.key;
        self.push_left_branch(&node.right);
        Some(key)
    }
}

/// Iterator over all the keys of the BST in the given range.
///
/// This iterator is eager (not lazy at all).  When the iterator is created, it consumes all the
/// tree nodes and stores all the keys in the iterator itself.
pub struct KeysRange<'a, K> {
    queue: VecDeque<&'a K>,
}

impl<'a, K: Ord> KeysRange<'a, K> {
    fn new<'b, V>(root: &'a Link<K, V>, lo: &'b K, hi: &'b K) -> Self {
        let mut iter = KeysRange {
            queue: VecDeque::new(),
        };
        keys(&root, &mut iter.queue, lo, hi);
        iter
    }
}

impl<'a, K: Ord> Iterator for KeysRange<'a, K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front()
    }
}

impl<'a, K: Ord> DoubleEndedIterator for KeysRange<'a, K> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.queue.pop_back()
    }
}

fn keys<'a, 'b, K: Ord, V>(x: &'a Link<K, V>, queue: &mut VecDeque<&'a K>, lo: &'b K, hi: &'b K) {
    match x {
        None => {
            return;
        }
        Some(y) => {
            let cmplo = lo.cmp(&y.key);
            let cmphi = hi.cmp(&y.key);
            if cmplo == Ordering::Less {
                keys(&y.left, queue, lo, hi);
            }
            if (cmplo == Ordering::Less || cmplo == Ordering::Equal)
                && (cmphi == Ordering::Greater || cmphi == Ordering::Equal)
            {
                queue.push_back(&y.key);
            }
            if cmphi == Ordering::Greater {
                keys(&y.right, queue, lo, hi);
            }
        }
    }
}

fn height<K, V>(x: &Link<K, V>) -> isize {
    match x {
        None => -1,
        Some(y) => 1 + height(&y.left).max(height(&y.right)),
    }
}

/// Iterator over all the keys of the BST in level order.
///
/// This iterator is eager (not lazy at all).  When the iterator is careted, it consumes all the
/// tree nodes and stores all the keys in the iterator itself.
pub struct KeysLevelOrder<'a, K> {
    queue: VecDeque<&'a K>,
}

impl<'a, K: Ord> KeysLevelOrder<'a, K> {
    fn new<V>(root: &'a Link<K, V>) -> Self {
        let mut iter = KeysLevelOrder {
            queue: VecDeque::new(),
        };
        let mut node_queue: VecDeque<&'a Link<K, V>> = VecDeque::new();
        node_queue.push_back(&root);
        while !node_queue.is_empty() {
            let x = node_queue.pop_front().unwrap();
            if x.is_none() {
                continue;
            }
            let y = x.as_ref().unwrap();
            iter.queue.push_back(&y.key);
            node_queue.push_back(&y.left);
            node_queue.push_back(&y.right);
        }
        iter
    }
}

impl<'a, K: Ord> Iterator for KeysLevelOrder<'a, K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front()
    }
}

fn is_bst<K: Ord, V>(x: &Link<K, V>, min: Option<&K>, max: Option<&K>) -> bool {
    if let Some(y) = x {
        if let Some(min_val) = min
            && y.key.cmp(min_val).is_le()
        {
            false
        } else if let Some(max_val) = max
            && y.key.cmp(max_val).is_ge()
        {
            false
        } else {
            is_bst(&y.left, min, Some(&y.key)) && is_bst(&y.right, Some(&y.key), max)
        }
    } else {
        true
    }
}

fn is_size_consistent<K, V>(x: &Link<K, V>) -> bool {
    if let Some(y) = x {
        if y.size != size(y.left.as_ref()) + size(y.right.as_ref()) + 1 {
            false
        } else {
            is_size_consistent(&y.left) && is_size_consistent(&y.right)
        }
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prepare_1() -> BST<String, usize> {
        let mut st = BST::new();
        let v = "SEARCHEXAMPLE".split("").collect::<Vec<&str>>(); // vec!["", "S", "E", ... "L", "E", ""]
        for (i, x) in v[1..v.len() - 1].iter().enumerate() {
            st.put(x.to_string(), i);
        }
        // (S 0
        //    (E 12
        //       (A 8
        //          nil
        //          (C 4 nil nil))
        //       (R 3
        //          (H 5
        //             nil
        //             (M 9
        //                (L 11 nil nil)
        //                (P 10 nil nil)))
        //          nil))
        //    (X 7 nil nil))
        st
    }

    fn prepare_2() -> BST<char, usize> {
        let mut st = BST::new();
        for (i, x) in "SEARCHEXAMPLE".chars().enumerate() {
            st.put(x, i);
        }
        // (S 0
        //    (E 12
        //       (A 8
        //          nil
        //          (C 4 nil nil))
        //       (R 3
        //          (H 5
        //             nil
        //             (M 9
        //                (L 11 nil nil)
        //                (P 10 nil nil)))
        //          nil))
        //    (X 7 nil nil))
        st
    }

    #[test]
    fn test_bst_put_and_keys() {
        let st1 = prepare_1();
        assert_eq!(
            st1.keys().map(|s| s.as_str()).collect::<String>(),
            "ACEHLMPRSX"
        );

        let st2 = prepare_2();
        assert_eq!(st2.keys().collect::<String>(), "ACEHLMPRSX");
    }

    #[test]
    fn test_bst_size() {
        let st = prepare_2();
        assert!(!st.is_empty());
        assert_eq!(st.size(), 10);
    }

    #[test]
    fn test_bst_contains_and_get() {
        let st = prepare_2();
        assert!(st.contains(&'X'));
        assert!(!st.contains(&'Q'));
        assert!(!st.contains(&'a'));

        assert_eq!(st.get(&'X'), Some(&7));
        assert_eq!(st.get(&'Q'), None);
        assert_eq!(st.get(&'a'), None);
        assert_eq!(st.get(&'A'), Some(&8));
    }

    #[test]
    fn test_bst_delete_min() {
        let mut empty_st: BST<i32, String> = Default::default();
        let r = empty_st.delete_min();
        assert!(match r {
            Err(InvalidArgument(s)) => s == "symbol table underflow",
            _ => false,
        });

        let mut st = prepare_2();
        assert_eq!(st.keys().collect::<String>(), "ACEHLMPRSX");
        let r = st.delete_min();
        assert!(r.is_ok());
        assert!(!st.contains(&'A'));
    }

    #[test]
    fn test_bst_delete_max() {
        let mut empty_st: BST<i32, String> = Default::default();
        let r = empty_st.delete_max();
        assert!(match r {
            Err(InvalidArgument(s)) => s == "symbol table underflow",
            _ => false,
        });

        let mut st = prepare_2();
        assert_eq!(st.keys().collect::<String>(), "ACEHLMPRSX");
        let r = st.delete_max();
        assert!(r.is_ok());
        assert!(!st.contains(&'X'));
    }

    #[test]
    fn test_bst_delete() {
        let mut empty_st: BST<i32, String> = BST::new();
        empty_st.delete(&80);
        assert!(empty_st.is_empty());

        let mut st = prepare_2();
        assert_eq!(st.keys().collect::<String>(), "ACEHLMPRSX");
        st.delete(&'F');
        assert_eq!(st.keys().collect::<String>(), "ACEHLMPRSX");
        st.delete(&'H');
        assert_eq!(st.keys().collect::<String>(), "ACELMPRSX");
    }

    #[test]
    fn test_bst_min_and_max() {
        let empty_st: BST<i32, String> = BST::new();
        assert!(empty_st.min().is_none());
        assert!(empty_st.max().is_none());

        let st = prepare_2();
        assert_eq!(st.min(), Some(&'A'));
        assert_eq!(st.max(), Some(&'X'));
    }

    #[test]
    fn test_bst_floor_and_ceiling() {
        let empty_st: BST<i32, String> = BST::new();
        assert!(empty_st.floor(&9).is_none());
        assert!(empty_st.ceiling(&20).is_none());

        let st = prepare_2();
        assert_eq!(st.floor(&'A'), Some(&'A'));
        assert_eq!(st.floor(&'B'), Some(&'A'));
        assert_eq!(st.floor2(&'A'), Some(&'A'));
        assert_eq!(st.floor2(&'B'), Some(&'A'));
        assert_eq!(st.ceiling(&'A'), Some(&'A'));
        assert_eq!(st.ceiling(&'B'), Some(&'C'));
        assert_eq!(st.ceiling(&'X'), Some(&'X'));
        assert_eq!(st.ceiling(&'Y'), None);
    }

    #[test]
    fn test_bst_select_and_rank() {
        let empty_st: BST<i32, String> = BST::new();
        assert!(empty_st.select(20).is_err());
        assert!(empty_st.select(0).is_err());
        assert_eq!(empty_st.rank(&-3), 0);
        assert_eq!(empty_st.rank(&0), 0);

        let st = prepare_2();
        let expected_keys = "ACEHLMPRSX";
        assert_eq!(st.keys().collect::<String>(), expected_keys);
        assert_eq!(st.size(), 10);
        for (i, v) in expected_keys.chars().enumerate() {
            assert_eq!(st.select(i).unwrap(), Some(&v));
        }
        assert!(empty_st.select(11).is_err());

        let expected_ranks = [
            ('A', 0),
            ('B', 1),
            ('C', 1),
            ('D', 2),
            ('E', 2),
            ('X', 9),
            ('Z', 10),
        ];
        for (k, r) in expected_ranks {
            assert_eq!(st.rank(&k), r);
        }
    }

    #[test]
    fn test_bst_keys_range() {
        let empty_st: BST<i32, String> = BST::new();
        assert_eq!(empty_st.keys_range(&2, &8).collect::<Vec<&i32>>().len(), 0);

        let st = prepare_2();
        let expected_keys = "ACEHLMPRSX";
        assert_eq!(st.keys_range(&'A', &'Z').collect::<String>(), expected_keys);
        assert_eq!(st.keys_range(&'B', &'Q').collect::<String>(), "CEHLMP");
        assert_eq!(st.keys_range(&'B', &'R').collect::<String>(), "CEHLMPR");
        assert_eq!(st.keys_range(&'A', &'B').collect::<String>(), "A");
        assert_eq!(st.keys_range(&'A', &'A').collect::<String>(), "A");
        assert_eq!(st.keys_range(&'B', &'B').collect::<String>(), "");
        assert_eq!(st.keys_range(&'C', &'A').collect::<String>(), "");

        assert_eq!(st.keys_range(&'C', &'M').rev().collect::<String>(), "MLHEC");

        let mut itr = st.keys_range(&'C', &'N'); // "CEHLM"
        assert_eq!(itr.next(), Some(&'C'));
        assert_eq!(itr.next_back(), Some(&'M'));
        assert_eq!(itr.next(), Some(&'E'));
        assert_eq!(itr.next_back(), Some(&'L'));
        assert_eq!(itr.next(), Some(&'H'));
        assert_eq!(itr.next_back(), None);
        assert_eq!(itr.next(), None);
    }

    #[test]
    fn test_bst_size_range() {
        let empty_st: BST<i32, String> = BST::new();
        assert_eq!(empty_st.size_range(&2, &8), 0);

        let st = prepare_2();
        // expected_keys = "ACEHLMPRSX";
        assert_eq!(st.size_range(&'A', &'Z'), 10);
        assert_eq!(st.size_range(&'B', &'Q'), 6);
        assert_eq!(st.size_range(&'B', &'R'), 7);
        assert_eq!(st.size_range(&'A', &'B'), 1);
        assert_eq!(st.size_range(&'A', &'A'), 1);
        assert_eq!(st.size_range(&'B', &'B'), 0);
        assert_eq!(st.size_range(&'C', &'A'), 0);
    }

    #[test]
    fn test_bst_height() {
        let empty_st: BST<i32, String> = BST::new();
        assert_eq!(empty_st.height(), -1);

        let st = prepare_2();
        assert_eq!(st.height(), 5);
    }

    #[test]
    fn test_bst_keys_level_order() {
        let empty_st: BST<i32, String> = BST::new();
        assert_eq!(empty_st.keys_level_order().collect::<Vec<&i32>>().len(), 0);

        let st = prepare_2();
        assert_eq!(st.keys_level_order().collect::<String>(), "SEXARCHMLP");
    }
}
