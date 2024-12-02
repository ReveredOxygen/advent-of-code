#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<Vec<isize>> {
    let mut result = vec![];

    for x in input.lines() {
        result.push(
            x.split_ascii_whitespace()
                .map(|y| y.parse().unwrap())
                .collect(),
        )
    }

    result
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<Vec<isize>>) -> usize {
    input
        .iter()
        .filter(|report| {
            let inc = report[0] < report[1];
            for i in 1..report.len() {
                if (report[i - 1] < report[i]) != inc
                    || (report[i - 1] - report[i]).abs() > 3
                    || report[i - 1] - report[i] == 0
                {
                    return false;
                }
            }
            true
        })
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Vec<isize>>) -> usize {
    input
        .iter()
        .filter(|report| {
            let mut seen_good = false;

            for i in 0..report.len() {
                let mut report2 = report[0..i].to_vec();
                report2.append(&mut report[i + 1..report.len()].to_vec());

                let mut seen_bad = false;

                let inc = report2[0] < report2[1];
                for i in 1..report2.len() {
                    if (report2[i - 1] < report2[i]) != inc
                        || (report2[i - 1] - report2[i]).abs() > 3
                        || report2[i - 1] - report2[i] == 0
                    {
                        seen_bad = true;
                    }
                }
                if !seen_bad {
                    seen_good = true;
                }
            }

            seen_good
        })
        .count()
}
