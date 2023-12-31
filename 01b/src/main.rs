#[cfg(debug_assertions)]
use std::fs::File;
#[cfg(debug_assertions)]
use std::io::Write;
#[cfg(debug_assertions)]
use std::io::Error;

use std::env;
use tokio;
use reqwest;
use dotenv::dotenv;

const NUMS: [(&[u8], char); 9] = [
    (b"one", '1'), (b"two", '2'), (b"three", '3'), (b"four", '4'), 
    (b"five", '5'), (b"six", '6'), (b"seven", '7'), (b"eight", '8'), 
    (b"nine", '9'),
];


fn parse_first_and_last_digit(input: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Split the input string into lines
    let lines: Vec<&str> = input.lines().collect();

    // Process each line to extract first and last digits
    let results: Vec<String> = lines
        .iter()
        .map(|line| {
            // Extract first number (multiplied by 10 to make it the first digit)
            let first_digit = (0..line.len())
                .find_map(|i| num(line.as_bytes(), i))
                .unwrap();

            // Extract last number with reversed loop
            let last_digit = (0..line.len())
                .rev()
                .find_map(|i| num(line.as_bytes(), i))
                .unwrap();

            // Combine first and last digits into a string
            format!("{}{}", first_digit, last_digit)
        })
        .collect();

    // Return the results as a Vec<String>
    Ok(results)
}


/// Extracts a numeric value from a byte slice.
///
/// Given a `line` of bytes and an index `i`, this function looks for numeric values
/// in the `line`. It first checks if the character at index `i` is an ASCII digit.
/// If so, it converts it to an integer and returns it. If not, it searches for
/// matches in `NUM_MAP` to find corresponding numeric values for words in the `line`.
/// It returns the numeric value found, or `None` if no numeric value is detected.
///
/// # Arguments
///
/// * `line` - The byte slice containing the input line.
/// * `i` - The index in the byte slice where the search for a numeric value begins.
///
/// # Returns
///
/// * `Some(usize)` - If a numeric value is found, it returns the numeric value as `Some`.
/// * `None` - If no numeric value is found, it returns `None`.
#[inline(always)]
fn num(line: &[u8], i: usize) -> Option<usize> {
    line[i]
        .is_ascii_digit()
        .then_some((line[i] - b'0') as usize)
        .or(NUMS
            .iter()
            .enumerate()
            .find(|(_, name)| line[i..].starts_with(name.0))
            .map(|(num, _)| num + 1))
}

fn calculate_sum(numbers: &[String]) -> Result<usize, Box<dyn std::error::Error>> {
    let sum: usize = numbers
        .iter()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?
        .iter()
        .sum();

    Ok(sum)
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
    let url = "https://adventofcode.com/2023/day/1/input";
    let cookie = env::var("SESSION_COOKIE").expect("SESSION_COOKIE not set in .env file");

    #[cfg(debug_assertions)]
    println!("Debug: Downloading input file...");

    let body = fetch_url(url, cookie).await?;

    #[cfg(debug_assertions)]
    println!("Debug: File downloaded successfully.");

    #[cfg(debug_assertions)]
    save_to_file("output/input.txt", &body)?;

    let parsed_results = parse_first_and_last_digit(&body)?;

    #[cfg(debug_assertions)]
    {
        let numbers = parsed_results.iter().cloned().collect::<Vec<String>>().join("\n");
        save_to_file("output/numbers.txt", &numbers)?;
    }
    let sum = calculate_sum(&parsed_results)?;

    #[cfg(debug_assertions)]
    {
        let sum_string = sum.to_string();
        save_to_file("output/output.txt", &sum_string)?;
    }

    println!("sum: {}", sum);

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
        // usize expected_sum
        let expected_sum = 281; // expected sum of 29, 83, 13, 24, 42, 14, and 76.
        let result = calculate_sum(&input).unwrap();
        println!("{}", result);

        assert_eq!(result, expected_sum);
    }
}
