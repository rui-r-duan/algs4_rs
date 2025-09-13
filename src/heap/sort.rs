/// Sorts a slice using <em>heapsort</em>.
///
/// This implementation takes &Theta;(<em>n</em> log <em>n</em>) time to sort any array of length
/// <em>n</em> (assuming comparisons take constant time).  It makes at most 2 <em>n</em>
/// log<sub>2</sub> <em>n</em> compares.
///
/// This sorting algorithm is not stable.
///
/// It uses &Theta;(1) extra memory (not including the input array).
///
/// For additional documentation, see <a href="https://algs4.cs.princeton.edu/24pq">Section 2.4</a>
/// of <i>Algorithms, 4th Edition</i> by Robert Sedgewick and Kevin Wayne.
pub fn heap_sort<T: Ord>(pq: &mut [T]) {
    let n = pq.len();

    // heapify phase
    for k in (1..=(n / 2)).rev() {
        sink(pq, k, n);
    }

    // sortdown phase
    let mut k = n;
    while k > 1 {
        exch(pq, 1, k);
        k -= 1;
        sink(pq, 1, k);
    }
}

fn sink<T: Ord>(pq: &mut [T], mut k: usize, n: usize) {
    while 2 * k <= n {
        let mut j = 2 * k;
        if j < n && less(pq, j, j + 1) {
            j += 1;
        }
        if !less(pq, k, j) {
            break;
        }
        exch(pq, k, j);
        k = j;
    }
}

fn less<T: Ord>(pq: &[T], i: usize, j: usize) -> bool {
    pq[i - 1].cmp(&pq[j - 1]).is_lt()
}

fn exch<T>(pq: &mut [T], i: usize, j: usize) {
    pq.swap(i - 1, j - 1);
}
