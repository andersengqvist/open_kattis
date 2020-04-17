use std::io;

// Not sure how the Box dyn thingie works, and Intellij complains
// But lets me handle all errors with the ?
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let data = line.trim_end().parse::<i32>()?;

    line.clear();
    io::stdin().read_line(&mut line)?;
    let months = line.trim_end().parse::<i32>()?;

    let mut used = 0;
    for _ in 0..months {
        line.clear();
        io::stdin().read_line(&mut line)?;
        used = used + line.trim_end().parse::<i32>()?;
    }

    // Add one because we are calculating data for next month
    println!("{}", data * (months + 1) - used);

    Ok(())
}
