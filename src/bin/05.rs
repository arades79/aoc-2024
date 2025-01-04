use std::{
    cmp::Ordering, ops::{Deref, DerefMut}
};

advent_of_code::solution!(5);

use winnow::{
    ascii::dec_uint,
    combinator::{separated, separated_pair},
    prelude::*,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Rule(u32, u32);

fn rule(input: &mut &str) -> PResult<Rule> {
    let (one, two) = separated_pair(dec_uint, '|', dec_uint).parse_next(input)?;
    Ok(Rule(one, two))
}

impl Rule {
    fn find(self, pages: &[u32]) -> Option<(usize, usize)> {
        let (first, _) = pages
            .iter()
            .enumerate()
            .find(|(_, page)| **page == self.0)?;
        let (second, _) = pages
            .iter()
            .enumerate()
            .find(|(_, page)| **page == self.1)?;
        Some((first, second))
    }

    fn check(self, pages: &[u32]) -> bool {
        if let Some((first, second)) = self.find(pages) {
            first < second
        } else {
            true
        }
    }

    fn fix(self, pages: &mut [u32]) {
        if let Some((first, second)) = self.find(pages) {
            if first > second {
                pages[first] = self.0;
                pages[second] = self.1;
            }
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Update(Box<[u32]>);

impl Deref for Update {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Update {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn update(input: &mut &str) -> PResult<Update> {
    let pages: Vec<u32> = separated(1.., dec_uint::<_, u32, _>, ',').parse_next(input)?;
    Ok(Update(pages.into_boxed_slice()))
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rules = vec![];
    let mut middle_page_count = 0;
    'outer: for line in input.lines() {
        if let Ok(rule) = rule.parse(line) {
            rules.push(rule);
        }
        if let Ok(update) = update.parse(line) {
            for rule in rules.iter() {
                if !rule.check(&update) {
                    println!("rule checking failed on rule {rule:?} for update {update:?}");
                    continue 'outer;
                }
            }
            println!("update {update:?} passed all rules");
            let middle_page_index = update.0.len() / 2;
            let middle_page = update.0[middle_page_index];
            println!("adding middle page {middle_page} to success counter");
            middle_page_count += middle_page;
        }
    }
    Some(middle_page_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rules = vec![];
    let mut middle_page_count = 0;
    for line in input.lines() {
        if let Ok(rule) = rule.parse(line) {
            rules.push(rule);
        }
        if let Ok(mut update) = update.parse(line) {
            if !rules.iter().all(|rule| rule.check(&update)) {
                let middle_page_index = update.len() / 2;
                println!("trying to fix order for {update:?}...");

                update.sort_by(|&a,&b| {
                    if rules.contains(&Rule(a,b)) {
                        Ordering::Less
                    } else if rules.contains(&Rule(b,a)) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });
                
                    if rules.iter().all(|rule| rule.check(&update)) {
                        let middle_page = update[middle_page_index];
                        println!("\nFixed order! adding new middle page {middle_page} to success counter");
                        middle_page_count += middle_page;
                    }
                
            }
        }
    }
    Some(middle_page_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_parse() {
        let rule = rule.parse("23|48").unwrap();
        assert_eq!(rule, Rule(23, 48))
    }

    #[test]
    fn update_parse() {
        let Update(update) = update.parse("75,47,61,53,29").unwrap();
        assert_eq!(update, vec![75, 47, 61, 53, 29].into_boxed_slice())
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
