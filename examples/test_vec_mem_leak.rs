//! This program is a template to test memory leak.
//!
//! ```
//! cargo build --example test_vec_mem_leak
//! valgrind --leak-check=full target/debug/examples/test_vec_mem_leak
//! ```

fn main() {
    // In a `algs4_rs::vec::Vec<T>`, the internal buffer should be freed, all the elements in the
    // Vec should be freed.

    // Use `algs4_rs::vec::Vec<&str>` to test the deallocation of internal buffer.

    // use `algs4_rs::vec::Vec<String>` to test the deallocation of String elements.

    // use `algs4_rs::vec::Vec<String>::into_iter()` to test the proper memory deallocation of
    // `algs4_rs::vec::IntoIter<String>`.

    let mut v: algs4_rs::vec::Vec<String> = algs4_rs::vec::Vec::new();
    v.push("hello".to_string());
    v.push("algs4".to_string());
    v.push("rs".to_string());
    v.push("lib".to_string());

    v.pop();

    let mut itr = v.into_iter();
    itr.next_back();
}
