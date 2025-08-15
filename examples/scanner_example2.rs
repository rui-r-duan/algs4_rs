use algs4::scanner::Scanner;

fn main() -> std::io::Result<()> {
    let mut scanner = Scanner::new(std::io::stdin().lock());

    let mut i = 0;
    loop {
	let hasnext = scanner.has_next_line()?;
	if !hasnext {
	    break;
	}
	let line = scanner.next_line()?;
	println!("{}", line);
	i += 1;
    }
    println!("got {} lines", i);

    Ok(())
}
