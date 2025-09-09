//! Traits for primitive integers and floating point types.

/// A trait that represents all the Rust primivive integers.
///
/// A good alternative is `PrimInt` trait in crate `num_traits`.
pub trait PrimInt {}

impl PrimInt for i8 {}
impl PrimInt for i16 {}
impl PrimInt for i32 {}
impl PrimInt for i64 {}
impl PrimInt for i128 {}
impl PrimInt for isize {}
impl PrimInt for u8 {}
impl PrimInt for u16 {}
impl PrimInt for u32 {}
impl PrimInt for u64 {}
impl PrimInt for u128 {}
impl PrimInt for usize {}

/// A trait that represents all the Rust primivive floating point types.
///
/// A good alternative is `Float` trait in crate `num_traits`.
pub trait PrimFloat {}

impl PrimFloat for f32 {}
impl PrimFloat for f64 {}
