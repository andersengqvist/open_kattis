use std::collections::HashSet;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let cases = line.trim_end().parse::<i32>()?;

    for _ in 0..cases {
        line.clear();
        io::stdin().read_line(&mut line)?;

        let trips = line.trim_end().parse::<i32>()?;

        let mut seen_places = HashSet::new();
        for _ in 0..trips {
            let mut place = String::new();
            io::stdin().read_line(&mut place)?;
            seen_places.insert(place);
        }
        println!("{}", seen_places.len());
    }

    Ok(())
}