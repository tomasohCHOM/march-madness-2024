use std::fs;

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

    println!("{:?}", lines);
}
