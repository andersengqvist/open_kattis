use std::collections::HashMap;
use std::io;

fn main() -> Result<(), std::io::Error> {
    let mut first_line = String::new();
    io::stdin().read_line(&mut first_line)?;
    let r: Vec<&str> = first_line.trim_end().split(" ").collect();
    let num_cards = match r[0].parse::<i32>() {
        Ok(num) => num * 4,
        _ => 0
    };
    let dominant_suit = r[1];

    let mut cards: Vec<String> = Vec::new();
    for _ in 0..num_cards {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        cards.push(line)
    }

    let points = points(&cards, dominant_suit);
    println!("{}", points);

    Ok(())
}

fn points(cards: &Vec<String>, dominant_suit: &str) -> u32 {

    let mut dominant: HashMap<&str, u32> = HashMap::new();
    dominant.insert("A", 11);
    dominant.insert("K", 4);
    dominant.insert("Q", 3);
    dominant.insert("J", 20);
    dominant.insert("T", 10);
    dominant.insert("9", 14);

    let mut not_dominant: HashMap<&str, u32> = HashMap::new();
    not_dominant.insert("A", 11);
    not_dominant.insert("K", 4);
    not_dominant.insert("Q", 3);
    not_dominant.insert("J", 2);
    not_dominant.insert("T", 10);

    let mut result: u32 = 0;
    for card in cards {
        let value: &str = &card[0..1]; // What am I doing here? A slice of a slice, why is it a &str?
        let suit: &str = &card[1..2];
        if suit == dominant_suit {
            match dominant.get(value) {
                Some(&number) => {
                    result = result + number
                },
                _ => ()
            }
        } else {
            match not_dominant.get(value) {
                Some(&number) => {
                    result = result + number
                },
                _ => ()
            }
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_60() {
        let cards = vec![
            String::from("TH"),
            String::from("9C"),
            String::from("KS"),
            String::from("QS"),
            String::from("JS"),
            String::from("TD"),
            String::from("AD"),
            String::from("JH")
        ];
        assert_eq!(points(&cards, "S"), 60);
    }

    #[test]
    fn test_92() {
        let cards = vec![
            String::from("AH"),
            String::from("KH"),
            String::from("QH"),
            String::from("JH"),
            String::from("TH"),
            String::from("9H"),
            String::from("8H"),
            String::from("7H"),
            String::from("AS"),
            String::from("KS"),
            String::from("QS"),
            String::from("JS"),
            String::from("TS"),
            String::from("9S"),
            String::from("8S"),
            String::from("7S")
        ];
        assert_eq!(points(&cards, "H"), 92);
    }

}
