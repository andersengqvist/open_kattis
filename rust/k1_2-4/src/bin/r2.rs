use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let v: Vec<&str> = line.trim_end().split(" ").collect();
    let r1 = v[0].parse::<i32>()?;
    let s = v[1].parse::<i32>()?;

    println!("{}", 2 * s - r1);

    Ok(())
}
