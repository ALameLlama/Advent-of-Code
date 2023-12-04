use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");

    let schematic = input.lines().collect::<Vec<_>>();

    let output = aoc_process(schematic).unwrap_or_default(); // println!("{output}");

    println!("{output}");
}

fn aoc_process(input: Vec<&str>) -> Result<u32, &'static str> {
    let max_hight = input.len() - 1;
    let max_width = input[0].len() - 1;

    let regex = Regex::new(r"(?<part>\d+)").expect("Failed to compile regex");
    let symbol = Regex::new(r"[*]").expect("Failed to compile regex");

    let mut total: u32 = 0;

    for (i, line) in input.iter().enumerate() {
        for ratios in symbol.captures_iter(line) {
            if let Some(ratio) = ratios.get(0) {
                let mut range_start = ratio.start();
                if range_start > 0 {
                    range_start -= 1;
                }

                let mut range_end = ratio.end();
                if range_end < max_width {
                    range_end += 1;
                }

                let mut total_ratio: Vec<u32> = Vec::new();

                // TODO: Refactor the nested statmenst to use a function instead.

                // Check the current line.
                if let Some(_found) = regex.captures(&input[i][range_start..range_end]) {
                    for parts in regex.captures_iter(&input[i]) {
                        if let Some(part) = parts.get(0) {
                            if ranges_overlap(range_start..range_end, part.start()..part.end()) {
                                let part_num = part
                                    .as_str()
                                    .parse::<u32>()
                                    .map_err(|_| "Failed to parse part")?;

                                total_ratio.push(part_num);
                            }
                        }
                    }
                }

                // Chech the above line.
                if i > 0 {
                    if let Some(_found) = regex.captures(&input[i - 1][range_start..range_end]) {
                        for parts in regex.captures_iter(&input[i - 1]) {
                            if let Some(part) = parts.get(0) {
                                if ranges_overlap(range_start..range_end, part.start()..part.end())
                                {
                                    let part_num = part
                                        .as_str()
                                        .parse::<u32>()
                                        .map_err(|_| "Failed to parse part")?;

                                    total_ratio.push(part_num);
                                }
                            }
                        }
                    }
                }

                // Check the below line.
                if i < max_hight {
                    if let Some(_found) = regex.captures(&input[i + 1][range_start..range_end]) {
                        for parts in regex.captures_iter(&input[i + 1]) {
                            if let Some(part) = parts.get(0) {
                                if ranges_overlap(range_start..range_end, part.start()..part.end())
                                {
                                    let part_num = part
                                        .as_str()
                                        .parse::<u32>()
                                        .map_err(|_| "Failed to parse part")?;

                                    total_ratio.push(part_num);
                                }
                            }
                        }
                    }
                }

                if total_ratio.len() == 2 {
                    let totals: u32 = total_ratio.iter().product();
                    total += totals;
                }
            }
        }
    }

    return Ok(total);
}

fn ranges_overlap(range1: std::ops::Range<usize>, range2: std::ops::Range<usize>) -> bool {
    range1.start < range2.end && range1.end > range2.start
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

        assert_eq!(output, 467835);
    }
}