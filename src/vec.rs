use raw_vec::{RawValIter, RawVec};
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;

pub(crate) mod raw_vec;

/// A Simple Vector.  Inspired by the Vec in [The
/// Rustonomicon](https://doc.rust-lang.org/nomicon/vec/vec.html), with some differences.
///
/// The nomicon version does not have the `shrink` allocation, while this implementation does.
/// **For the nitty-gritty, please read The Rustonomicon.**
pub struct SVec<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> SVec<T> {
    /// Create an empty `SVec` which does not allocate any memory.
    pub fn new() -> Self {
        SVec {
            buf: RawVec::new(),
            len: 0,
        }
    }

    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    fn cap(&self) -> usize {
        self.buf.cap
    }

    /// Appends an element to the back of a collection.  The value of variable `elem` is moved
    /// into this `SVec` so that this `SVec` owns it.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` *bytes*.
    ///
    /// # Time complexity
    ///
    /// Takes amortized *O*(1) time.
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() {
            self.buf.grow();
        }

        unsafe {
            ptr::write(self.ptr().add(self.len), elem);
        }

        // Can't fail, we'll OOM first.
        self.len += 1;
    }

    /// Removes and returns the element most recently added to this `SVec`, or `None` if this `SVec`
    /// is empty.
    ///
    /// # Time complexity
    ///
    /// Takes &Theta;(1) time.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let elem = unsafe { Some(ptr::read(self.ptr().add(self.len))) };
            if self.len == self.buf.cap / 4 {
                self.buf.shrink();
            }
            elem
        }
    }

    /// Inserts an element at position `index` with the vector, shifting all elements after it to
    /// the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    ///
    /// # Time complexity
    ///
    /// Takes *O*(`SVec::len`) time.  All items after the insertion index must be shifted to the
    /// right.  In the worst case, all elements are shifted when the insertion index is 0.
    pub fn insert(&mut self, index: usize, elem: T) {
        // Note: `<=` because it's valid to insert after everything which would be equivalent to
        // push.
        assert!(index <= self.len, "index out of bounds");
        if self.len == self.cap() {
            self.buf.grow();
        }

        unsafe {
            // ptr::copy(src, dest, len): "copy from src to dest len elems"
            ptr::copy(
                self.ptr().add(index),
                self.ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.ptr().add(index), elem);
        };

        self.len += 1;
    }

    /// Removes and returns the element at position `index` within the vector, shifting all elements
    /// after it to the left.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// # Time complexity
    ///
    /// Takes *O*(*n*) time.  Because of this shifts over the remaining elements, it has a
    /// worst-case performance of *O*(*n*).
    pub fn remove(&mut self, index: usize) -> T {
        // Note: `<` because it's *not* valid to remove after everything
        assert!(index < self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr().add(index));
            ptr::copy(
                self.ptr().add(index + 1),
                self.ptr().add(index),
                self.len - index,
            );
            result
        }
    }

    /// Removes the whole slice of the whole vector, returning a double-ended iterator over the
    /// removed slice.
    ///
    /// If the iterator is dropped before being fully consumed, it drops the remaining removed
    /// elements.
    ///
    /// The returned iterator keeps a mutable borrow on the vector to optimize its implementation.
    ///
    /// # Leaking
    ///
    /// If the returned iterator goes out of scope without being dropped (due to `mem::forget`, for
    /// example), the vector may have lost and leaked elements arbitrarily, including elements
    /// outside the range.
    pub fn drain(&mut self) -> Drain<'_, T> {
        let iter = unsafe { RawValIter::new(&self) };

        // This is mem::forget safety thing.  If Drain is forgotton, we just
        // leak the whole SVec's contents.  Also we need to do this *eventualy*
        // anyway, so why not do it now?
        self.len = 0;

        Drain {
            iter,
            vec: PhantomData,
        }
    }
}

impl<T> Drop for SVec<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
        // deallocation is handled by RawVec
    }
}

/// Coerce `Vec<T>` to slice `[T]`.  A slice provides all sorts of bells and whistles such as `len`,
/// `first`, `last`, indexing, slicing, sorting, `iter`, `iter_mut`, etc.
impl<T> Deref for SVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.len) }
    }
}

/// Coerce `Vec<T>` to slice `[T]`.  A slice provides all sorts of bells and whistles such as `len`,
/// `first`, `last`, indexing, slicing, sorting, `iter`, `iter_mut`, etc.
impl<T> DerefMut for SVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr(), self.len) }
    }
}

pub struct SVecIntoIter<T> {
    _buf: RawVec<T>, // we don't actually care about this, just need it to live
    iter: RawValIter<T>,
}

impl<T> IntoIterator for SVec<T> {
    type Item = T;
    type IntoIter = SVecIntoIter<T>;
    fn into_iter(self) -> SVecIntoIter<T> {
        let (iter, buf) = unsafe { (RawValIter::new(&self), ptr::read(&self.buf)) };
        mem::forget(self);

        SVecIntoIter { iter, _buf: buf }
    }
}

impl<T> Iterator for SVecIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for SVecIntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<T> Drop for SVecIntoIter<T> {
    fn drop(&mut self) {
        // only need to ensure all our elements are read, and thus their destructors are called;
        // buffer will clean itself up afterwards.
        for _ in &mut *self {}
    }
}

/// A draining iterator for [`SVec`].
pub struct Drain<'a, T: 'a> {
    vec: PhantomData<&'a mut SVec<T>>,
    iter: RawValIter<T>,
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

impl<T: Clone> Clone for SVec<T> {
    fn clone(&self) -> Self {
        let mut v = SVec {
            buf: self.buf.clone(),
            len: 0,
        };
        for elem in self.iter() {
            v.push(elem.clone());
        }
        v
    }
}

impl<T: fmt::Debug> fmt::Debug for SVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, x) in self.iter().enumerate() {
            fmt::Debug::fmt(x, f)?;
            if i < self.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

impl<T> std::iter::FromIterator<T> for SVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> SVec<T> {
        let mut v = SVec::new();
        for x in iter {
            v.push(x);
        }
        v
    }
}

impl<T> Default for SVec<T> {
    fn default() -> Self {
        SVec::new()
    }
}

/// Similar to macro [`std::vec!`].
///
/// Unlike array expressions, form `svec![T; N]` requires that `T` implements `Clone` trait instead
/// of `Copy` which is required by an array, and `N` does not have to be a constant.
#[macro_export]
macro_rules! svec {
    () => (
        $crate::vec::SVec::new()
    );
    ($elem:expr; $n:expr) => ({
        let mut v = $crate::vec::SVec::new();
        for _i in 0..$n {
            v.push($elem.clone());
        }
        v
    });
    ($($x:expr),+ $(,)?) => ({
        let mut v = $crate::vec::SVec::new();
        $(
            v.push($x);
        )*
        v
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_basics() {
        let mut v: SVec<&str> = SVec::new();
        v.push("hello");
        v.push("algs4");
        v.push("rs");
        v.push("lib");
        assert_eq!(v.len(), 4);
        assert_eq!(v.pop(), Some("lib"));
        assert_eq!(&v[2], &"rs");
        assert_eq!(v.pop(), Some("rs"));
        let mut itr = v.into_iter();
        assert_eq!(itr.next_back(), Some("algs4"));
        assert_eq!(itr.size_hint(), (1, Some(1)));
        assert_eq!(itr.next(), Some("hello"));
        assert_eq!(itr.next_back(), None);
        assert_eq!(itr.next(), None);
    }

    /// Compared to `std::vec::Vec`, our implementation is more strict.
    ///
    /// The following example code will not compile.  But it is OK.  We do not allow such use.
    ///
    /// Making it compile requires relaxing the drop checker's conservative assumption which does
    /// not allow `T` (`&str` in this example) to dangle.
    ///
    /// To relax the borrow checking condition, we need to use the unstable feature: attribute
    /// `#[may_dangle]`, which does not compile in stable Rust.
    ///
    /// ```compile_fail,E0597
    /// fn test_may_dangle() {
    ///     let mut v: SVec<&str> = SVec::new();
    ///     let s: String = "Short-lived".into();
    ///     v.push(&s); // borrowed value &s does not live long enough
    ///     drop(s);
    /// }
    /// ```
    fn _doc_test() {}

    #[derive(Debug, Eq, PartialEq)]
    struct ZST;

    impl std::fmt::Display for ZST {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(f, "ZST")
        }
    }

    #[test]
    fn test_vec_zst() {
        let mut v: SVec<ZST> = SVec::new();
        v.push(ZST);
        v.push(ZST);
        v.push(ZST);
        v.push(ZST);
        assert_eq!(v.len(), 4);
        assert_eq!(v.pop(), Some(ZST));
        assert_eq!(v[0], ZST);
        let mut itr = v.into_iter();
        assert_eq!(itr.size_hint(), (3, Some(3)));
        assert_eq!(itr.next(), Some(ZST));

        let mut v1 = SVec::new();
        v1.push(ZST);
        v1.push(ZST);
        let mut drainer = v1.drain();
        assert_eq!(drainer.size_hint(), (2, Some(2)));
        assert_eq!(drainer.next_back(), Some(ZST));
        assert_eq!(drainer.next(), Some(ZST));
        assert_eq!(drainer.next_back(), None);
    }

    #[test]
    fn test_vec_clone() {
        let mut v: SVec<String> = SVec::new();
        v.push(String::from("memcpy"));
        v.push(String::from("memmove"));
        v.push(String::from("diff"));

        let u = v.clone();
        assert_eq!(u[0], v[0]);
        assert_eq!(u[1], v[1]);
        assert_eq!(u[2], v[2]);
        assert_eq!(u.len(), v.len());
    }

    #[test]
    fn test_vec_macro() {
        let v: SVec<f64> = svec![];
        assert_eq!(v.len(), 0);
        assert_eq!(format!("{:?}", v), "[]");

        let v = svec![2, 3, 4];
        let s = format!("{:?}", v);
        assert_eq!(s, "[2, 3, 4]");

        let v = svec!["no"; 0];
        assert_eq!(v.len(), 0);

        let non_const = 4;
        let v = svec!["no"; non_const];
        assert_eq!(v.len(), non_const);
        assert_eq!(
            v.iter().cloned().collect::<Vec<_>>(),
            ["no", "no", "no", "no"]
        );

        let v = svec![
            "Rustonomicon".to_string(),
            "dark".to_string(),
            "magic".to_string()
        ];
        assert_eq!(v.len(), 3);
        assert_eq!(
            v.iter().cloned().collect::<Vec<_>>(),
            ["Rustonomicon", "dark", "magic"]
        );
    }

    #[test]
    fn test_vec_from_iterator() {
        let mut v: SVec<&str> = SVec::default();
        v.push("aaa");
        v.push("bbb");

        let x: SVec<&str> = v.iter().cloned().collect();

        //----------------------------------------------------------------
        // The following two lines do not compile.
        // Because `assert_eq` macro does `match (&$left, &$right)` and `(*left_val == *right_val)`.
        // So `==` is applied on the left `$lhs` and the right `$rhs`.
        //
        // `std::vec::Vec` implements a lot of `PartialEq<$rhs> for $lhs` using an internal
        // macro `__impl_slice_eq1` to achieve the ergonomic.
        //----------------------------------------------------------------
        // assert_eq!(&x, &["aaa", "bbb"]);  // &SVec<&str> == &[&str; 2]
        // assert_eq!(x, ["aaa", "bbb"]);    // SVec<&str> == [&str; 2]

        assert_eq!(x[..], ["aaa", "bbb"][..]); // [&str] == [&str]

        fn str_slice_eq(a: &[&str], b: &[&str]) -> bool {
            a == b
        }
        str_slice_eq(&x, &["aaa", "bbb"]);

        let y: Vec<&str> = v.iter().cloned().collect();
        assert_eq!(y, ["aaa", "bbb"],); // Vec<&str> == [&str; 2]
    }
}
