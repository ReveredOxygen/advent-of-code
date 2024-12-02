use std::{collections::HashMap, iter::zip};

#[aoc_generator(day1)]
pub fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);

    let mut next_left = true;
    for s in input.split_whitespace() {
        if next_left {
            left.push(s.parse().unwrap());
        } else {
            right.push(s.parse().unwrap());
        }

        next_left = !next_left;
    }

    (left, right)
}

#[aoc(day1, part1)]
pub fn part1(input: &(Vec<i64>, Vec<i64>)) -> i64 {
    let mut left = input.0.clone();
    let mut right = input.1.clone();

    left.sort();
    right.sort();

    let mut acc = 0;
    for x in zip(left, right) {
        acc += (x.0 - x.1).abs();
    }

    acc
}

#[aoc(day1, part2)]
pub fn part2(input: &(Vec<i64>, Vec<i64>)) -> i64 {
    let left = &input.0;
    let right = &input.1;

    let mut freqs = HashMap::new();

    for x in right {
        let freq = freqs.get_mut(&x);
        match freq {
            Some(f) => {
                *f += 1;
            }
            None => {
                freqs.insert(x, 1);
            }
        }
    }

    left.iter()
        .map(|x| freqs.get(&x).map(|y| (x, y)))
        .flatten()
        .map(|(x, y)| x * y)
        .sum()
}
