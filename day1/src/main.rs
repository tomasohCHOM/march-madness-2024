use std::fs;

fn day1(file_contents: String) -> (u32, u32) {
    let mut part1: u32 = 0;
    for line in file_contents.lines() {
        if line.starts_with("[STOP]") {
            part1 += 1;
        }
    }

    let part2 = part1 + file_contents.lines().count() as u32;

    return (part1, part2);
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Unable to read file");
    println!("{:?}", day1(file_contents));
}
