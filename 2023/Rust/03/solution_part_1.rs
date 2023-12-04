use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");

    let schematic = input.lines().collect::<Vec<_>>();

    let output = aoc_process(schematic).unwrap_or_default();

    println!("{output}");
}

fn aoc_process(input: Vec<&str>) -> Result<u32, &'static str> {
    let max_hight = input.len() - 1;
    let max_width = input[0].len() - 1;

    let regex = Regex::new(r"(?<part>\d+)").expect("Failed to compile regex");
    let symbol = Regex::new(r"[^0-9\.]").expect("Failed to compile regex");

    let mut total: u32 = 0;

    for (i, line) in input.iter().enumerate() {
        for parts in regex.captures_iter(line) {
            if let Some(part) = parts.get(1) {
                if let Ok(part_num) = part.as_str().parse::<u32>() {
                    let mut range_start = part.start();
                    if range_start > 0 {
                        range_start -= 1;
                    }

                    let mut range_end = part.end();
                    if range_end < max_width {
                        range_end += 1;
                    }

                    // Check the current line.
                    if let Some(_found) = symbol.captures(&input[i][range_start..range_end]) {
                        total += part_num;
                        continue;
                    }

                    // Chech the above line.
                    if i > 0 {
                        if let Some(_found) = symbol.captures(&input[i - 1][range_start..range_end])
                        {
                            total += part_num;
                            continue;
                        }
                    }

                    // Check the below line.
                    if i < max_hight {
                        if let Some(_found) = symbol.captures(&input[i + 1][range_start..range_end])
                        {
                            total += part_num;
                            continue;
                        }
                    }

                    println!("Is not a part: {part_num}");
                } else {
                    return Err("Failed to get part");
                }
            }
        }
    }

    return Ok(total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let schematic = input.lines().collect::<Vec<_>>();

        let output = aoc_process(schematic).unwrap_or_default();

        assert_eq!(output, 4361);
    }
}
