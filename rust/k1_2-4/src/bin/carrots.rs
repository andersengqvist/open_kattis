use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let v: Vec<&str> = line.trim_end().split(" ").collect();
    let solved_problems = v[1].parse::<i32>()?;

    println!("{}", solved_problems);

    Ok(())
}
