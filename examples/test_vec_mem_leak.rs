//! This program is a template to test memory leak.
//!
//! ```
//! cargo build --example test_vec_mem_leak
//! valgrind --leak-check=full target/debug/examples/test_vec_mem_leak
//! ```

fn main() {
    // In a `algs4_rs::vec::SVec<T>`, the internal buffer should be freed, all the elements in the
    // SVec should be freed.

    // Use `algs4_rs::vec::SVec<&str>` to test the deallocation of internal buffer.

    // use `algs4_rs::vec::SVec<String>` to test the deallocation of String elements.

    // use `algs4_rs::vec::SVec<String>::into_iter()` to test the proper memory deallocation of
    // `algs4_rs::vec::IntoIter<String>`.

    // Feel free to modify `algs_rs::vec::SVec<T>`'s memory allocation/deallocation related code to
    // test cases of memory leak.

    let mut v: algs4_rs::SVec<String> = algs4_rs::SVec::new();
    v.push("hello".to_string());
    v.push("algs4".to_string());
    v.push("rs".to_string());
    v.push("lib".to_string());

    v.pop();

    {
        let mut drainer = v.drain();

        // Pull out an element and immediately drop it.
        assert_eq!(drainer.next(), Some("hello".to_string()));

        // Get rid of drainer, but don't call its destructor
        std::mem::forget(drainer);
    }

    // Rust's memory-safety only prevents invalid memory access which causes segmentation faults, it
    // does not 100% prevent memory leaks, because strictly speaking, "leaking" is unpreventable.
    //
    // [Leaking - The Rustonomicon](https://doc.rust-lang.org/nomicon/leaking.html#drain.)
    // introduces this Drainer case and a few other memory leaking cases.
    //
    // [Memory safety in Rust - CS 242: Programmign Languages, Fall
    // 2018](https://stanford-cs242.github.io/f18/lectures/05-1-rust-memory-safety.html) classifies
    // the memory properties of programs into two categories:
    //
    // - Memory safety
    // - Memory containment (related to memory leaking)
    println!("{}", v[0]); // panic: index out of bounds: the len is 0 but the index is 0
}
