use chrono::{FixedOffset, NaiveDate};
use std::collections::{HashMap, HashSet};
use std::fs;

fn part1(lines: &Vec<String>) -> u32 {
    let mut logs: HashMap<String, HashMap<String, String>> = HashMap::new();
    for line in lines {
        let parts: Vec<String> = line.split(":").map(str::to_string).collect();
        let time: &String = &parts[0];
        let other: &String = &(parts[1].trim().to_string());
        let other_parts: Vec<String> = other.split(" ").map(str::to_string).collect();
        if other_parts[1] == "->" {
            logs.entry(other_parts[0].to_string())
                .or_insert_with(HashMap::new)
                .insert(other_parts[2].to_string(), time.to_string());
        } else {
            logs.get_mut(&other_parts[0])
                .unwrap()
                .remove(&other_parts[2]);
        }
    }

    let mut output: u32 = 0;
    for v in logs.values() {
        if let Some(time) = v.values().nth(0) {
            output = time.parse::<u32>().unwrap();
        }
    }
    return output;
}

fn part2(lines: &Vec<String>, unix_time: u32) -> u32 {
    let timezone_offset = -7 * 3600;
    let timezone = FixedOffset::west_opt(timezone_offset).unwrap();
    fn unix_timestamp_to_day(time: i64, timezone: chrono::FixedOffset) -> NaiveDate {
        let datetime = chrono::DateTime::from_timestamp(time, 0).unwrap();
        return datetime.with_timezone(&timezone).date_naive();
    }
    let sus_date = unix_timestamp_to_day(unix_time as i64, timezone);
    let mut suspects: HashSet<String> = HashSet::new();
    let mut entries = 0;
    for line in lines {
        let parts: Vec<String> = line.split(":").map(str::to_string).collect();
        let time: &String = &parts[0];
        let other: &String = &(parts[1].trim().to_string());
        let other_parts: Vec<String> = other.split(" ").map(str::to_string).collect();
        if unix_timestamp_to_day(time.parse::<i64>().unwrap(), timezone) == sus_date
            && other_parts[1] == "->"
        {
            suspects.insert(other_parts[0].to_string());
            entries += 1;
        }
    }
    println!("{:?}", suspects);
    return (suspects.len() * entries) as u32;
}

fn main() {
    let mut lines: Vec<String> = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .lines()
        .map(String::from)
        .collect();
    lines.sort_by(|a, b| {
        a.split(':')
            .nth(0)
            .unwrap()
            .cmp(b.split(':').nth(0).unwrap())
    });

    let part1 = part1(&lines);
    println!("Part 1 Result: {:?}", part1);
    println!("Part 2 Result: {:?}", part2(&lines, part1));
}
