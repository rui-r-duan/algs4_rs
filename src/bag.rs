//! A generic bag or multiset, implemented using a singly linked list.

pub mod linkedbag;
pub mod vecbag;

pub use linkedbag::*;
pub use vecbag::*;

#[cfg(test)]
mod tests;
