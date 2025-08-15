//! Binary search for a sorted sequence without duplicates.

use std::cmp::Ordering;

/// Returns the index of the specified key in the specified sequence,
/// or -1 if not found.
pub fn index_of<T: Ord>(a: &[T], key: &T) -> i64 {
    let (mut lo, mut hi) = (0, a.len() as i64 - 1);
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let cmp = key.cmp(&a[mid as usize]);
        if cmp == Ordering::Less {
            hi = mid - 1;
        } else if cmp == Ordering::Greater {
            lo = mid + 1;
        } else {
            return mid;
        }
    }
    -1
}

/// Returns the index of the specified key in the specified sequence,
/// or -1 if not found.
pub fn index_of_i32_seq(a: &[i32], key: &i32) -> i64 {
    let (mut lo, mut hi) = (0, a.len() as i64 - 1);
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if *key < a[mid as usize] {
            hi = mid - 1;
        } else if *key > a[mid as usize] {
            lo = mid + 1;
        } else {
            return mid;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_of() {
        let a = ["1", "3", "5", "7", "9"];

        let i = index_of(&a, &"5");
        assert_eq!(i, 2);

        let i = index_of(&a, &"4");
        assert_eq!(i, -1);

	// empty list
	let a = [];
	let i = index_of(&a, &"5");
	assert_eq!(i, -1);

        // If there are duplicated elements in the sequence,
        // the result is undefined.
        let a = ["1", "3", "5", "5", "5", "7", "9"];
        let i = index_of(&a, &"5");
        assert!(match i {
            2..=4 => true,
            _ => false,
        });
    }

    #[test]
    fn test_index_of_i32_seq() {
        let b = [0, 1, 2, 3, 5, 8, 13, 21, 34, 55];

        let i = index_of_i32_seq(&b, &13);
        assert_eq!(i, 6);

        let i = index_of_i32_seq(&b, &4);
        assert_eq!(i, -1);

        let i = index_of_i32_seq(&b, &100);
        assert_eq!(i, -1);

        let i = index_of_i32_seq(&b, &1);
        assert_eq!(i, 1);

	// empty list
	let a = [];
	let i = index_of(&a, &"5");
	assert_eq!(i, -1);

        // If there are duplicated elements in the sequence,
        // the result is undefined.
        let b = [1, 3, 5, 5, 5, 7, 9];
        let i = index_of_i32_seq(&b, &5);
        assert!(match i {
            2..=4 => true,
            _ => false,
        });
    }
}
