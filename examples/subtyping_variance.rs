/*!
 This is a template to help test the varaince of a generic type, especially of a container type
 `F<T>`.

 As long as this program compiles, the test passes, which means `F<T>` is covariant over `T`.

 Covariance means:

   U is a subtype of T
     =>
   F<U> is a subtype of F<T>

 See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance.

 In this example, `F<T>` is `LinkedQueue<T>`.
 Feel free to change it to anything to test their varaince.  For example:
   - std::cell::Cell<T>  (does not compile, because it is not covariant over T)
   - std::vec::Vec<T>
   - algs4_rs::SVec<T>
   - algs4_rs::ResizingQueue<T>
   - algs4_rs::LinkedStack<T>

 `examples/linkedqueue_mut_pointer.rs` provides a simple implementation for `LinkedQueue` using
 mut pointers.  You can use that `LinkedQueue` here to see that it is NOT covariant over T.

 Example compile error:
 ----------------------------------------------------------------
   error: lifetime may not live long enough
     --> algs4_rs/examples/subtyping_variance.rs:47:4
      |
   46 | fn _two_refs<'short, 'long: 'short>(a: F<&'short str>, b: F<&'long str>) {
      |              ------  ----- lifetime `'long` defined here
      |              |
      |              lifetime `'short` defined here
   47 |    _take_two(a, b);
      |    ^^^^^^^^^^^^^^^ argument requires that `'short` must outlive `'long`
      |
      = help: consider adding the following bound: `'short: 'long`
      = note: requirement occurs because of the type `Cell<&str>`, which makes the generic argument `&str` invariant
      = note: the struct `Cell<T>` is invariant over the parameter `T`
      = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
 ----------------------------------------------------------------
*/

use algs4_rs::LinkedQueue;

type _F<T> = LinkedQueue<T>;

fn _two_refs<'short, 'long: 'short>(a: _F<&'short str>, b: _F<&'long str>) {
    _take_two(a, b);
}
fn _take_two<T>(_val1: T, _val2: T) {}

fn main() {}
