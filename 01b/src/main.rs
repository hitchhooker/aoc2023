use std::env;
use std::fs::File;
use std::io::Write;
use std::io::Error;
use std::collections::HashMap;

use tokio;
use reqwest;
use dotenv::dotenv;

fn parse_first_and_last_digit(input: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let mut results = Vec::new();

    for line in input.lines() {
        let legit_digits = transform_to_digits(line);
        let digits: Vec<char> = legit_digits.chars().filter(|c| c.is_numeric()).collect();
        if let Some(first_digit) = digits.first() {
            if let Some(last_digit) = digits.last() {
                results.push(format!("{}{}", first_digit, last_digit));
                println!("{}{}", first_digit, last_digit)
            }
        }
    }
    Ok(results)
}


fn transform_to_digits(input: &str) -> String {
    let number_map = create_number_map();
    let mut result = String::new();
    let mut current_word = String::new();

    for ch in input.chars() {

        if ch.is_alphabetic() {
            current_word.push(ch);
            for (word, &digit) in &number_map {
                if current_word.contains(word) {
                    // Find the position of the number word
                    let start = current_word.find(word).unwrap();
                    let end = start + word.len();
                    // Keep the part of current_word before the number word
                    result.push_str(&current_word[..start]);
                    // Replace the number word with the digit
                    result.push(digit);
                    // Keep the part of current_word after the number word
                    current_word = current_word[end..].to_string();
                    break;
                }
            }
        } else {
            result.push_str(&current_word);
            current_word.clear();
            result.push(ch);
        }
        println!("{}", current_word)
    }
    result.push_str(&current_word); // Append any remaining characters
    println!("{}{}", "input: ", input);
    println!("{}{}", "result: ", result);
    result
}

fn create_number_map() -> HashMap<&'static str, char> {
    let mut map = HashMap::new();
    map.insert("one", '1');
    map.insert("two", '2');
    map.insert("three", '3');
    map.insert("four", '4');
    map.insert("five", '5');
    map.insert("six", '6');
    map.insert("seven", '7');
    map.insert("eight", '8');
    map.insert("nine", '9');
    map
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
            println!("{}{}", "result:", output)
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
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        // Expected: 29, 83, 13, 24, 42, 14, and 76.
        let expected = vec!["29".to_string(), "83".to_string(), "13".to_string(), "24".to_string(), "42".to_string(), "14".to_string(), "76".to_string()];
        let result = parse_first_and_last_digit(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_calculate_sum() {
        let input = vec!["29".to_string(), "83".to_string(), "13".to_string(), "24".to_string(), "42".to_string(), "14".to_string(), "76".to_string()];
        let expected_sum = "281"; // expected sum of 29, 83, 13, 24, 42, 14, and 76.
        let result = calculate_sum(input).unwrap();
        println!("{}", result);

        assert_eq!(result, expected_sum);
    }
}
