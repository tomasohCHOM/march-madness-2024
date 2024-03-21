use std::fs;

fn part1(file_contents: String) -> u32 {
    let mut output: u32 = 0;
    for line in file_contents.lines() {
        if line.starts_with("[STOP]") {
            output += 1;
        }
    }

    return output;
}

fn part2(file_contents: &String) -> u32 {
    let mut output: u32 = 0;
    output += part1(file_contents.to_string());
    output += file_contents.len() as u32;
    return output;
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Unable to read file");
    println!("{}", part1(file_contents));
    // println!("{}", part2(&file_contents));
}
