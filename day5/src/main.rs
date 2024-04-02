use chrono::{DateTime, Datelike, Local, TimeZone, Timelike, Utc};
use std::collections::HashMap;
use std::fs;

fn part1(lines: &Vec<String>) -> u32 {
    let mut logs: HashMap<String, HashMap<String, String>> = HashMap::new();
    for line in lines {
        let parts: Vec<String> = line.split(":").map(str::to_string).collect();
        let time: &String = &parts[0];
        let other: &String = &(parts[1].trim().to_string());
        let thing: Vec<String> = other.split(" ").map(str::to_string).collect();
        if thing[1] == "->" {
            logs.entry(thing[0].clone())
                .or_insert_with(HashMap::new)
                .insert(thing[2].to_string(), time.to_string());
        } else {
            logs.get_mut(&thing[0]).unwrap().remove(&thing[2]);
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

fn part2(lines: &Vec<String>, unix_time: u32) {}

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
}
