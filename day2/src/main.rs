use std::fs;

fn part1(lines: &Vec<String>) -> u32 {
    return lines
        .iter()
        .map(|line| line.chars().filter(|c| *c == '.').count() as u32)
        .sum::<u32>();
}

fn part2(contents: &Vec<String>) -> u32 {
    let mut output = 0;
    let rows = contents.len();
    let cols = contents[0].len();
    let square_size = 15;

    for row in 0..rows - square_size + 1 {
        for col in 0..cols - square_size + 1 {
            if (0..square_size).all(|i| {
                (0..square_size).all(|j| contents[row + i].chars().nth(col + j).unwrap() != '#')
            }) {
                output += 1;
            }
        }
    }

    return output;
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
