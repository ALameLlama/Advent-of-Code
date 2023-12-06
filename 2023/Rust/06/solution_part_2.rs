use std::char;

fn main() {
    let input = include_str!("./input2.txt");

    let output: u64 = aoc_process(input).unwrap_or_default();

    println!("{output}");
}

fn aoc_process(input: &str) -> Result<u64, &'static str> {
    let (times_str, distances_str) = input.split_once('\n').unwrap();
    let (_, times) = times_str.split_once(':').unwrap();
    let (_, distances) = distances_str.split_once(':').unwrap();

    let time: u64 = times
        .replace(char::is_whitespace, "")
        .parse()
        .map_err(|_| "Failed to parse time")?;

    let distance: u64 = distances
        .replace(char::is_whitespace, "")
        .parse()
        .map_err(|_| "Failed to parse distance")?;

    let start_time = (0..time).find(|i| i * (time - i) > distance).unwrap();
    let end_time = (0..time).rev().find(|i| i * (time - i) > distance).unwrap();

    Ok(end_time - start_time + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let output: u64 = aoc_process(input).unwrap_or_default();

        assert_eq!(output, 71503);
    }
}