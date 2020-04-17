use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let v: Vec<&str> = line.trim_end().split(" ").collect();
    let safety_limit = v[0].parse::<i32>()?;
    let groups = v[1].parse::<i32>()?;

    let mut curr_on_balcony = 0;
    let mut rejected = 0;
    for _ in 0..groups {
        line.clear();
        io::stdin().read_line(&mut line)?;
        let v: Vec<&str> = line.trim_end().split(" ").collect();
        let action = v[0];
        let group = v[1].parse::<i32>()?;

        if action == "enter" {
            if curr_on_balcony + group <= safety_limit {
                curr_on_balcony = curr_on_balcony + group;
            }
            else {
                rejected = rejected + 1;
            }
        }
        else if action == "leave" {
            curr_on_balcony = curr_on_balcony - group;
        }
        else {
            eprintln!("unknown action: {}", action);
        }
    }
    println!("{}", rejected);

    Ok(())
}