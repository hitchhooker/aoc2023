use std::collections::HashSet;

#[derive(Clone)]
#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: HashSet<usize>,
    player_numbers: HashSet<usize>,
    matching_numbers: Option<u32>,
}

impl Card {
    fn new(id: usize, winning_numbers: HashSet<usize>, player_numbers: HashSet<usize>) -> Self {
        Self {
            id,
            winning_numbers,
            player_numbers,
            matching_numbers: None,
        }
    }

    fn calculate_matches(&mut self) -> u32 {
        if let Some(matches) = self.matching_numbers {
            return matches;
        }

        let matches = self.player_numbers
            .intersection(&self.winning_numbers)
            .count() as u32;

        self.matching_numbers = Some(matches); // Cache the calculated value
        matches
    }



    fn create_subsequent_copies(&self, cards: &Vec<Card>, matches: u32) -> Vec<Card> {
        let mut new_cards = Vec::new();
        for i in 1..=matches as usize {
            if let Some(subsequent_card) = cards.get(self.id + i - 1) {
                new_cards.push(subsequent_card.clone());
            }
        }
        new_cards
    }

}

fn parse_card(card: &str) -> Option<Card> {
    let mut parts = card.split(" | ");

    // Parsing card ID
    let id_part = parts.next()?;
    let card_id_str = id_part.split_whitespace().nth(1)?.split(':').next()?;
    let card_id = card_id_str.parse::<usize>().ok()?;

    // Parsing player numbers
    let player_numbers = id_part
        .split_whitespace()
        .skip(2) // Skip "Card" and the ID
        .filter_map(|num| num.parse::<usize>().ok())
        .collect::<HashSet<usize>>();

    // Parsing winning numbers
    let winning_numbers = parts.next()?
        .split_whitespace()
        .filter_map(|num| num.parse::<usize>().ok())
        .collect::<HashSet<usize>>();

    Some(Card::new(card_id, winning_numbers, player_numbers))
}


fn parse_cards(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    for card in input.split('\n') {
        if let Some(card) = parse_card(card) {
            cards.push(card);
        }
    }

    cards
}


fn play_cards(mut cards: Vec<Card>) -> Vec<Card> {
    let mut i = 0;
    while i < cards.len() {
        // Clone the card to avoid borrow checker issues
        let mut card = cards[i].clone();
        let matches = card.calculate_matches();

        if matches > 0 {
            let new_cards = card.create_subsequent_copies(&cards, matches);
            cards.extend(new_cards);
        }

        i += 1;
    }
    cards
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok(); // Load .env file
    let input_file_path = "output/input.txt";
    let url = "https://adventofcode.com/2023/day/4/input";
    let cookie = std::env::var("SESSION_COOKIE").expect("SESSION_COOKIE not set in .env file");
    let body = aoc2023::get_data(url, cookie, input_file_path).await?;

    let mut cards = parse_cards(&body);
    cards = play_cards(cards);
    let result = cards.len();

    #[cfg(debug_assertions)]
    {
        let sum_string = result.to_string();
        aoc2023::save_to_file("output/output.txt", &sum_string)?;
    }

    println!("{}", result);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    /*
    *
    * Card 1 has four matching numbers, so you win one copy each of the next four cards: cards 2, 3, 4, and 5.
    * Your original card 2 has two matching numbers, so you win one copy each of cards 3 and 4.
    * Your copy of card 2 also wins one copy each of cards 3 and 4.
    * Your four instances of card 3 (one original and three copies) have two matching numbers, so you win four copies each of cards 4 and 5.
    * Your eight instances of card 4 (one original and seven copies) have one matching number, so you win eight copies of card 5.
    * Your fourteen instances of card 5 (one original and thirteen copies) have no matching numbers and win no more cards.
    * Your one instance of card 6 (one original) has no matching numbers and wins no more cards.
    * Once all of the originals and copies have been processed, you end up with 1 instance of card 1,
    * 2 instances of card 2, 4 instances of card 3, 8 instances of card 4, 14 instances of card 5,
    * and 1 instance of card 6. In total, this example pile of scratchcards causes you to ultimately have 30 scratchcards!
    *
    *   Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    *   Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    *   Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    *   Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    *   Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    *   Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    *
    */
    #[test]
    fn test_game_points() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let expected = 30; // 
        let mut cards = parse_cards(input);
        cards = play_cards(cards);
        let score = cards.len();
        assert_eq!(score, expected);
    }
}
