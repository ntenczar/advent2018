use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;
use regex::Regex;
use chrono::prelude::*;

#[derive(Debug, Clone)]
struct TimeSlice {
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
}

#[derive(Debug, Clone)]
struct Guard {
    naps: Vec<TimeSlice>,
    shifts: Vec<TimeSlice>,
    id: u64,
}

impl Guard {
    pub fn new(id: u64) -> Guard {
        return Guard {
            naps: Vec::new(),
            shifts: Vec::new(),
            id: id,
        };
    }

    pub fn total_naptime_secs(&self) -> i64 {
        let mut total: i64 = 0;
        for nap in &self.naps {
            total += nap.end.signed_duration_since(nap.begin).num_seconds();
        }
        return total;
    }
}

fn main() -> Result<()> {
    let file = File::open("input")?;
    let mut lines: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        lines.push(line.unwrap());
    }
    let guards = parse_lines(lines);
    part_one(guards.clone());
    part_two(guards);
    Ok(())
}

fn part_two(guards: HashMap<u64, Guard>) {
    let mut highest = (0, 0);
    let mut guard_id = 0;
    for (_, guard) in guards.into_iter() {
        let mut sec_counts: HashMap<u64, u64> = HashMap::new();
        for nap in guard.naps.clone() {
            let start = nap.begin
                .time()
                .format("%M")
                .to_string()
                .parse::<u64>()
                .unwrap();
            let end = nap.end
                .time()
                .format("%M")
                .to_string()
                .parse::<u64>()
                .unwrap();
            for i in start..end {
                // rust does inclusive starts
                if let Some(x) = sec_counts.get_mut(&i) {
                    *x += 1;
                } else {
                    sec_counts.insert(i, 1);
                }
            }
        }
        let mut counts_vec: Vec<(u64, u64)> = sec_counts
            .into_iter()
            .map(|(ts, count)| (count, ts))
            .collect();
        counts_vec.sort();
        counts_vec.reverse(); // fuck why is this backward anyway
        if counts_vec.len() > 0 {
            let (c, ts) = counts_vec[0];
            let (c_highest, _ts_highest) = highest;
            if c > c_highest {
                highest = (c, ts);
                guard_id = guard.id;
            }
        }
    }
    println!("guard id: {}", guard_id);
    println!("{:?}", highest);
    println!("yada yada {}", guard_id*highest.1);
}

fn part_one(guards: HashMap<u64, Guard>) {
    let mut highest_naptime = 0;
    let mut highest_guard: Option<Guard> = None;
    for (_, guard) in guards.clone().into_iter() {
        if guard.total_naptime_secs() > highest_naptime {
            highest_naptime = guard.total_naptime_secs();
            highest_guard = Some(guard);
        }
    }
    let guard_id = highest_guard.unwrap().id;
    println!("Guard ID: {}", guard_id);
    println!("Total naptime {}", highest_naptime);
    let guard = guards.get(&guard_id).unwrap();
    let mut sec_counts: HashMap<u64, u64> = HashMap::new();
    for nap in guard.naps.clone() {
        // this feels awful but fuck it it's 2:15 AM
        let start = nap.begin
            .time()
            .format("%M")
            .to_string()
            .parse::<u64>()
            .unwrap();
        let end = nap.end
            .time()
            .format("%M")
            .to_string()
            .parse::<u64>()
            .unwrap();
        for i in start..end {
            // rust does inclusive starts
            if let Some(x) = sec_counts.get_mut(&i) {
                *x += 1;
            } else {
                sec_counts.insert(i, 1);
            }
        }
    }
    let mut counts_vec: Vec<(u64, u64)> = sec_counts
        .into_iter()
        .map(|(ts, count)| (count, ts))
        .collect();
    counts_vec.sort();
    counts_vec.reverse();
    // fuck it it's 761*25 = 19025
    println!("{:?}", counts_vec);
}

fn parse_lines(lines: Vec<String>) -> HashMap<u64, Guard> {
    let guard_start_regex =
        Regex::new(r"\[(\d*)-(\d*)-(\d*) (\d*):(\d*)] Guard #(\d*)").unwrap();
    let guard_sleep_regex =
        Regex::new(r"\[(\d*)-(\d*)-(\d*) (\d*):(\d*)] falls asleep").unwrap();
    let guard_wake_regex =
        Regex::new(r"\[(\d*)-(\d*)-(\d*) (\d*):(\d*)] wakes up").unwrap();
    let date_parse_regex =
        Regex::new(r"\[(\d*)-(\d*)-(\d*) (\d*):(\d*)]").unwrap();
    let mut guards: HashMap<u64, Guard> = HashMap::new();
    let mut lines_by_date: Vec<(DateTime<Utc>, String)> = Vec::new();
    for line in lines {
        let line_date = parse_date(&line, &date_parse_regex).unwrap();
        lines_by_date.push((line_date, line));
    }
    lines_by_date.sort();
    let mut previous_guard_id_opt: Option<u64> = None;
    let mut previous_date_opt: Option<DateTime<Utc>> = None;
    let mut is_awake = false;
    // forgive me father for I have sinned
    for (current_date, line) in lines_by_date {
        if guard_start_regex.is_match(&line) {
            match previous_guard_id_opt {
                Some(guard_id) => {
                    let ts = TimeSlice {
                        begin: previous_date_opt.unwrap(),
                        end: current_date,
                    };
                    match guards.get_mut(&guard_id) {
                        Some(g) => {
                            if is_awake {
                                g.shifts.push(ts);
                            } else {
                                g.naps.push(ts);
                            }
                        }
                        None => {
                            let mut guard = Guard::new(guard_id);
                            if is_awake {
                                guard.shifts.push(ts);
                            } else {
                                guard.naps.push(ts);
                            }
                            guards.insert(guard_id, guard);
                        }
                    }
                }
                None => (),
            }
            is_awake = true;
            let guard_id = parse_guard_id(&line, &guard_start_regex).unwrap();
            previous_guard_id_opt = Some(guard_id);
            previous_date_opt =
                Some(parse_date(&line, &date_parse_regex).unwrap());
        } else if guard_wake_regex.is_match(&line) {
            is_awake = true;
            let ts = TimeSlice {
                begin: previous_date_opt.unwrap(),
                end: current_date,
            };
            let guard_id = previous_guard_id_opt.unwrap();
            match guards.get_mut(&guard_id) {
                Some(g) => {
                    g.naps.push(ts);
                }
                None => {
                    let mut guard = Guard::new(guard_id);
                    guard.naps.push(ts);
                    guards.insert(guard_id, guard);
                }
            }
            previous_date_opt = Some(current_date);
        } else if guard_sleep_regex.is_match(&line) {
            is_awake = false;
            let ts = TimeSlice {
                begin: previous_date_opt.unwrap(),
                end: current_date,
            };
            let guard_id = previous_guard_id_opt.unwrap();
            match guards.get_mut(&guard_id) {
                Some(g) => {
                    g.shifts.push(ts);
                }
                None => {
                    let mut guard = Guard::new(guard_id);
                    guard.shifts.push(ts);
                    guards.insert(guard_id, guard);
                }
            }
            previous_date_opt = Some(current_date);
        }
    }
    return guards;
}

fn parse_guard_id(line: &String, guard_start_regex: &Regex) -> Option<u64> {
    for cap in guard_start_regex.captures_iter(&line) {
        let gid = cap[6].parse::<u64>().unwrap();
        return Some(gid);
    }
    None
}

fn parse_date(
    line: &String,
    date_parse_regex: &Regex,
) -> Option<DateTime<Utc>> {
    for cap in date_parse_regex.captures_iter(&line) {
        let year = cap[1].parse::<i32>().unwrap();
        let month = cap[2].parse::<u32>().unwrap();
        let day = cap[3].parse::<u32>().unwrap();
        let hour = cap[4].parse::<u32>().unwrap();
        let min = cap[5].parse::<u32>().unwrap();
        return Some(Utc.ymd(year, month, day).and_hms(hour, min, 0));
    }
    None
}
