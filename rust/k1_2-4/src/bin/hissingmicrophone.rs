use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    if line.contains("ss") {
        println!("hiss");
    }
    else {
        println!("no hiss");
    }

    Ok(())
}