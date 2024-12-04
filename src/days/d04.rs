use core::str;
use std::cmp;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

// SHOULD have used GRIDS smh, would have made my life much easier
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
    let mut sum = 0;
    let mut vertical_lines = Vec::new();

    let mut diagonal_idx = 0;
    let mut diagonal_left_lines = Vec::new();
    let mut diagonal_right_lines = Vec::new();

    for line in BufReader::new(file).lines().flatten() {
        sum += xmas_substr_count(line.as_bytes());

        if vertical_lines.len() == 0 {
            vertical_lines.resize(line.as_bytes().len(), Vec::new())
        }
        if diagonal_left_lines.len() == 0 {
            diagonal_left_lines.resize(line.as_bytes().len(), Vec::new())
        } else {
            diagonal_left_lines.push(Vec::new());
        }
        if diagonal_right_lines.len() == 0 {
            diagonal_right_lines.resize(line.as_bytes().len(), Vec::new())
        } else {
            diagonal_right_lines.push(Vec::new());
        }

        for (pos, char) in line.as_bytes().iter().enumerate() {
            vertical_lines[pos].push(*char);
            diagonal_left_lines[pos + diagonal_idx].push(*char);
            diagonal_right_lines[line.len() - pos - 1 + diagonal_idx].push(*char);
        }
        diagonal_idx += 1;
    }
    for line in &vertical_lines {
        sum += xmas_substr_count(line);
    }
    for line in &diagonal_right_lines {
        sum += xmas_substr_count(line);
    }
    for line in &diagonal_left_lines {
        sum += xmas_substr_count(line);
    }

    Ok(println!("part 1: sum: {}", sum))
}

fn xmas_substr_count(line: &[u8]) -> usize {
    let substr = "XMAS";
    let rev_substr = "SAMX";
    line.windows(4)
        .filter(|&w| w == substr.as_bytes() || w == rev_substr.as_bytes())
        .count()
}

fn part_two(file_name: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_name)?;
    let mut sum = 0;

    let mut diagonal_idx = 0;
    let mut diagonal_left_lines = Vec::new();
    let mut diagonal_right_lines = Vec::new();
    let mut width = 0;

    for line in BufReader::new(file).lines().flatten() {
        if width == 0 {
            width = line.as_bytes().len();
        }
        if diagonal_left_lines.len() == 0 {
            diagonal_left_lines.resize(line.as_bytes().len(), Vec::new())
        } else {
            diagonal_left_lines.push(Vec::new());
        }
        if diagonal_right_lines.len() == 0 {
            diagonal_right_lines.resize(line.as_bytes().len(), Vec::new())
        } else {
            diagonal_right_lines.push(Vec::new());
        }

        for (pos, char) in line.as_bytes().iter().enumerate() {
            diagonal_left_lines[pos + diagonal_idx].push(*char);
            diagonal_right_lines[line.len() - pos - 1 + diagonal_idx].push(*char);
        }
        diagonal_idx += 1;
    }

    for (l_idx, line) in diagonal_left_lines.iter().enumerate() {
        for (w_idx, w) in line.windows(3).enumerate() {
            if !(w == b"SAM" || w == b"MAS") {
                continue;
            }
            let (dr_l_idx, dr_a_idx) = convert_to_dr_idx(l_idx as i32, w_idx as i32, width as i32);
            sum += match (
                diagonal_right_lines[dr_l_idx][dr_a_idx - 1],
                diagonal_right_lines[dr_l_idx][dr_a_idx + 1],
            ) {
                (b'S', b'M') | (b'M', b'S') => 1,
                _ => 0,
            };
        }
    }

    Ok(println!("part 2: sum: {}", sum))
}

// One hour worth of math because I am out of practice lmao
fn convert_to_dr_idx(l_idx: i32, w_idx: i32, width: i32) -> (usize, usize) {
    let a_idx = w_idx + 1;
    let y = a_idx + cmp::max(0, l_idx - width + 1);
    let dr_l_idx = width - 1 - l_idx + (2 * y);
    (
        dr_l_idx as usize,
        (y - cmp::max(0, dr_l_idx - width + 1)) as usize,
    )
}
