use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let f = fs::File::open("calories-input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut elf_with_most_cals = Elf::new(0);
    let mut curr_elf = Elf::new(1);

    for line in reader.lines() {
        let line = line.unwrap();

        if line.trim() == "" {
            elf_with_most_cals = if elf_with_most_cals.carries_more(&curr_elf) {
                elf_with_most_cals
            } else {
                curr_elf
            };

            curr_elf = Elf::new(curr_elf.id + 1);
        } else {
            curr_elf.add_calories(line.parse().unwrap());
        }
    }

    println!(
        "Elf {} carries a total of {} calories.",
        elf_with_most_cals.id, elf_with_most_cals.calories_carried
    );
}

#[derive(Clone, Copy)]
struct Elf {
    id: u16,
    calories_carried: u64,
}

impl Elf {
    fn new(id: u16) -> Self {
        Elf {
            id: id,
            calories_carried: 0,
        }
    }

    fn carries_more(&self, other: &Elf) -> bool {
        self.calories_carried > other.calories_carried
    }

    fn add_calories(&mut self, calories: u64) {
        self.calories_carried += calories;
    }
}
