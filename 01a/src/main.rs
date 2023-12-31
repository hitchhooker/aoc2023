use std::env;
use std::fs::File;
use std::io::Write;
use std::io::Error;

use tokio;
use reqwest;
use dotenv::dotenv;

fn parse_first_and_last_digit(input: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let mut results = Vec::new();

    for line in input.lines() {
        let digits: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
        if let Some(first_digit) = digits.first() {
            if let Some(last_digit) = digits.last() {
               results.push(format!("{}{}", first_digit, last_digit));
            }
        }
    }
    Ok(results)
}

fn calculate_sum(results: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    let sum = results
        .iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?
        .iter()
        .sum::<i32>();

    Ok(sum.to_string())  // Convert the sum back to a String for output
}

async fn fetch_url(url: &str, cookie: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    // let response = reqwest::get(url).await?;
    let response = client
        .get(url)
        .header("Cookie", cookie)
        .send()
        .await?;
    response.text().await
}

fn save_to_file(filename: &str, data: &str) -> Result<(), Error> {
    let mut file = File::create(filename)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file
    let url = "https://adventofcode.com/2023/day/1/input";
    let cookie = env::var("SESSION_COOKIE").expect("SESSION_COOKIE not set in .env file");

    let body = fetch_url(url, cookie).await?;

    // input
    let _ = save_to_file("output/input.txt", &body);

    // output
    match parse_first_and_last_digit(&body) {
        Ok(results) => {
            let numbers = results.join("\n");  // Combine the results into a single string
            let _ = save_to_file("output/numbers.txt", &numbers)?;
            let output = calculate_sum(results).unwrap();
            let _ = save_to_file("output/output.txt", &output)?;
        },
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_first_and_last() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let expected = vec!["12".to_string(), "38".to_string(), "15".to_string(), "77".to_string()];
        let result = parse_first_and_last_digit(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_calculate_sum() {
        let input = vec!["12".to_string(), "38".to_string(), "15".to_string(), "77".to_string()];
        let expected_sum = "142"; // expected sum of 12 + 38 + 15 + 77

        let result = calculate_sum(input).unwrap();
        assert_eq!(result, expected_sum);
    }
}
