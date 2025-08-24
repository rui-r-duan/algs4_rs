use algs4_rs::threesum;
use std::time::{Duration, Instant};

fn time_trial(n: usize) -> Duration {
    const MAXIMUM_INTEGER: i32 = 1_000_000;
    let mut a: Vec<i32> = vec![0; n];
    for i in 0..n {
        a[i] = rand::random_range(-MAXIMUM_INTEGER..MAXIMUM_INTEGER);
    }
    let now = Instant::now();
    let _ = threesum::count(&a);
    now.elapsed()
}

fn main() {
    let mut n = 250;
    loop {
        let time: Duration = time_trial(n);
        println!("{:7} {:.1}", n, time.as_secs_f64());
        n += n;
    }
}
