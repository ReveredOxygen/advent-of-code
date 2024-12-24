use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<usize>;

#[aoc_generator(day22)]
fn parse(input: &str) -> Input {
    let result = Input::new();

    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn evolve(secret: &mut usize) {
    *secret ^= *secret * 64;
    *secret %= 16777216;
    *secret ^= *secret / 32;
    *secret %= 16777216;
    *secret ^= *secret * 2048;
    *secret %= 16777216;
}

#[aoc(day22, part1)]
fn part1(input: &Input) -> usize {
    let mut input2 = input.clone();

    input2
        .iter_mut()
        .map(|x| {
            for _ in 0..2000 {
                evolve(x);
            }

            *x
        })
        .sum()
}

#[aoc(day22, part2)]
fn part2(input: &Input) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn part1_example() {
    //     assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    // }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
