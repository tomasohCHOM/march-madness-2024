use std::collections::HashMap;
use std::fs;

fn part1(ingredients_list: &HashMap<String, Vec<String>>) -> u32 {
    return 0;
}

fn part2(ingredients_list: &HashMap<String, Vec<String>>) -> u32 {
    return 0;
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
            .split(" = ")
            .map(str::to_string)
            .collect();
        ingredients_list.insert(k, v);
    }

    println!("Part 1 Result: {:?}", part1(&ingredients_list));
    println!("Part 2 Result: {:?}", part2(&ingredients_list));
}
