use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let addends = line.trim_end().parse::<i32>()?;

    let mut output: u64 = 0;
    for _ in 0..addends {
        line.clear();
        io::stdin().read_line(&mut line)?;
        let s: &str = line.trim_end();
        // Rust strings are hard: codepoints this and graphemes that
        // Luckily we are dealing with ascii here
        let i: u64 = s[0..s.len()-1].parse::<u64>()?;
        let p: u32 = s[s.len()-1..].parse::<u32>()?;

        output = output + i.pow(p);
    }

    println!("{}", output);

    Ok(())
}