use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq)]
enum SequenceInfo {
    Empty,
    First(i32),
    Node(i32),
}

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
    let range = 1..=3;
    let mut sum = 0;

    for line in BufReader::new(file).lines().flatten() {
        let levels: Vec<i32> = line
            .split(" ")
            .map(|val| val.parse::<i32>())
            .flatten()
            .collect();
        if levels[0] > levels[1] && levels.is_sorted_by(|a, b| range.contains(&(a - b))) {
            sum += 1
        } else if levels[0] < levels[1] && levels.is_sorted_by(|a, b| range.contains(&(b - a))) {
            sum += 1
        }
    }

    Ok(println!("part 1: sum: {}", sum))
}

fn part_two(file_name: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_name)?;
    let range = 1..=3;
    let mut sum = 0;

    for line in BufReader::new(file).lines().flatten() {
        let levels: Vec<i32> = line
            .split(" ")
            .map(|val| val.parse::<i32>())
            .flatten()
            .collect();
        if levels[0] > levels[1] && levels.is_sorted_by(|a, b| range.contains(&(a - b))) {
            sum += 1;
        } else if levels[0] < levels[1] && levels.is_sorted_by(|a, b| range.contains(&(b - a))) {
            sum += 1;
        } else {
            for i in 0..levels.len() {
                let mut new_levels = levels.clone();
                new_levels.remove(i);
                if new_levels[0] > new_levels[1]
                    && new_levels.is_sorted_by(|a, b| range.contains(&(a - b)))
                {
                    sum += 1;
                    break;
                } else if new_levels[0] < new_levels[1]
                    && new_levels.is_sorted_by(|a, b| range.contains(&(b - a)))
                {
                    sum += 1;
                    break;
                }
            }
        }
    }
    // TODO: finish sliding window solution I wanted to figure out
    // 'main_loop: for line in BufReader::new(file).lines().flatten() {
    //     let levels: Vec<i32> = line
    //         .split(" ")
    //         .map(|val| val.parse::<i32>())
    //         .flatten()
    //         .collect();
    //     if levels[0] > levels[1] && levels.is_sorted_by(|a, b| range.contains(&(a - b))) {
    //         sum += 1
    //     } else if levels[0] < levels[1] && levels.is_sorted_by(|a, b| range.contains(&(b - a))) {
    //         sum += 1
    //     } else {
    //         println!("{}", line);
    //         use SequenceInfo::*;
    //         let mut prev = Empty;
    //         let mut prevprev = Empty;
    //         let mut discarded = Empty;
    //         let mut incr = false;
    //         for curr in levels {
    //             match prev {
    //                 Empty => prev = First(curr),
    //                 First(p) => {
    //                     if range.contains(&(p - curr)) {
    //                         incr = false;
    //                     } else if range.contains(&(curr - p)) {
    //                         incr = true;
    //                     } else {
    //                         if let Node(disc) = discarded {
    //                             if range.contains(&(disc - curr)) {
    //                                 incr = false;
    //                             } else if range.contains(&(disc - p)) {
    //                                 incr = true;
    //                             } else {
    //                                 continue 'main_loop;
    //                             }
    //                             prev = discarded;
    //                             discarded = First(p);
    //                         } else {
    //                             discarded = Node(curr);
    //                             continue;
    //                         }
    //                     }
    //                     prevprev = prev;
    //                     prev = Node(curr);
    //                 }
    //                 Node(p) => {
    //                     if (incr && !range.contains(&(curr - p)))
    //                         || (!incr && !range.contains(&(p - curr)))
    //                     {
    //                         println!("here");
    //                         if discarded != Empty {
    //                             println!("unsafe");
    //                             continue 'main_loop;
    //                         } else {
    //                             discarded = Node(curr);
    //                             continue;
    //                         }
    //                     }
    //                     prevprev = prev;
    //                     prev = Node(curr);
    //                 }
    //             }
    //         }
    //         sum += 1;
    //     }
    // }

    Ok(println!("part 2: sum: {}", sum))
}
