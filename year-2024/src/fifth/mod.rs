use std::{collections::HashMap, hash::Hash, num::ParseFloatError};

const SAMPLE: &str = "47|53
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
97,13,75,29,47";

#[derive(Debug)]
struct PageOrderingRules {
    rules: HashMap<u16, Vec<u16>>,
}

impl PageOrderingRules {
    pub fn is_allowed_update(&self, update: &Vec<u16>) -> (bool, Option<Vec<u16>>) {
        let mut already_printed_pages = Vec::new();

        for (i, page) in update.iter().enumerate() {
            if let Some(must_not_be_printed_before) = self.rules.get(page) {
                // check if we already printed something we are not allowed to
                for (j, already_printed_page) in already_printed_pages.iter().enumerate() {
                    if must_not_be_printed_before.contains(already_printed_page) {
                        // eprintln!("{already_printed_page} must not be printed before {page}");

                        let mut fixed_update = update.clone();
                        fixed_update.swap(i, j);

                        let (is_allowed, maybe_fixed) = self.is_allowed_update(&fixed_update);

                        if is_allowed {
                            return (false, Some(fixed_update));
                        } else {
                            return (false, maybe_fixed);
                        }
                    }
                }
            }

            already_printed_pages.push(*page);
        }

        (true, None)
    }
}

pub(crate) fn fifth_december() {
    let input = include_str!("fifth.txt");
    // let input = SAMPLE;

    let end_of_page_ordering_rules_section = input.find("\n\n").unwrap();

    let page_ordering_rules_section = &input[0..=end_of_page_ordering_rules_section];
    let rules = parse_page_ordering_rules(page_ordering_rules_section);
    let mut sum_of_mid_pages = 0;
    let mut sum_of_fixed_mid_pages = 0;

    for update in input[end_of_page_ordering_rules_section..].trim().lines() {
        let update = update
            .split(',')
            .map(|x| x.parse::<u16>().unwrap())
            .collect();

        let (is_allowed, maybe_fixed_update) = rules.is_allowed_update(&update);

        if is_allowed {
            let mid_page = update[update.len() / 2];
            sum_of_mid_pages += mid_page;
        } else if let Some(fixed_update) = maybe_fixed_update {
            let mid_page = fixed_update[fixed_update.len() / 2];
            sum_of_fixed_mid_pages += mid_page;
        }
    }

    dbg!(sum_of_mid_pages);
    dbg!(sum_of_fixed_mid_pages);
}

fn parse_page_ordering_rules(input: impl AsRef<str>) -> PageOrderingRules {
    let input = input.as_ref();

    let mut rules: HashMap<u16, Vec<u16>> = HashMap::new();

    for rule in input.lines() {
        let (before, after) = rule.split_at(rule.find('|').unwrap());

        let (before, after) = (before.parse().unwrap(), after[1..].parse().unwrap()); // ignore separation character

        rules.entry(before).or_default().push(after);
    }

    PageOrderingRules { rules }
}
