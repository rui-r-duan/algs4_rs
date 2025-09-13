use algs4_rs::Algs4Error;
use algs4_rs::StdIn;
use algs4_rs::heap_sort;

fn show<T: std::fmt::Display>(a: &[T]) {
    for x in a {
        println!("{x}");
    }
}

fn main() -> Result<(), Algs4Error> {
    let mut stdin = StdIn::new();
    let mut a: Vec<String> = stdin.read_all_strings()?;
    heap_sort(&mut a);
    show(&a);
    Ok(())
}
