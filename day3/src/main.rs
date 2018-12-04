use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Pos {
    x: u64,
    y: u64,
}

impl Pos {
    pub fn new(x: u64, y: u64) -> Pos {
        return Pos { x: x, y: y };
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Claim {
    id: u64,
    top_left: Pos,
    width: u64,
    height: u64,
}

impl Claim {
    pub fn add_to_coord_space(
        &self,
        coord_space: &mut HashMap<Pos, Vec<Claim>>,
    ) {
        for x in self.top_left.x..(self.top_left.x + self.width) {
            for y in self.top_left.y..(self.top_left.y + self.height) {
                let p = Pos::new(x, y);
                match coord_space.get(&p) {
                    Some(v) => {
                        let mut new_vec = v.clone();
                        new_vec.push(self.clone());
                        coord_space.insert(p, new_vec);
                    }
                    None => {
                        let mut v = Vec::new();
                        v.push(self.clone());
                        coord_space.insert(p, v);
                    }
                };
            }
        }
    }
}

fn main() -> Result<()> {
    let file = File::open("input")?;
    let mut lines: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        lines.push(line.unwrap());
    }
    let claim_regex =
        Regex::new(r"^#(\d*) @ (\d*),(\d*): (\d*)x(\d*)$").unwrap();
    let mut claims: Vec<Claim> = Vec::new();
    for line in lines.iter() {
        for cap in claim_regex.captures_iter(line) {
            let id = cap[1].parse::<u64>().unwrap();
            let top_left = Pos {
                x: cap[2].parse::<u64>().unwrap(),
                y: cap[3].parse::<u64>().unwrap(),
            };
            let width = cap[4].parse::<u64>().unwrap();
            let height = cap[5].parse::<u64>().unwrap();
            claims.push(Claim {
                id: id,
                top_left: top_left.clone(),
                width: width,
                height: height,
            });
        }
    }
    let mut coordinate_space: HashMap<Pos, Vec<Claim>> = HashMap::new();
    for claim in claims.iter() {
        claim.add_to_coord_space(&mut coordinate_space);
    }
    part_one(coordinate_space.clone());
    part_two(coordinate_space, claims);
    Ok(())
}

fn part_one(coordinate_space: HashMap<Pos, Vec<Claim>>) {
    let mut num_gr_2 = 0;
    for (_c, n) in coordinate_space.into_iter() {
        if n.len() >= 2 {
            num_gr_2 += 1;
        }
    }
    println!("Number of overlapping claims {}", num_gr_2);
}

fn part_two(coordinate_space: HashMap<Pos, Vec<Claim>>, claims: Vec<Claim>) {
    let mut claim_set = HashSet::new();
    for c in claims {
        claim_set.insert(c);
    }
    for (_p, n) in coordinate_space.into_iter() {
        if n.len() >= 2 {
            for c in n.into_iter() {
                claim_set.remove(&c);
            }
        }
    }
    println!("Claim that isn't overlapping: {:?}", claim_set);
}
