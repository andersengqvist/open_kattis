use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let v: Vec<&str> = line.trim_end().split(" ").collect();
    let x = v[0].parse::<i32>()?;
    let y = v[1].parse::<i32>()?;
    let n = v[2].parse::<i32>()?;

    for i in 1..n + 1 {
        if i % x == 0 {
            if i % y == 0 {
                println!("FizzBuzz");
            }
            else {
                println!("Fizz");
            }
        }
        else if i % y == 0 {
            println!("Buzz");
        }
        else {
            println!("{}", i);
        }
    }

    Ok(())
}