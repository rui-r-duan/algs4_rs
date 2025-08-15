use algs4::scanner::Scanner;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut scanner = Scanner::new(std::io::stdin().lock());

    print!("Type a string: ");
    std::io::stdout().flush()?;
    let s = scanner.next()?;
    println!("Your string was: {}", s);
    println!();

    print!("Type an i32: ");
    std::io::stdout().flush()?;
    let a = scanner.next_i32()?;
    println!("Your int was: {}", a);
    println!();

    print!("Type a bool (case-insensitive valid forms: \"1\", \"0\", \"true\", \"false\"): ");
    std::io::stdout().flush()?;
    let b = scanner.next_bool()?;
    println!("Your bool was: {}", b);
    println!();

    print!("Type an invalid bool: ");
    std::io::stdout().flush()?;
    match scanner.next_bool() {
        Ok(b) => {
            println!("got {}", b);
            println!();
        }
        Err(e) => {
            println!("{e}");	// You can also try `println!("{e:?}");`.
	    // If we do not consume the invalid-bool token, then
	    // the following `next_f64` would consume the token, which
	    // yields an error.
            let t = scanner.next()?;
            println!("Consuming (fetching) the next token ... it was {}", t);
	    println!();
        }
    }

    print!("Type an f64, optaionally followed by whitespaces and some words: ");
    std::io::stdout().flush()?;
    let c = scanner.next_f64()?;
    println!("Your f64 was: {}", c);
    println!();

    println!("Consuming the remaining of your input line ...");
    println!(
        "The remaining of your input line was (without the trailing Linefeed if any): \"{}\"",
        scanner.next_line()?
    );
    println!();

    print!("Type a line: ");
    std::io::stdout().flush()?;
    let d = scanner.next_line()?;
    println!("Your line was: {}", d);
    println!();

    println!(
        "Type a few words separated by ASCII whitespaces (U+0009 TAB, U+000A LF, U+000C FF, U+000D CR, or U+0020 SPACE) and terminated with EOF (on Linux or Mac Ctrl+D, on Windows Ctrl+Z): "
    );
    loop {
        let hasnext = scanner.has_next()?;
        if !hasnext {
            break;
        }
        let t = scanner.next()?;
        println!("Got token: {}", t);
    }

    Ok(())
}
