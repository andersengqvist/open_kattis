use std::cmp;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let vec: Vec<&str> = line.trim_end().split(" ").collect();
    let n = vec[0].parse::<i32>()?;
    let h = vec[1].parse::<i32>()?;
    let v = vec[2].parse::<i32>()?;

    println!("{}", cmp::max(h, n - h) * cmp::max(v, n - v) * 4);

    Ok(())
}