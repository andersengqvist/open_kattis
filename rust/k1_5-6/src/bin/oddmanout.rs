use std::collections::HashMap;
use std::io;

fn main() -> Result<(), std::io::Error> {
    let mut first_line = String::new();
    io::stdin().read_line(&mut first_line)?;
    let num_cases = match first_line.trim_end().parse::<i32>() {
        Ok(num) => num,
        _ => 0
    };

    for case in 1..num_cases+1 {
        test_case(case)?;
    }

    Ok(())
}

fn test_case(case_number: i32) -> Result<(), std::io::Error> {
    let mut unused = String::new();
    io::stdin().read_line(&mut unused)?;

    let mut guests = String::new();
    io::stdin().read_line(&mut guests)?;

    let v: Vec<u32> = guests.trim_end()
                            .split(" ")
                            .map(|n: &str| match n.parse::<u32>() {
                                Ok(num) => num,
                                _ => 0
                            })
                            .collect();

    println!("Case #{}: {}", case_number, find_odd(&v));

    Ok(())
}

fn find_odd(guests: &Vec<u32>) -> u32 {
    let mut map: HashMap<u32, bool> = HashMap::new();

    for guest in guests {
        if map.contains_key(guest) {
            map.remove(guest);
        }
        else {
            map.insert(*guest, true);
        }
    }
    *map.keys().collect::<Vec<&u32>>()[0]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_guest_1() {
        let guests: Vec<u32> = vec![
            1, 2147483647, 2147483647
        ];
        assert_eq!(find_odd(&guests), 1);
    }

    #[test]
    fn test_find_guest_7() {
        let guests: Vec<u32> = vec![
            3, 4, 7, 4, 3
        ];
        assert_eq!(find_odd(&guests), 7);
    }

    #[test]
    fn test_find_guest_5() {
        let guests: Vec<u32> = vec![
            2, 10, 2, 10, 5
        ];
        assert_eq!(find_odd(&guests), 5);
    }

}
