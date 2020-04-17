use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    line.clear();
    io::stdin().read_line(&mut line)?;
    let expenses: i64 = line.trim_end()
                            .split(" ")
                            .map(|w| match w.parse::<i64>() {
                                Ok(num) => num,
                                _ => 0
                            })
                            .filter(|n| *n < 0)
                            .map(|n| -n)
                            .sum();

    println!("{}", expenses);

    Ok(())
}