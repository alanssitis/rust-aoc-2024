use regex::Regex;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(format!("expected usage: {} <file-name>", args[0]).into());
    }

    // part_one(&args[1])
    part_two(&args[1])
}

fn part_one(file_name: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_name)?;
    let re = Regex::new(r"mul\((\d+),(\d+)\)")?;
    let mut sum = 0;

    for line in BufReader::new(file).lines().flatten() {
        sum += re
            .captures_iter(&line)
            .map(|c| c.extract())
            .map(|(_, [a, b])| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap())
            .sum::<i32>();
    }

    Ok(println!("part 1: sum: {}", sum))
}

fn part_two(file_name: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_name)?;
    let re_main = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don\'t\(\))")?;
    let re_sub = Regex::new(r"mul\((\d+),(\d+)\)")?;
    let mut sum = 0;
    let mut enabled = true;

    for line in BufReader::new(file).lines().flatten() {
        for m in re_main.find_iter(&line) {
            match m.as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                s => {
                    if enabled {
                        let (_, [a, b]) = re_sub.captures(s).unwrap().extract();
                        sum += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
                    }
                }
            }
        }
    }

    Ok(println!("part 2: sum: {}", sum))
}
