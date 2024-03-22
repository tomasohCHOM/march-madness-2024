use std::fs;

fn day2(file_contents: String) -> (u32, u32) {
    let part1 = file_contents.chars().filter(|c| *c == '.').count() as u32;

    let mut part2 = 0;
    let rows = file_contents.lines().count();
    let cols = file_contents.lines().nth(0).unwrap().len();
    let square_size = 15;

    for row in 0..rows - square_size {
        for col in 0..cols - square_size {
            let mut flag = true;
            for i in 0..square_size {
                for j in 0..square_size {
                    if file_contents
                        .lines()
                        .nth(row + i)
                        .unwrap()
                        .chars()
                        .nth(col + j)
                        .unwrap()
                        == '#'
                    {
                        flag = false;
                        break;
                    }
                }
                if !flag {
                    break;
                }
                part2 += 1;
                println!("{}", part2);
            }
        }
        println!("{:?}", file_contents.lines().nth(row).unwrap());
    }

    return (part1, part2);
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Unable to read file");
    println!("{:?}", day2(file_contents));
}
