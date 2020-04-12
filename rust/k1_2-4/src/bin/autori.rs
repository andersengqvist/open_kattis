use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("{}", shorten(&input));
        }
        Err(_) => ()
    }
}

fn shorten(names: &str) -> String {
    let v: Vec<&str> = names.split("-").map(|n| &n[0..1]).collect();
    v.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knp() {
        assert_eq!(shorten("Knuth-Morris-Pratt"), "KMP");
    }

    #[test]
    fn test_ms() {
        assert_eq!(shorten("Mirko-Slavko"), "MS");
    }

    #[test]
    fn test_pp() {
        assert_eq!(shorten("Pasko-Patak"), "PP");
    }

    #[test]
    fn test_a() {
        assert_eq!(shorten("Anders"), "A");
    } 
}
