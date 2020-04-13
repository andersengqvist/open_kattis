use std::io;

fn main() -> Result<(), std::io::Error> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;


    let v: Vec<i32> = line.trim_end()
                          .split(" ")
                          .map(|n: &str| match n.parse::<i32>() {
                              Ok(num) => num,
                              _ => 0
                          })
                          .collect();

    println!("{}", slurp(v[0] + v[1], v[2]));

    Ok(())
}

fn slurp(init_bottles: i32, cost: i32) -> i32 {
    let mut sodas = 0;

    let mut bottles = init_bottles;
    while bottles >= cost {
        let new_sodas = bottles / cost; // By new sodas
        bottles = bottles % cost; // Remove bottles used for buying new soda
        sodas = sodas + new_sodas; // Drink the soda
        bottles = bottles + new_sodas; // Add those new bottles to the collection
    }

    sodas
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slurp_9() {
        assert_eq!(slurp(9, 3), 4);
    }

    #[test]
    fn test_slurp_10() {
        assert_eq!(slurp(10, 2), 9);
    }
}