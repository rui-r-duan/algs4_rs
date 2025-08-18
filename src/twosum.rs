//! Take n integers and counts the number of pairs that sum to exactly 0.
//!
//! ### Limitations
//! - We ignore integer overflow

/// O(n^2)
pub fn print_all(a: &[i32]) {
    let n = a.len();
    for i in 0..n {
        for j in i + 1..n {
            if a[i] + a[j] == 0 {
                println!("{} {}", a[i], a[j]);
            }
        }
    }
}

/// O(n^2)
pub fn count(a: &[i32]) -> i32 {
    let n = a.len();
    let mut count = 0;
    for i in 0..n {
        for j in i + 1..n {
            if a[i] + a[j] == 0 {
                count += 1;
            }
        }
    }
    count
}
