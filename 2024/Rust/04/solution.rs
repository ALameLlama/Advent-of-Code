pub fn run(only_part1: bool, only_part2: bool) {
    let input = include_str!("../../input/day04.txt");

    println!("Day 04");
    if !only_part2 {
        println!("Part 1 answer: {}", part1(input));
    }
    if !only_part1 {
        println!("Part 2 answer: {}", part2(input));
    }
}

fn part1(input: &str) -> i64 {
    let mut sum = 0;
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let max_rows = matrix.len();
    let max_cols = matrix[0].len();

    for row in 0..max_rows {
        for col in 0..max_cols {
            if matrix[row][col] == 'X' {
                // Forward
                if col + 3 < max_cols
                    && matrix[row][col + 1] == 'M'
                    && matrix[row][col + 2] == 'A'
                    && matrix[row][col + 3] == 'S'
                {
                    sum += 1;
                }

                // Backward
                if col >= 3
                    && matrix[row][col - 1] == 'M'
                    && matrix[row][col - 2] == 'A'
                    && matrix[row][col - 3] == 'S'
                {
                    sum += 1;
                }

                // Vertical down
                if row + 3 < max_rows
                    && matrix[row + 1][col] == 'M'
                    && matrix[row + 2][col] == 'A'
                    && matrix[row + 3][col] == 'S'
                {
                    sum += 1;
                }

                // Vertical up
                if row >= 3
                    && matrix[row - 1][col] == 'M'
                    && matrix[row - 2][col] == 'A'
                    && matrix[row - 3][col] == 'S'
                {
                    sum += 1;
                }

                // Diagonal down-right
                if row + 3 < max_rows
                    && col + 3 < max_cols
                    && matrix[row + 1][col + 1] == 'M'
                    && matrix[row + 2][col + 2] == 'A'
                    && matrix[row + 3][col + 3] == 'S'
                {
                    sum += 1;
                }

                // Diagonal down-left
                if row + 3 < max_rows
                    && col >= 3
                    && matrix[row + 1][col - 1] == 'M'
                    && matrix[row + 2][col - 2] == 'A'
                    && matrix[row + 3][col - 3] == 'S'
                {
                    sum += 1;
                }

                // Diagonal up-right
                if row >= 3
                    && col + 3 < max_cols
                    && matrix[row - 1][col + 1] == 'M'
                    && matrix[row - 2][col + 2] == 'A'
                    && matrix[row - 3][col + 3] == 'S'
                {
                    sum += 1;
                }

                // Diagonal up-left
                if row >= 3
                    && col >= 3
                    && matrix[row - 1][col - 1] == 'M'
                    && matrix[row - 2][col - 2] == 'A'
                    && matrix[row - 3][col - 3] == 'S'
                {
                    sum += 1;
                }
            }
        }
    }

    sum
}

fn part2(input: &str) -> i64 {
    let mut sum = 0;
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let max_rows = matrix.len();
    let max_cols = matrix[0].len();

    for row in 0..max_rows.saturating_sub(2) {
        for col in 0..max_cols.saturating_sub(2) {
            if row + 2 < max_rows && col + 2 < max_cols && {
                // named to stop hurting myself in confusion
                let top_left = matrix[row][col];
                let center = matrix[row + 1][col + 1];
                let bottom_right = matrix[row + 2][col + 2];
                let top_right = matrix[row][col + 2];
                let bottom_left = matrix[row + 2][col];

                // Bail early
                if center != 'A' {
                    continue;
                }

                // Forward-Forward (M-A-S for both diagonals)
                let forward_forward = top_left == 'M'
                    && bottom_right == 'S'
                    && top_right == 'M'
                    && bottom_left == 'S';

                // Forward-Backward (M-A-S and S-A-M)
                let forward_backward = top_left == 'M'
                    && bottom_right == 'S'
                    && top_right == 'S'
                    && bottom_left == 'M';

                // Backward-Forward (S-A-M and M-A-S)
                let backward_forward = top_left == 'S'
                    && bottom_right == 'M'
                    && top_right == 'M'
                    && bottom_left == 'S';

                // Backward-Backward (S-A-M for both diagonals)
                let backward_backward = top_left == 'S'
                    && bottom_right == 'M'
                    && top_right == 'S'
                    && bottom_left == 'M';

                forward_forward || forward_backward || backward_forward || backward_backward
            } {
                sum += 1;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";
        assert_eq!(part1(input), 18);
    }

    #[test]
    fn test_part2() {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!(part2(input), 9);
    }

    #[test]
    fn test_part2_edge_case_1() {
        let input = ".M.M......
..A.......
.S.S......
..........";
        assert_eq!(part2(input), 1);
    }
}
