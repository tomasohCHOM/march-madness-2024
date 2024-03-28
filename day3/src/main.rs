use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn part1(ingredients_list: &HashMap<String, Vec<String>>, base: &String) -> u32 {
    let mut q: VecDeque<&String> = VecDeque::from([base]);
    let mut output: HashSet<&String> = HashSet::new();
    while !q.is_empty() {
        let ingredient = q.pop_front().unwrap();
        output.insert(ingredient);
        if ingredients_list.contains_key(ingredient) {
            for item in ingredients_list.get(ingredient).unwrap() {
                q.push_back(item);
            }
        }
    }
    output.remove(base);
    return output.len() as u32;
}

fn part2(ingredients_list: &HashMap<String, Vec<String>>, needed_list: &Vec<String>) -> u32 {
    let mut output: HashSet<&String> = HashSet::new();
    for needed in needed_list {
        let mut q: VecDeque<&String> = VecDeque::from([needed]);
        while !q.is_empty() {
            let ingredient = q.pop_front().unwrap();
            if ingredients_list.contains_key(ingredient) {
                for item in ingredients_list.get(ingredient).unwrap() {
                    q.push_back(item);
                }
            } else {
                output.insert(ingredient);
            }
        }
    }
    return output.len() as u32;
}

fn main() {
    let lines: Vec<String> = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .lines()
        .map(String::from)
        .collect();

    let mut ingredients_list: HashMap<String, Vec<String>> = HashMap::new();
    for i in 2..lines.len() {
        let list: Vec<String> = lines[i].split(" = ").map(str::to_string).collect();
        let k = list[0].to_string();
        let v = list[1]
            .to_string()
            .split(" + ")
            .map(str::to_string)
            .collect();
        ingredients_list.insert(k, v);
    }

    let needed = (&lines[0]).split(": ").nth(1).unwrap().to_string();
    let needed_list: Vec<String> = needed.split(", ").map(str::to_string).collect();

    println!(
        "Part 1 Result: {:?}",
        part1(&ingredients_list, &needed_list[0])
    );
    println!(
        "Part 2 Result: {:?}",
        part2(&ingredients_list, &needed_list)
    );
}
