use std::collections::{BinaryHeap, HashMap, HashSet};
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

    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();

    for line in BufReader::new(file).lines().flatten() {
        let (l, r) = line
            .split_once("   ")
            .ok_or("invalid file format: numbers not separated by 3 spaces")?;
        left_heap.push(l.parse::<i32>()?);
        right_heap.push(r.parse::<i32>()?);
    }

    let mut sum = 0;
    for (l, r) in left_heap
        .into_sorted_vec()
        .into_iter()
        .zip(right_heap.into_sorted_vec())
    {
        println!("l: {}, r: {}", l, r);
        sum += (l - r).abs();
    }

    Ok(println!("part 1: sum: {}", sum))
}

fn part_two(file_name: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_name)?;

    let mut nums = HashSet::new();
    let mut left_nums = HashMap::new();
    let mut right_nums = HashMap::new();

    for line in BufReader::new(file).lines().flatten() {
        let (l, r) = line
            .split_once("   ")
            .ok_or("invalid file format: numbers not separated by 3 spaces")?;
        let l = l.parse::<i32>()?;
        let r = r.parse::<i32>()?;
        nums.insert(l);
        *left_nums.entry(l).or_insert(0) += 1;
        *right_nums.entry(r).or_insert(0) += 1;
    }

    let mut sum = 0;
    for n in nums.into_iter() {
        sum += n * *left_nums.entry(n).or_default() * *right_nums.entry(n).or_default();
    }
    Ok(println!("part 2: sum: {}", sum))
}
