pub fn print_all(a: &[i32]) {
    let n = a.len();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if a[i] + a[j] + a[k] == 0 {
                    println!("{} {} {}", a[i], a[j], a[k]);
                }
            }
        }
    }
}

pub fn count(a: &[i32]) -> i32 {
    let n = a.len();
    let mut count = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if a[i] + a[j] + a[k] == 0 {
                    count += 1;
                }
            }
        }
    }
    count
}
