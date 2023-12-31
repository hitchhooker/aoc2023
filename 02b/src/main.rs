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

#[derive(Clone)]
struct GameData {
    id: usize,
    red: Vec<usize>,
    green: Vec<usize>,
    blue: Vec<usize>,
}

fn parse_data(input: &str) -> Result<Vec<GameData>, Box<dyn std::error::Error>> {
    let lines: Vec<&str> = input.lines().collect();

    let results: Vec<GameData> = lines
        .iter()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(":").collect();
            let id = parts.get(0)?.split_whitespace().nth(1)?.parse::<usize>().ok()?;
            let segments = parts.get(1)?.split(";");

            let mut red = Vec::new();
            let mut green = Vec::new();
            let mut blue = Vec::new();

            for segment in segments {
                let colors: Vec<&str> = segment.split(",").collect();
                red.push(extract_color_quantity(colors.as_slice(), "red"));
                green.push(extract_color_quantity(colors.as_slice(), "green"));
                blue.push(extract_color_quantity(colors.as_slice(), "blue"));
            }

            Some(GameData { id, red, green, blue })
        })
        .collect();
    Ok(results)
}

fn extract_color_quantity(colors: &[&str], color: &str) -> usize {
    colors.iter()
          .filter_map(|&c| {
              let parts: Vec<&str> = c.trim().split_whitespace().collect();
              if parts.get(1).map_or(false, |&col| col == color) {
                  parts.get(0).and_then(|&num| num.parse::<usize>().ok())
              } else {
                  None
              }
          })
          .next()
          .unwrap_or(0)
}

// we search largest number of cubes in each color array and multiply them together
fn fewest_cubes(games: &[GameData]) -> Vec<usize> {
    games.iter()
        .map(|game| {
            let max_red = *game.red.iter().max().unwrap_or(&0);
            let max_green = *game.green.iter().max().unwrap_or(&0);
            let max_blue = *game.blue.iter().max().unwrap_or(&0);
            max_red * max_green * max_blue
        })
        .collect()
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
    let url = "https://adventofcode.com/2023/day/2/input";
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

    let parsed_results = parse_data(&body)?;

    let game_points = fewest_cubes(&parsed_results);

    let sum = game_points.iter().sum::<usize>();
    #[cfg(debug_assertions)]
    {
        let sum_string = sum.to_string();
        save_to_file("output/output.txt", &sum_string)?;
    }
    println!("{}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_game_data_from_string(input: &str) -> Vec<GameData> {
        parse_data(input).expect("Failed to parse data")
    }

    #[test]
    fn test_fewest_cubes() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let games = create_game_data_from_string(input);
        let game_points = fewest_cubes(&games);
        let sum = game_points.iter().sum::<usize>();
        let expected = 2286; // 48, 12, 1560, 630, 36 = 2286
        assert_eq!(sum, expected);
    }
}
