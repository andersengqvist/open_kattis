use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let stones = line.trim_end().parse::<i32>()?;

    if stones % 2 == 0 {
        println!("Bob");
    }
    else {
        println!("Alice");
    }

    Ok(())
}
