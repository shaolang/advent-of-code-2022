use std::collections::BTreeSet;
use std::fs;
use std::io::{BufRead, BufReader};

pub fn run() {
    let f = fs::File::open("data/01-calories-input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut tops = BTreeSet::<u64>::new();
    let mut curr: u64 = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.trim() == "" {
            tops.insert(curr);

            if tops.len() > 3 {
                tops = tops.into_iter().skip(1).collect();
            }

            curr = 0;
        } else {
            curr += line.parse::<u64>().unwrap();
        }
    }

    println!("Day 01 Answers:");
    println!("            Most calories: {}", tops.iter().last().unwrap());
    println!(
        "Total calories from top 3: {}",
        tops.into_iter().sum::<u64>()
    );
}
