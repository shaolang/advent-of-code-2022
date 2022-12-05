use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};

pub fn run() {
    let f = fs::File::open("data/03-rucksacks.txt").unwrap();
    let reader = BufReader::new(f);

    let sum_of_dup_item_priorities: u32 = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.trim().len() > 0)
        .map(|l| find_duplicates_in_rucksack(&l))
        .sum();

    println!("\nDay 03 Answers:");
    println!(
        "Sum of dup. item priorities: {:}",
        sum_of_dup_item_priorities
    );

    let f = fs::File::open("data/03-rucksacks.txt").unwrap();
    let reader = BufReader::new(f);
    let sum_of_group_item_priorities: u32 = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.trim().len() > 0)
        .collect::<Vec<String>>()
        .as_slice()
        .chunks(3)
        .map(|chunk| find_group_item(chunk))
        .sum();

    println!(
        "Sum of group item priorities: {:}",
        sum_of_group_item_priorities
    );
}

fn find_duplicates_in_rucksack(s: &str) -> u32 {
    let mid = s.len() / 2;
    let (first, second) = s.split_at(mid);
    let first = first.chars().collect::<HashSet<char>>();
    let second = second.chars().collect::<HashSet<char>>();
    let common = first.intersection(&second);

    common.into_iter().map(as_priority).sum()
}

fn find_group_item(group: &[String]) -> u32 {
    group
        .iter()
        .map(|s| s.chars().collect())
        .reduce(|accum: HashSet<char>, rucksack| {
            accum
                .intersection(&rucksack)
                .map(|c| c.to_owned())
                .collect()
        })
        .unwrap()
        .iter()
        .map(|c| as_priority(c))
        .sum()
}

fn as_priority(c: &char) -> u32 {
    let v = *c as u32;

    if v < 97 {
        v - 64 + 26
    } else {
        v - 96
    }
}
