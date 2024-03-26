use std::fs;

fn part1(lines: &Vec<String>) -> u32 {
    let mut output: u32 = 0;
    for line in lines {
        if line.starts_with("[STOP]") {
            output += 1;
        }
    }
    return output;
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
