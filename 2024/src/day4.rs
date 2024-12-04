use arrayvec::ArrayVec;
use core::str;

type Input = ArrayVec<ArrayVec<u8, 140>, 140>;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day4)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.as_bytes().iter().map(|x| x.clone()).collect())
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> usize {
    let max_i = input.len() - 3;
    let max_j = input[0].len() - 3;

    let mut num_xmas = 0;

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if j < max_j && &input[i][j..j + 4] == b"XMAS" {
                num_xmas += 1;
            }
            if j < max_j && &input[i][j..j + 4] == b"SAMX" {
                num_xmas += 1;
            }
            if i < max_i
                && input[i][j] == b'X'
                && input[i + 1][j] == b'M'
                && input[i + 2][j] == b'A'
                && input[i + 3][j] == b'S'
            {
                num_xmas += 1;
            }
            if i < max_i
                && input[i][j] == b'S'
                && input[i + 1][j] == b'A'
                && input[i + 2][j] == b'M'
                && input[i + 3][j] == b'X'
            {
                num_xmas += 1;
            }
            if i < max_i + 0
                && j < max_j + 0
                && input[i][j] == b'X'
                && input[i + 1][j + 1] == b'M'
                && input[i + 2][j + 2] == b'A'
                && input[i + 3][j + 3] == b'S'
            {
                num_xmas += 1;
            }
            if i >= 3
                && j < max_j
                && input[i][j] == b'X'
                && input[i - 1][j + 1] == b'M'
                && input[i - 2][j + 2] == b'A'
                && input[i - 3][j + 3] == b'S'
            {
                num_xmas += 1;
            }
            if i < max_i
                && j < max_j
                && input[i][j] == b'S'
                && input[i + 1][j + 1] == b'A'
                && input[i + 2][j + 2] == b'M'
                && input[i + 3][j + 3] == b'X'
            {
                num_xmas += 1;
            }
            if i >= 3
                && j < max_j
                && input[i][j] == b'S'
                && input[i - 1][j + 1] == b'A'
                && input[i - 2][j + 2] == b'M'
                && input[i - 3][j + 3] == b'X'
            {
                num_xmas += 1;
            }
        }
    }

    num_xmas
}

#[aoc(day4, part2)]
fn part2(input: &Input) -> usize {
    let mut num_xmas = 0;

    for i in 1..input.len() - 1 {
        for j in 1..input[0].len() - 1 {
            let mut mas1 = false;
            let mut mas2 = false;

            if input[i][j] != b'A' {
                continue;
            }

            if input[i - 1][j - 1] == b'M' && input[i + 1][j + 1] == b'S' {
                mas1 = true;
            }
            if input[i - 1][j - 1] == b'S' && input[i + 1][j + 1] == b'M' {
                mas1 = true;
            }
            if input[i - 1][j + 1] == b'M' && input[i + 1][j - 1] == b'S' {
                mas2 = true;
            }
            if input[i - 1][j + 1] == b'S' && input[i + 1][j - 1] == b'M' {
                mas2 = true;
            }

            if mas1 && mas2 {
                num_xmas += 1;
            }
        }
    }

    num_xmas
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 9);
    }
}
