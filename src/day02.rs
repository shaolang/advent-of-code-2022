use std::fs;
use std::io::{BufRead, BufReader};

pub fn run() {
    println!("\nDay 02 Answers:");
    let score_xyz_as_shapes = cal_score_from_guide(line_to_shapes_xyz_shapes);
    println!("Total score (XYZ as shapes)  : {}", score_xyz_as_shapes);

    let score_xyz_as_outcomes = cal_score_from_guide(line_to_shapes_xyz_outcomes);
    println!("Total score (XYZ as outcomes): {}", score_xyz_as_outcomes);
}

fn cal_score_from_guide<F>(func: F) -> u16
where
    F: Fn(&str) -> Vec<Shape>,
{
    let f = fs::File::open("data/02-strategy-guide.txt").unwrap();
    let reader = BufReader::new(f);

    reader
        .lines()
        .map(|line| line.expect("Problem reading file"))
        .map(|line| {
            let shapes = func(&line);
            shapes.last().unwrap().score(*shapes.first().unwrap())
        })
        .sum()
}

fn line_to_shapes_xyz_shapes(line: &str) -> Vec<Shape> {
    line.split(' ').map(Shape::from_string).collect()
}

fn line_to_shapes_xyz_outcomes(line: &str) -> Vec<Shape> {
    let ss = line
        .split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let theirs = Shape::from_string(ss.first().unwrap());
    let mine = match ss.last().unwrap().as_str() {
        "X" => match theirs {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            _ => Shape::Paper,
        },
        "Y" => theirs,
        _ => match theirs {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            _ => Shape::Rock,
        },
    };

    vec![theirs, mine]
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
