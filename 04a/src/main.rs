use std::collections::HashSet;

// parse a card into a tuple vector
fn parse_card(card: &str) -> Option<(HashSet<usize>, Vec<usize>)> {
    let mut parts = card.split(" | ");
    let winning_numbers = parts.next()?
        .split_whitespace()
        .filter_map(|num| num.parse::<usize>().ok())
        .collect::<HashSet<usize>>();
    let player_numbers = parts.next()?
        .split_whitespace()
        .filter_map(|num| num.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    Some((winning_numbers, player_numbers))
}


// add points only after each card
fn calculate_card_points(winning_numbers: &HashSet<usize>, player_numbers: &[usize]) -> u32 {
    let mut points = 0;
    let mut matches = 0;

    for &number in player_numbers {
        if winning_numbers.contains(&number) {
            matches += 1;
            // Only add points if there are matches
            if matches > 0 {
                points = 2u32.pow(matches - 1);
            }
        }
    }
    points
}

fn calculate_score(input: &str) -> u32 {
    let mut score = 0;

    for card in input.split('\n') {
        if let Some((winning_numbers, player_numbers)) = parse_card(card) {
            score += calculate_card_points(&winning_numbers, &player_numbers);
        }
    }

    score
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok(); // Load .env file
    let input_file_path = "output/input.txt";
    let url = "https://adventofcode.com/2023/day/4/input";
    let cookie = std::env::var("SESSION_COOKIE").expect("SESSION_COOKIE not set in .env file");
    let body = aoc2023::get_data(url, cookie, input_file_path).await?;

    let score = calculate_score(&body);

    #[cfg(debug_assertions)]
    {
        let sum_string = score.to_string();
        aoc2023::save_to_file("output/output.txt", &sum_string)?;
    }

    println!("{}", score);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    /*
    * Game: Calculate points for scratchcards.
    * Structure: Each card has winning numbers and player's numbers.
    * Points: points += 2u32.pow(matches - 1);
    * Total: Add points from all cards for total score.
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
        let expected = 13; // 2*2*2 + 2 + 2 + 1 = 13
        let score = calculate_score(input);
        assert_eq!(score, expected);
    }
}
