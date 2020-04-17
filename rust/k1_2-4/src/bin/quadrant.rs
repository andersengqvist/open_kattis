use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let x = line.trim_end().parse::<i32>()?;
    line.clear();
    io::stdin().read_line(&mut line)?;
    let y = line.trim_end().parse::<i32>()?;

    if x > 0 {
        if y > 0 {
            println!("1");
        }
        else {
            println!("4");
        }
    }
    else {
        if y > 0 {
            println!("2");
        }
        else {
            println!("3");
        }
    }

    Ok(())
}