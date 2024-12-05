use std::collections::{HashMap, HashSet, VecDeque};
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

    let mut before_orderings = HashMap::new();
    let mut getting_orderings = true;
    let mut sum = 0;

    'main: for line in BufReader::new(file).lines().flatten() {
        if getting_orderings {
            if let Some((l, r)) = line.split_once('|') {
                before_orderings
                    .entry(r.parse::<i32>().unwrap())
                    .or_insert(HashSet::new())
                    .insert(l.parse::<i32>().unwrap());
                before_orderings
                    .entry(l.parse::<i32>().unwrap())
                    .or_insert(HashSet::new());
            } else {
                getting_orderings = false;
            }
        } else {
            let update: Vec<i32> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
            let mut prev_pages = HashSet::new();
            for page in &update {
                if prev_pages.difference(&before_orderings[page]).count() > 0 {
                    continue 'main;
                }
                prev_pages.insert(*page);
            }
            sum += update[update.len() / 2];
        }
    }

    Ok(println!("part 1: sum: {}", sum))
}

fn part_two(file_name: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_name)?;

    let mut before_orderings = HashMap::new();
    let mut after_orderings = HashMap::new();
    let mut getting_orderings = true;
    let mut sum = 0;

    'main: for line in BufReader::new(file).lines().flatten() {
        if getting_orderings {
            if let Some((l, r)) = line.split_once('|') {
                before_orderings
                    .entry(r.parse::<i32>().unwrap())
                    .or_insert(HashSet::new())
                    .insert(l.parse::<i32>().unwrap());
                before_orderings
                    .entry(l.parse::<i32>().unwrap())
                    .or_insert(HashSet::new());
                after_orderings
                    .entry(l.parse::<i32>().unwrap())
                    .or_insert(HashSet::new())
                    .insert(r.parse::<i32>().unwrap());
                after_orderings
                    .entry(r.parse::<i32>().unwrap())
                    .or_insert(HashSet::new());
            } else {
                getting_orderings = false;
            }
        } else {
            let update: Vec<i32> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
            let mut prev_pages = HashSet::new();
            for page in &update {
                if prev_pages.difference(&before_orderings[page]).count() > 0 {
                    sum += solve_order(&update, &before_orderings, &after_orderings);
                    continue 'main;
                }
                prev_pages.insert(*page);
            }
        }
    }

    Ok(println!("part 2: sum: {}", sum))
}

fn solve_order(
    nums: &Vec<i32>,
    before_orderings: &HashMap<i32, HashSet<i32>>,
    after_orderings: &HashMap<i32, HashSet<i32>>,
) -> i32 {
    let mut nums_queue: VecDeque<i32> = nums.clone().into();
    let mut actual_order = Vec::new();
    while nums_queue.len() > 0 {
        let num = nums_queue.pop_front().unwrap();
        if (HashSet::from_iter(actual_order.clone())
            .difference(&before_orderings[&num])
            .count()
            > 0)
            || (HashSet::from_iter(nums_queue.clone())
                .difference(&after_orderings[&num])
                .count()
                > 0)
        {
            nums_queue.push_back(num);
            continue;
        }
        actual_order.push(num);
    }
    actual_order[actual_order.len() / 2]
}
