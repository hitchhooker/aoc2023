#[cfg(debug_assertions)]
use std::fs::File;
#[cfg(debug_assertions)]
use std::io::Write;
#[cfg(debug_assertions)]
use std::io::Error;

use std::path::Path;
use std::fs;
use std::collections::HashSet;

use std::env;
use tokio;
use reqwest;
use dotenv::dotenv;

pub struct EngineSchematic {
    grid: Vec<Vec<char>>,
}

impl EngineSchematic {
    // implement the constructor
    fn new(input: &str) -> EngineSchematic {
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        EngineSchematic { grid }
    }

    // New method to calculate the sum of gear ratios
    fn sum_of_gear_ratios(&self) -> u32 {
        let mut sum = 0;
        let mut seen = vec![vec![false; self.grid[0].len()]; self.grid.len()];

        for (i, row) in self.grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == '*' {
                    // Find and multiply adjacent part numbers
                    if let Some(gear_ratio) = self.find_and_multiply_adjacent_parts(i, j, &mut seen) {
                        sum += gear_ratio;
                    }
                }
            }
        }
        sum
    }

    // Method to find and multiply adjacent part numbers to a '*'
    fn find_and_multiply_adjacent_parts(&self, row: usize, col: usize, seen: &mut Vec<Vec<bool>>) -> Option<u32> {
        let neighbors = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),          (0, 1),
            (1, -1), (1, 0), (1, 1)
        ];
        let mut part_numbers = HashSet::new();

        // we should measure whole length of the number to be around the '*'
        for &(dx, dy) in &neighbors {
            let new_row = (row as isize + dx) as usize;
            let new_col = (col as isize + dy) as usize;

            if new_row < self.grid.len() && new_col < self.grid[0].len() && self.grid[new_row][new_col].is_digit(10) {
                if !seen[new_row][new_col] {
                    let number = self.extract_number(new_row, new_col, seen);
                    part_numbers.insert(number.parse::<u16>().unwrap_or(0));
                }
            }
        }

        let parts = part_numbers.into_iter().collect::<Vec<_>>();
        if parts.len() == 2 {
            #[cfg(debug_assertions)]
            println!("{} {} {} {}", row, col, parts[0], parts[1]);
            Some(parts[0] as u32 * parts[1] as u32)
        } else {
            None
        }
    }

    fn extract_number(&self, row: usize, col: usize, seen: &mut Vec<Vec<bool>>) -> String {
        let mut number = String::new();
        let grid_width = self.grid[row].len();

        // Move to beginning in the matched number string
        let mut current_col = col;
        while current_col > 0 && self.grid[row][current_col - 1].is_digit(10) {
            current_col -= 1;
        }

        // Now move right to extract the entire number string
        while current_col < grid_width && self.grid[row][current_col].is_digit(10) {
            seen[row][current_col] = true;
            number.push(self.grid[row][current_col]);
            current_col += 1;
        }

        number
    }


}


async fn fetch_url(url: &str, cookie: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("Cookie", cookie)
        .send()
        .await?;
    response.text().await
}

#[cfg(debug_assertions)]
fn save_to_file(filename: &str, data: &str) -> Result<(), Error> {
    let mut file = File::create(filename)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file
    let url = "https://adventofcode.com/2023/day/3/input";
    let cookie = env::var("SESSION_COOKIE").expect("SESSION_COOKIE not set in .env file");

    let input_file_path = "output/input.txt";
    let body = if Path::new(input_file_path).exists() {
        #[cfg(debug_assertions)]
        println!("Debug: Input file found locally. Reading...");

        fs::read_to_string(input_file_path)?
    } else {
        #[cfg(debug_assertions)]
        println!("Debug: Downloading input file...");

        let downloaded_body = fetch_url(url, cookie).await?;

        #[cfg(debug_assertions)]
        println!("Debug: File downloaded successfully. Saving...");

        #[cfg(debug_assertions)]
        save_to_file(input_file_path, &downloaded_body)?;

        downloaded_body
    };

    // Create an instance of EngineSchematic
    let schematic = EngineSchematic::new(&body);

    let score = schematic.sum_of_gear_ratios();

    #[cfg(debug_assertions)]
    {
        let sum_string = score.to_string();
        save_to_file("output/output.txt", &sum_string)?;
    }
    println!("{}", score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /* Test case:
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    */
    #[test]
    fn test_game_points() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

        // Create an instance of EngineSchematic
        let schematic = EngineSchematic::new(input);

        // Call the sum_of_part_numbers method
        let score = schematic.sum_of_gear_ratios();

        let expected = 467835; // 467*35 + 598*755
        assert_eq!(score, expected);
    }
}
