use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let periods = line.trim_end().parse::<i32>()?;

    let mut qaly: f64 = 0.0;
    for _ in 0..periods {
        line.clear();
        io::stdin().read_line(&mut line)?;
        let v: Vec<&str> = line.trim_end().split(" ").collect();
        let quality = v[0].parse::<f64>()?;
        let years = v[1].parse::<f64>()?;
        qaly = qaly + quality * years;
    }

    println!("{}", qaly);

    Ok(())
}