use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    let file = File::open("input")?;
    let mut nums: Vec<i64> = Vec::new();
    for line in BufReader::new(file).lines() {
        let l = line.unwrap();
        let to_int = l.parse::<i64>().unwrap();
        nums.push(to_int);
    }
    part_one(nums.clone());
    part_two(nums);
    Ok(())
}


fn part_one(init_vals: Vec<i64>) {
    let sum = init_vals.iter().fold(0, |acc, x| acc + x);
    println!("{}", sum);
}

fn part_two(init_vals: Vec<i64>) {
    let mut freqs = HashMap::new();
    let mut frequency: i64 = 0;
    freqs.insert(frequency, true);
    loop {
        for change in init_vals.iter() {
            frequency += change;
            match freqs.get(&frequency) {
                Some(_b) => {
                    println!("{}", frequency);
                    return;
                }
                _ => ()
            }
            freqs.insert(frequency, true);
        }
    }
}
