#[derive(Debug)]
enum Garden {
    Seed(Seed),
    Soil(GardenMapping),
    Fertilizer(GardenMapping),
    Water(GardenMapping),
    Light(GardenMapping),
    Temperature(GardenMapping),
    Humidity(GardenMapping),
    Location(GardenMapping),
    Unknown,
}

#[derive(Debug)]
struct Seed {
    id: u64,
}

#[derive(Debug)]
struct GardenMapping {
    dst: u64,
    src: u64,
    range: u64,
}

impl GardenMapping {
    fn new(dst: u64, src: u64, range: u64) -> Self {
        GardenMapping { dst, src, range }
    }
}


fn parse_data(input: &str) -> Vec<Garden> {
    let mut garden_data = Vec::new();

    // Split the input into sections
    let sections: Vec<&str> = input.split("\n\n").collect();

    // Handle the seeds section separately
    if let Some(seeds_section) = sections.first() {
        if seeds_section.starts_with("seeds:") {
            garden_data.extend(
                seeds_section.split_whitespace()
                    .skip(1) // Skip the "seeds:" part
                    .filter_map(|s| s.parse().ok())
                    .map(|id| Garden::Seed(Seed { id }))
            );
        }
    }

    // Process the remaining sections
    for &section in sections.iter().skip(1) { // Skip the first section (seeds)
        if let Some((title, data)) = section.split_once('\n') {
            let title = title.trim().trim_end_matches(':');
            garden_data.extend(
                data.lines()
                    .map(parse_garden_mapping)
                    .map(|mapping| map_title_to_garden(title, mapping))
                    .collect::<Vec<Garden>>()
            );
        }
    }

    garden_data
}


fn map_title_to_garden(title: &str, mapping: GardenMapping) -> Garden {
    match title {
        "seed-to-soil map" => Garden::Soil(mapping),
        "soil-to-fertilizer map" => Garden::Fertilizer(mapping),
        "fertilizer-to-water map" => Garden::Water(mapping),
        "water-to-light map" => Garden::Light(mapping),
        "light-to-temperature map" => Garden::Temperature(mapping),
        "temperature-to-humidity map" => Garden::Humidity(mapping),
        "humidity-to-location map" => Garden::Location(mapping),
        _ => Garden::Unknown, // Or handle this case as you see fit
    }
}


fn parse_garden_mapping(line: &str) -> GardenMapping {
    let values: Vec<u64> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();
    GardenMapping::new(values[0], values[1], values[2])
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok(); // Load .env file
    let input_file_path = "output/input.txt";
    let url = "https://adventofcode.com/2023/day/5/input";
    let cookie = std::env::var("SESSION_COOKIE").expect("SESSION_COOKIE not set in .env file");
    let body = aoc2023::get_data(url, cookie, input_file_path).await?;

    let data = parse_data(&body);
    let result = 0;//solve_path(&data);

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
    *
    * seeds: 79 14 55 13
    * seed-to-soil map:
    * 50 98 2
    * 52 50 48
    * 
    * soil-to-fertilizer map:
    * 0 15 37
    * 37 52 2
    * 39 0 15
    * 
    * fertilizer-to-water map:
    * 49 53 8
    * 0 11 42
    * 42 0 7
    * 57 7 4
    * 
    * water-to-light map:
    * 88 18 7
    * 18 25 70
    * 
    * light-to-temperature map:
    * 45 77 23
    * 81 45 19
    * 68 64 13
    * 
    * temperature-to-humidity map:
    * 0 69 1
    * 1 0 69
    * 
    * humidity-to-location map:
    * 60 56 37
    * 56 93 4
    *
    */
    #[test]
    fn test_game_points() {
        let input = "seeds: 79 14 55 13\n seed-to-soil map:\n 50 98 2\n 52 50 48\n \n soil-to-fertilizer map:\n 0 15 37\n 37 52 2\n 39 0 15\n \n fertilizer-to-water map:\n 49 53 8\n 0 11 42\n 42 0 7\n 57 7 4\n \n water-to-light map:\n 88 18 7\n 18 25 70\n \n light-to-temperature map:\n 45 77 23\n 81 45 19\n 68 64 13\n \n temperature-to-humidity map:\n 0 69 1\n 1 0 69\n \n humidity-to-location map:\n 60 56 37\n 56 93 4";
        let expected = 35; //
        let data = parse_data(&input);
        let score = 35;
        assert_eq!(score, expected);
    }
}
