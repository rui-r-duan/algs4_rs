//! A generic bag or multiset, implemented using a singly linked list.

pub mod linkedbag;
pub mod resizingbag;

pub use linkedbag::*;
pub use resizingbag::*;

#[cfg(test)]
mod tests;
