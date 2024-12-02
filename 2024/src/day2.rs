#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<Vec<isize>> {
    let mut result = Vec::with_capacity(1000);

    for x in input.lines() {
        let mut report = Vec::with_capacity(8);
        report.extend(x.split(' ').map(|y| y.parse::<isize>().unwrap()));

        result.push(report);
    }

    result
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<Vec<isize>>) -> usize {
    input.iter().filter(|report| is_safe(report.iter())).count()
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Vec<isize>>) -> usize {
    input
        .iter()
        .filter(|report| {
            for i in 0..report.len() {
                let removed = report[0..i]
                    .iter()
                    .chain(report[i + 1..report.len()].iter());

                if is_safe(removed) {
                    return true;
                }
            }

            false
        })
        .count()
}

fn is_safe<'a>(mut report: impl Iterator<Item = &'a isize>) -> bool {
    let mut prev = report.next().unwrap();

    let head = report.next().unwrap();
    let inc = prev < head;

    if !within_range(*prev, *head) {
        return false;
    }

    prev = head;

    report.all(|x| {
        let mut safe = true;
        safe = safe && ((prev < x) == inc);
        safe = safe && within_range(*prev, *x);
        prev = x;
        safe
    })
}

fn within_range(a: isize, b: isize) -> bool {
    (0 != a - b) && (a - b).abs() <= 3
}
