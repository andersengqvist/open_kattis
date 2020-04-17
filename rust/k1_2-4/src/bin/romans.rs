use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let a = line.trim_end().parse::<f64>()?;
    let b = a * 5280.0 / 4.854;

    println!("{}", b.round() as i64);

    Ok(())
}