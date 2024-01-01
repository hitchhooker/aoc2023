use reqwest;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

// Asynchronous function to fetch data from a URL.
pub async fn fetch_url(url: &str, cookie: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    client.get(url).header("Cookie", cookie).send().await?.text().await
}

// Function to save data to a file.
pub fn save_to_file(filename: &str, data: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

// Function to read data from a file or fetch from a URL if the file does not exist.
pub async fn get_data(url: &str, cookie: String, local_path: &str) -> Result<String, Box<dyn Error>> {
    if Path::new(local_path).exists() {
        Ok(fs::read_to_string(local_path)?)
    } else {
        let fetched_data = fetch_url(url, cookie).await?;
        save_to_file(local_path, &fetched_data)?;
        Ok(fetched_data)
    }
}
