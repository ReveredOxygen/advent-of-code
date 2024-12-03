use aoc_runner_derive::aoc;
use std::str;

#[aoc(day3, part1)]
fn part1(mut input: &str) -> u64 {
    let mut acc = 0;

    while input.len() >= 8 {
        match parse_mul(input) {
            Some((res, consumed)) => {
                acc += res;
                input = &input[consumed..];
            }
            None => input = &input[1..],
        }
    }

    acc
}

#[aoc(day3, part2)]
fn part2(mut input: &str) -> u64 {
    let mut acc = 0;

    let mut parsing = true;

    while input.len() >= 8 {
        if let Some((continuing, consumed)) = parse_do_dont(input) {
            parsing = continuing;
            input = &input[consumed..];
        } else if parsing {
            match parse_mul(input) {
                Some((res, consumed)) => {
                    acc += res;
                    input = &input[consumed..];
                }
                None => input = &input[1..],
            }
        } else {
            input = &input[1..];
        }
    }

    acc
}

fn parse_do_dont(input: &str) -> Option<(bool, usize)> {
    if !input.as_bytes().starts_with(b"do") {
        return None;
    }

    if &input.as_bytes()[2..4] == b"()" {
        return Some((true, 4));
    } else if &input.as_bytes()[2..7] == b"n't()" {
        return Some((false, 7));
    }

    return None;
}

fn parse_mul(input: &str) -> Option<(u64, usize)> {
    if !input.as_bytes().starts_with(b"mul(") {
        return None;
    }

    let mut consumed = 4;

    let mut args = input[4..].as_bytes();

    let mut n1: u64 = 0;
    let mut n2: u64 = 0;

    let mut broken = false;

    for i in 0..3 {
        if b'0' <= args[i] && args[i] <= b'9' {
            n1 *= 10;
            n1 += (args[i] - b'0') as u64;
        } else if args[i] == b',' {
            if i > 0 {
                args = &args[i + 1..];
                consumed += i + 1;
                broken = true;
                break;
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    if !broken && args[3] != b',' {
        return None;
    } else if !broken {
        args = &args[4..];
        consumed += 4;
    }

    for i in 0..4 {
        if b'0' <= args[i] && args[i] <= b'9' {
            n2 *= 10;
            n2 += (args[i] - b'0') as u64;
        } else if args[i] == b')' {
            if i > 0 {
                return Some((n1 * n2, consumed + i + 1));
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_do() {
        assert_eq!(parse_do_dont("do()"), Some((true, 4)));
    }

    #[test]
    fn parse_dont() {
        assert_eq!(parse_do_dont("don't()"), Some((false, 7)));
    }

    #[test]
    fn mul_parser() {
        assert_eq!(parse_mul("mul(2,412)fhgweuiosfh"), Some((824, 10)));
    }

    #[test]
    fn mul_parser2() {
        assert_eq!(parse_mul("mul(177,274)when"), Some((48498, 12)));
    }

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
