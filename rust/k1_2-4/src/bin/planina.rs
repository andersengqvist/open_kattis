use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim_end().parse::<u64>() {
                Ok(iter) => {
                    println!("{}", points(iter));
                }
                Err(e) => eprintln!("Failed to parse: {}", e)
            }
        }
        Err(e) => eprintln!("Failed to read line: {}", e)
    }
}

fn side(iter: u64) -> u64 {
    let mut side: u64 = 2;
    for _ in 0..iter {
        side = side + side - 1;
    }
    side
}

fn points(iter: u64) -> u64 {
    let side = side(iter);
    side * side
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_side_zero_iter() {
        assert_eq!(side(0u64), 2u64);
    }

    #[test]
    fn test_side_one_iter() {
        assert_eq!(side(1u64), 3u64);
    }

    #[test]
    fn test_side_two_iter() {
        assert_eq!(side(2u64), 5u64);
    }

    #[test]
    fn test_side_three_iter() {
        assert_eq!(side(3u64), 9u64);
    }

    #[test]
    fn test_side_four_iter() {
        assert_eq!(side(4u64), 17u64);
    }

    #[test]
    fn test_points_zero_iter() {
        assert_eq!(points(0u64), 4u64);
    }

    #[test]
    fn test_points_one_iter() {
        assert_eq!(points(1u64), 9u64);
    }

    #[test]
    fn test_points_two_iter() {
        assert_eq!(points(2u64), 25u64);
    }

    #[test]
    fn test_points_three_iter() {
        assert_eq!(points(3u64), 81u64);
    }

    #[test]
    fn test_points_five_iter() {
        assert_eq!(points(5u64), 1089u64);
    }

}
