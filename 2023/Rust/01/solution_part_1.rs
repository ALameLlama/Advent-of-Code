use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");

    let dig = Regex::new(r"(?P<num>\d)").expect("Failed to compile regex");

    let output: u32 = input
        .lines()
        .map(|line| aoc_process(line, &dig).unwrap_or_default())
        .sum();

    println!("{output}");
}

fn aoc_process(input: &str, dig: &Regex) -> Result<u32, &'static str> {
    let caps_first = dig.captures(input).ok_or("Failed to match first capture")?;

    let binding = input.chars().rev().collect::<String>();
    let caps_last = dig
        .captures(binding.as_str())
        .ok_or("Failed to match last capture")?;

    let count: u32 = [caps_first["num"].to_string(), caps_last["num"].to_string()]
        .concat()
        .parse::<u32>()
        .map_err(|_| "Failed to parse into u32")?;

    println!("{}", count);
    return Ok(count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

        let dig = Regex::new(r"(?P<num>\d)").expect("Failed to compile regex");

        let output: u32 = input
            .lines()
            .map(|line| aoc_process(line, &dig).unwrap_or_default())
            .sum();

        assert_eq!(output, 142);
    }
}
