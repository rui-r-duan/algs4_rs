//! Last-in-first-out (LIFO) stack of generic items.

pub mod linkedstack;
pub mod resizingstack;

pub use linkedstack::*;
pub use resizingstack::*;

#[cfg(test)]
mod tests;
