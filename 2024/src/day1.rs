use arrayvec::ArrayVec;

const NUM_ELEMS: usize = 1000;

#[aoc_generator(day1)]
pub fn parse(input: &str) -> (ArrayVec<i32, NUM_ELEMS>, ArrayVec<i32, NUM_ELEMS>) {
    let mut left = ArrayVec::new();
    let mut right = ArrayVec::new();

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
pub fn part1(input: &(ArrayVec<i32, NUM_ELEMS>, ArrayVec<i32, NUM_ELEMS>)) -> i32 {
    let mut left = input.0.clone();
    let mut right = input.1.clone();

    left.sort();
    right.sort();

    let mut differences = [0; 1000];
    abs_diff(left.as_slice(), right.as_slice(), &mut differences);

    differences.iter().sum()
}

fn abs_diff(a: &[i32], b: &[i32], c: &mut [i32]) {
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
unsafe fn abs_diff_avx2(a: &[i32], b: &[i32], c: &mut [i32]) {
    abs_diff_fallback(a, b, c) // the function below is inlined here
}

fn abs_diff_fallback(a: &[i32], b: &[i32], c: &mut [i32]) {
    for ((a, b), c) in a.iter().zip(b).zip(c) {
        *c = (*a - *b).abs();
    }
}

#[aoc(day1, part2)]
pub fn part2(input: &(ArrayVec<i32, NUM_ELEMS>, ArrayVec<i32, NUM_ELEMS>)) -> i32 {
    let left = &input.0;
    let right = &input.1;

    let mut freqs: [u8; 100000] = [0; 100000];

    for x in right {
        freqs[*x as usize] += 1;
    }

    left.iter().map(|x| x * freqs[*x as usize] as i32).sum()
}
