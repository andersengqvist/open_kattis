use std::io;

fn main() -> Result<(), std::io::Error> {
    let mut sq = String::new();
    io::stdin().read_line(&mut sq)?;

    let area = match sq.trim_end().parse::<f64>() {
        Ok(num) => num,
        _ => 0.0
    };

    println!("{}", area.sqrt() * 4.0);

    Ok(())
}