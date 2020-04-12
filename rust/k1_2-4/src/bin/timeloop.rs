use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim_end().parse::<i32>() {
                Ok(iterations) => {
                    for iter in 1..iterations+1 {
                        println!("{} Abracadabra", iter);
                    }
                }
                Err(e) => eprintln!("Failed to parse: {}", e)
            }
        }
        Err(e) => eprintln!("Failed to read line: {}", e)
    }
}