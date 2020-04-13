use std::collections::HashSet;
use std::io;

fn main() -> Result<(), std::io::Error> {
    let mut word = String::new();
    io::stdin().read_line(&mut word)?;
    let mut permutation = String::new();
    io::stdin().read_line(&mut permutation)?;

    if guess(word.trim_end(), permutation.trim_end()) {
        println!("WIN");
    }
    else {
        println!("LOSE");
    }

    Ok(())
}

fn guess(word: &str, permutation: &str) -> bool {

    let mut word_chars: HashSet<char> = word.chars().collect::<HashSet<char>>();

    let mut wrong_guesses = 0;

    for c in permutation.chars() {
        if word_chars.contains(&c) {
            word_chars.remove(&c);
            if word_chars.is_empty() {
                return true
            }
        }
        else {
            wrong_guesses = wrong_guesses + 1;
            if wrong_guesses >= 10 {
                return false;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_hangman() {
        assert_eq!(guess("HANGMAN", "ABCDEFGHIJKLMNOPQRSTUVWXYZ"), true);
    }

    #[test]
    fn test_guess_banana() {
        assert_eq!(guess("BANANA", "ABCDEFGHIJKLMNOPQRSTUVWXYZ"), false);
    }

    #[test]
    fn test_guess_rainbows() {
        assert_eq!(guess("RAINBOWS", "USIANBVLOJRKWXZCTQGHPFMYDE"), true);
    }

}