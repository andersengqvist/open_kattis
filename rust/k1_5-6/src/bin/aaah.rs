use std::io;

fn main() -> Result<(), std::io::Error> {
    let mut jon = String::new();
    io::stdin().read_line(&mut jon)?;

    let mut doc = String::new();
    io::stdin().read_line(&mut doc)?;

    if jon.len() >= doc.len() {
        println!("go");
    }
    else {
        println!("no");
    }

    Ok(())
}