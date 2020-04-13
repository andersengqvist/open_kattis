use std::io;

fn main() -> Result<(), std::io::Error> {
    let mut line = String::new();

    loop {
        line.clear();
        io::stdin().read_line(&mut line)?;

        let m = match line.trim_end().parse::<u32>() {
            Ok(num) => num,
            _ => 0
        };

        if m == 0 {
            break;
        }

        println!("{}", find_p(m));
    }
    Ok(())
}

fn find_p(n: u32) -> u32 {
    let sum_n = sum_of_digits(n);
    let mut p: u32 = 11;
    loop {
        let sum_p = sum_of_digits(p * n);
        if sum_p == sum_n {
            return p;
        }
        p = p + 1;
    }
}

fn sum_of_digits(num: u32) -> u32 {
    let mut n: u32 = num;
    let mut result: u32 = 0;
    while n != 0 {
        result = result + n % 10;
        n = n / 10;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_3029() {
        assert_eq!(sum_of_digits(3029), 14);
    }

    #[test]
    fn test_sum_of_78754() {
        assert_eq!(sum_of_digits(78754), 31);
    }

    #[test]
    fn test_find_3029() {
        assert_eq!(find_p(3029), 37);
    }

    #[test]
    fn test_find_4() {
        assert_eq!(find_p(4), 28);
    }

    #[test]
    fn test_find_5() {
        assert_eq!(find_p(5), 28);
    }

    #[test]
    fn test_find_42() {
        assert_eq!(find_p(42), 25);
    }

}
