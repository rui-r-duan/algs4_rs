//! Take n integers and counts the number of pairs that sum to exactly 0.
//!
//! ### Limitations
//! - We ignore integer overflow

use crate::error::InvalidArgument;

/// O(n log n)
pub fn print_all(a: &mut [i32]) -> Result<(), InvalidArgument> {
    let n = a.len();
    a.sort_unstable();
    if contains_duplicates(a) {
        return Err(InvalidArgument(
            "slice contains duplicate integers".to_string(),
        ));
    }
    for i in 0..n {
        if let Ok(j) = a.binary_search(&-a[i])
            && j > i
        {
            println!("{} {}", a[i], a[j]);
        }
    }
    Ok(())
}

/// O(n log n)
pub fn count(a: &mut [i32]) -> Result<i32, InvalidArgument> {
    let n = a.len();
    a.sort_unstable();
    if contains_duplicates(a) {
        return Err(InvalidArgument(
            "slice contains duplicate integers".to_string(),
        ));
    }
    let mut count = 0;
    for i in 0..n {
        if let Ok(j) = a.binary_search(&-a[i])
            && j > i
        {
            count += 1;
        }
    }
    Ok(count)
}

// pre: `a` is sorted
fn contains_duplicates(a: &[i32]) -> bool {
    for i in 1..a.len() {
        if a[i] == a[i - 1] {
            return true;
        }
    }
    false
}
