use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input")?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
    Ok(())
}

fn part_one(input: &[String]) -> u32 {
    let mut elves = vec![];
    let mut current: u32 = 0;
    for i in input.iter() {
        if i.is_empty() {
            elves.push(current);
            current = 0;
            continue;
        }
        current += i.parse::<u32>().unwrap();
    }
    return elves.into_iter().max().unwrap();
}
fn part_two(input: &[String]) -> u32 {
    let mut elves = vec![];
    let mut current: u32 = 0;
    for i in input.iter() {
        if i.is_empty() {
            elves.push(current);
            current = 0;
            continue;
        }
        current += i.parse::<u32>().unwrap();
    }
    elves.sort();
    elves.reverse();
    return elves.iter().take(3).sum();
}
