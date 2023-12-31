#[cfg(debug_assertions)]
use std::fs::File;
#[cfg(debug_assertions)]
use std::io::Write;
#[cfg(debug_assertions)]
use std::io::Error;

use std::path::Path;
use std::fs;

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

    // sum up all numbers that don't have a symbol adjacent to it
    fn sum_of_part_numbers(&self) -> u64 {
        let mut sum = 0;
        let mut seen = vec![vec![false; self.grid[0].len()]; self.grid.len()];

        for (i, row) in self.grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                // if decimal digit and not seen yet
                if cell.is_digit(10) && !seen[i][j] {
                    let number = self.extract_number(i, j, &mut seen);
                    if self.is_number_adjacent_to_symbol(i, j, &number) {
                        #[cfg(debug_assertions)]
                        println!("{}", number.parse::<u64>().unwrap());
                        sum += number.parse::<u64>().unwrap();
                    }
                }
            }
        }
        sum
    }

    // Implement the method to extract the full number starting from a digit
    // Also, mark the cells of this number as seen to avoid double-counting
    fn extract_number(&self, start_row: usize, start_col: usize, seen: &mut Vec<Vec<bool>>) -> String {
        let mut number = String::new();
        let mut col = start_col;
        let grid_width = self.grid[start_row].len();
        #[cfg(debug_assertions)]
        println!("{} {} {}", start_row, start_col, grid_width);

        while col < grid_width && self.grid[start_row][col].is_digit(10) {
            seen[start_row][col] = true;
            number.push(self.grid[start_row][col]);
            col += 1;
        }

        #[cfg(debug_assertions)]
        println!("{}", number);
        number
    }

    // Implement the method to check if a number is adjacent to a symbol so we won't sum it
    fn is_number_adjacent_to_symbol(&self, start_row: usize, start_col: usize, number: &str) -> bool {
        let row_len = self.grid[start_row].len();
        let grid_height = self.grid.len();
        let num_length = number.len();
        #[cfg(debug_assertions)]
        println!("numlength: {}", num_length);

        // Define relative coordinates for all adjacent cells
        let neighbors = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1), (1, 0), (1, 1)
        ];

        // Iterate through each character in the number
        for offset in 0..num_length {
            let col = start_col + offset;
            #[cfg(debug_assertions)]
            println!("col: {}", col);

            // Iterate through all possible neighbors
            for (dx, dy) in &neighbors {
                let new_row = (start_row as isize + dy) as usize;
                let new_col = (col as isize + dx) as usize;

                // Check if the new coordinates are within bounds of the grid
                if new_row < grid_height && new_col < row_len {
                    // Check for special symbols
                    let symbol = self.grid[new_row][new_col];
                    if symbol.is_ascii_punctuation() && symbol != '.' {
                        #[cfg(debug_assertions)]
                        println!("{} {} {}", symbol, "is adjacent to a ", number);

                        return true;
                    }
                }
            }
        }
        false
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

    // Call the sum_of_part_numbers method
    let score = schematic.sum_of_part_numbers();

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

    // calculate all the points
    // create grid to find points that needs to be deducted
    // deduct points from all points
    #[test]
    fn test_game_points() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

        // Create an instance of EngineSchematic
        let schematic = EngineSchematic::new(input);

        // Call the sum_of_part_numbers method
        let score = schematic.sum_of_part_numbers();

        let expected = 4361; // 467+35+633+617+592+755+664+598+58+114-58-114 = 4361
        assert_eq!(score, expected);
    }
}
