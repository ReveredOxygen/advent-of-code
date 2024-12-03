use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day3, part1)]
fn part1(input: &str) -> i64 {
    let re = Regex::new(r#"mul\((?<d1>\d+),(?<d2>\d+)\)"#).unwrap();

    let mut acc = 0;

    for cap in re.captures_iter(input) {
        acc += cap["d1"].parse::<i64>().unwrap() * cap["d2"].parse::<i64>().unwrap();
    }

    acc
}

#[aoc(day3, part2)]
fn part2(input: &str) -> i64 {
    let re = Regex::new(r#"(?<tog>do(n't)?\(\))|(mul\((?<d1>\d+),(?<d2>\d+)\))"#).unwrap();

    let mut acc = 0;
    let mut mul = true;

    for cap in re.captures_iter(input) {
        if let Some(s) = cap.name("tog") {
            if s.len() == 4 {
                mul = true;
            } else {
                mul = false;
            }
        } else if mul {
            acc += cap["d1"].parse::<i64>().unwrap() * cap["d2"].parse::<i64>().unwrap();
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#),
            48
        );
    }
}
