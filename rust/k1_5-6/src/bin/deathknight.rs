use std::io;

fn main() -> Result<(), std::io::Error> {
    let mut first_line = String::new();
    io::stdin().read_line(&mut first_line)?;
    let num_battles = match first_line.trim_end().parse::<i32>() {
        Ok(num) => num,
        _ => 0
    };

    let mut battles_won = 0;
    for _ in 0..num_battles {
        if load_check_battle()? {
            battles_won = battles_won + 1;
        }
    }

    println!("{}", battles_won);

    Ok(())
}

fn load_check_battle() -> Result<bool, std::io::Error> {
    let mut battle = String::new();
    io::stdin().read_line(&mut battle)?;

    Ok(!battle.contains("CD"))
}
