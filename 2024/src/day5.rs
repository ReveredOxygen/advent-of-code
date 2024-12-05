use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

type Rules = HashMap<u8, Vec<u8>>;
type Update = Vec<u8>;
type Input = (Rules, Vec<Update>);

#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    let mut rules = Rules::new();
    let mut updates = Vec::new();

    let mut parsing_rules = true;
    for line in input.lines() {
        if line.len() == 0 {
            parsing_rules = false;
            continue;
        }
        if parsing_rules {
            let mut split = line.split('|');
            let req = split.next().unwrap().parse().unwrap();
            let page = split.next().unwrap().parse().unwrap();

            rules.get_mut(&page).unwrap_or(&mut Vec::new()).push(req);
            match rules.get_mut(&page) {
                Some(v) => v.push(req),
                None => {
                    rules.insert(page, vec![req]);
                }
            }
        } else {
            updates.push(line.split(',').map(|x| x.parse().unwrap()).collect())
        }
    }

    (rules, updates)
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> usize {
    let mut acc = 0;

    let (rules, mut updates) = input.clone();

    for mut update in updates {
        if validate(&update, &rules).0 {
            acc += update[update.len() / 2] as usize;
        } else {
            // do nothing
        }
    }

    acc
}

fn validate(update: &Update, rules: &Rules) -> (bool, usize) {
    let mut seen = HashSet::new();
    let printing: HashSet<&u8> = HashSet::from_iter(update.iter());
    for (i, x) in update.iter().enumerate() {
        seen.insert(x);
        for req in rules.get(&x).unwrap_or(&Vec::new()) {
            if !seen.contains(req) && printing.contains(req) {
                return (false, i);
            }
        }
    }

    (true, 0)
}

fn fix(update: &mut Update, rules: &Rules) {
    while let (false, idx) = validate(update, rules) {
        let x = update.remove(idx);
        update.push(x);
    }
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> usize {
    let mut acc = 0;

    let (rules, mut updates) = input.clone();

    for mut update in updates {
        if validate(&update, &rules).0 {
            // do nothing
        } else {
            fix(&mut update, &rules);
            acc += update[update.len() / 2] as usize;
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 123);
    }
}
