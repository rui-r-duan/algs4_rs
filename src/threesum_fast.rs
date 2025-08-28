//! Take n integers and counts the number of triples that sum to exactly 0.
//!
//! ### Limitations
//! - We ignore integer overflow

use crate::error::InvalidArgument;

/// O(n^2 log n)
pub fn print_all(a: &mut [i32]) -> Result<(), InvalidArgument> {
    let n = a.len();
    a.sort_unstable();
    if contains_duplicates(a) {
        return Err(InvalidArgument(
            "slice contains duplicate integers".to_string(),
        ));
    }
    for i in 0..n {
        for j in i + 1..n {
            if let Ok(k) = a.binary_search(&-(a[i] + a[j])) {
                if k > j {
                    println!("{} {} {}", a[i], a[j], a[k]);
                }
            }
        }
    }
    Ok(())
}

/// O(n^2 log n)
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
        for j in i + 1..n {
            if let Ok(k) = a.binary_search(&-(a[i] + a[j])) {
                if k > j {
                    count += 1;
                }
            }
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
