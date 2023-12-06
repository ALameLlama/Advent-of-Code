fn main() {
    let input = include_str!("./input1.txt");

    let output: u32 = aoc_process(input).unwrap_or_default();

    println!("{output}");
}

fn aoc_process(input: &str) -> Result<u32, &'static str> {
    let (times_str, distances_str) = input.split_once('\n').unwrap();
    let (_, times) = times_str.split_once(':').unwrap();
    let (_, distances) = distances_str.split_once(':').unwrap();

    let times_parsed = times
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();

    let distances_parsed = distances
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();

    let count: usize = times_parsed
        .iter()
        .zip(distances_parsed)
        .map(|(time, distance)| (0..*time).filter(|&i| i * (time - i) > distance).count())
        .product();

    return Ok(count as u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let output: u32 = aoc_process(input).unwrap_or_default();

        assert_eq!(output, 288);
    }
}