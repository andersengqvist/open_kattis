use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let cases = line.trim_end().parse::<i32>()?;

    for _ in 0..cases {
        line.clear();
        io::stdin().read_line(&mut line)?;
        let v: Vec<&str> = line.trim_end().split(" ").collect();
        let b = v[0].parse::<f64>()?;
        let p = v[1].parse::<f64>()?;

        println!("{} {} {}", (b - 1.0) * 60.0 / p, b * 60.0 / p, (b + 1.0) * 60.0 / p);
    }

    Ok(())
}