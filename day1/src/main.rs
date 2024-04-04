use std::fs;

fn part1(lines: &Vec<String>) -> u32 {
    return lines
        .iter()
        .filter(|line| line.starts_with("[STOP]"))
        .count() as u32;
}

fn part2(lines: &Vec<String>) -> u32 {
    return part1(&lines) + lines.len() as u32;
}

fn main() {
    let lines: Vec<String> = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .lines()
        .map(String::from)
        .collect();
    println!("Part 1 Result: {:?}", part1(&lines));
    println!("Part 2 Result: {:?}", part2(&lines));
}
