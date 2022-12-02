use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let f = fs::File::open("calories-input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut most: u64 = 0;
    let mut curr: u64 = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.trim() == "" {
            most = if most > curr { most } else { curr };
            curr = 0;
        } else {
            curr += line.parse::<u64>().unwrap();
        }
    }

    println!("Most calories {}.", most);
}
