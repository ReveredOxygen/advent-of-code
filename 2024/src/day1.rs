use std::collections::HashMap;

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

    let mut differences = [0; 1000];
    abs_diff(left.as_slice(), right.as_slice(), &mut differences);

    differences.iter().sum()
}

fn abs_diff(a: &[i64], b: &[i64], c: &mut [i64]) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        // Note that this `unsafe` block is safe because we're testing
        // that the `avx2` feature is indeed available on our CPU.
        if is_x86_feature_detected!("avx2") {
            return unsafe { abs_diff_avx2(a, b, c) };
        }
    }

    abs_diff_fallback(a, b, c)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn abs_diff_avx2(a: &[i64], b: &[i64], c: &mut [i64]) {
    abs_diff_fallback(a, b, c) // the function below is inlined here
}

fn abs_diff_fallback(a: &[i64], b: &[i64], c: &mut [i64]) {
    for ((a, b), c) in a.iter().zip(b).zip(c) {
        *c = (*a - *b).abs();
    }
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
