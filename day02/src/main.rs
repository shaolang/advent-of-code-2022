use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let f = fs::File::open("strategy-guide.txt").unwrap();
    let reader = BufReader::new(f);
    let total_score = reader
        .lines()
        .map(|line| line.expect("Problem reading file"))
        .map(|line| {
            let shapes = line_to_shapes(&line);
            shapes.last().unwrap().score(*shapes.first().unwrap())
        })
        .sum::<u16>();

    println!("Total score: {}", total_score);
}

fn line_to_shapes(line: &str) -> Vec<Shape> {
    line.split(' ').map(Shape::from_string).collect()
}

#[derive(Copy, Clone, PartialEq)]
enum Shape {
    Scissors,
    Paper,
    Rock,
}

impl Shape {
    fn from_string(s: &str) -> Self {
        match s {
            "A" => Shape::Rock,
            "X" => Shape::Rock,
            "B" => Shape::Paper,
            "Y" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => Shape::Scissors,
        }
    }

    fn score(self, other: Self) -> u16 {
        if self == Shape::Rock {
            1 + match other {
                Shape::Rock => 3,
                Shape::Paper => 0,
                Shape::Scissors => 6,
            }
        } else if self == Shape::Paper {
            2 + match other {
                Shape::Rock => 6,
                Shape::Paper => 3,
                Shape::Scissors => 0,
            }
        } else {
            3 + match other {
                Shape::Rock => 0,
                Shape::Paper => 6,
                Shape::Scissors => 3,
            }
        }
    }
}
