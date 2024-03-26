use std::fs;

fn part1(file_contents: &String) -> u32 {
    let mut output: u32 = 0;
    for line in file_contents.lines() {
        if line.starts_with("[STOP]") {
            output += 1;
        }
    }
    return output;
}

fn part2(file_contents: &String) -> u32 {
    return part1(&file_contents) + file_contents.lines().count() as u32;
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Unable to read file");
    println!("Part 1 Result: {:?}", part1(&file_contents));
    println!("Part 2 Result: {:?}", part2(&file_contents));
}
