use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;

fn count_chars_line(line: String) -> HashMap<char, u64> {
    let mut counts = HashMap::new();
    for c in line.chars() {
        match counts.get(&c) {
            Some(count) => counts.insert(c, count + 1),
            None => counts.insert(c, 1)
        };
    }
    return counts;
}

#[test]
fn test_count_chars_line() {
    let example_line: String = "bababc".to_string();
    let example_counts = count_chars_line(example_line);
    assert_eq!(example_counts.get(&'b').unwrap(), &3);
    assert_eq!(example_counts.get(&'a').unwrap(), &2);
    assert_eq!(example_counts.get(&'c').unwrap(), &1);
}

fn main() -> Result<()> {
    let file = File::open("input")?;
    let mut lines: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        lines.push(line.unwrap());
    }
    part_one(lines.clone());
    part_two(lines);
    Ok(())
}

fn part_one(lines: Vec<String>) {
    let counted_lines: Vec<HashMap<char, u64>> = lines.into_iter().map(|l| {
        count_chars_line(l)
    }).collect();
    let mut two_count = 0;
    let mut three_count = 0;
    for counted in counted_lines.into_iter() {
        let mut three_add = 0;
        let mut two_add = 0;
        for (_c, count) in counted.into_iter() {
            if count == 3 {
                three_add = 1;
            }
            if count == 2 {
                two_add = 1;
            }
        }
        two_count += two_add;
        three_count += three_add;
    }
    println!("Two count: {}", two_count);
    println!("Three count: {}", three_count);
    println!("Multiplied: {}", two_count * three_count);
}

fn part_two(lines: Vec<String>) {
    let reference_lines = lines.clone();
    for line in lines.into_iter() {
        for compare_line in reference_lines.clone().into_iter() {
            let mut diffs = 0;
            let mut char_indices = compare_line.char_indices();
            let mut composite_string = String::new();
            for c in line.chars() {
                match char_indices.next() {
                    Some((_i, next_compare_char)) => {
                        if c != next_compare_char {
                            diffs += 1;
                        } else {
                            composite_string.push(c);
                        }
                    }
                    None => ()
                }
            }
            if diffs == 1 {
                println!("{} and {}", line, compare_line);
                println!("Composite: {}", composite_string);
                return;
            }
        }
    }
}
