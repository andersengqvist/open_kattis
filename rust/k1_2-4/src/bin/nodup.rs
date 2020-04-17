use std::collections::HashSet;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let words: Vec<&str> = line.trim_end().split(" ").collect();

    let mut seen_words: HashSet<&str> = HashSet::new();

    for word in words.iter() {
        if seen_words.contains(word) {
            println!("no");
            return Ok(())
        }
        seen_words.insert(*word);
    }

    println!("yes");

    Ok(())
}