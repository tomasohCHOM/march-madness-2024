use std::fs;

fn area_of_intersection(c1_x: f64, c1_y: f64, r1: f64, c2_x: f64, c2_y: f64, r2: f64) -> f64 {
    let rr1: f64 = r1 * r1;
    let rr2: f64 = r2 * r2;
    let c: f64 = ((c2_x - c1_x).powi(2) + (c2_y - c1_y).powi(2)).sqrt();
    let phi: f64 = ((rr1 + (c * c) - rr2) / (2.0 * r1 * c)).acos() * 2.0;
    let theta: f64 = ((rr2 + (c * c) - rr1) / (2.0 * r2 * c)).acos() * 2.0;
    let area1: f64 = 0.5 * theta * rr2 - 0.5 * rr2 * theta.sin();
    let area2: f64 = 0.5 * phi * rr1 - 0.5 * rr1 * phi.sin();
    return area1 + area2;
}

fn part1(captures: &Vec<(i32, i32, i32)>) -> u32 {
    let mut output: u32 = 0;
    for i in 0..captures.len() {
        for j in (i + 1)..captures.len() {
            let xd = captures[i].0 - captures[j].0;
            let yd = captures[i].1 - captures[j].1;
            let dist: f32 = f32::sqrt((xd.pow(2) + yd.pow(2)) as f32);
            if dist <= (captures[i].2 + captures[j].2) as f32 {
                output += 1;
            }
        }
    }
    return output;
}

fn part2(captures: &Vec<(i32, i32, i32)>) -> u32 {
    let mut output = 0.0;
    for i in 0..captures.len() {
        for j in (i + 1)..captures.len() {
            let xd = captures[i].0 - captures[j].0;
            let yd = captures[i].1 - captures[j].1;
            let dist: f32 = f32::sqrt((xd.pow(2) + yd.pow(2)) as f32);
            if dist <= (captures[i].2 + captures[j].2) as f32 {
                let area: f64 = area_of_intersection(
                    captures[i].0 as f64,
                    captures[i].1 as f64,
                    captures[i].2 as f64,
                    captures[j].0 as f64,
                    captures[j].1 as f64,
                    captures[j].2 as f64,
                );

                if area > output {
                    output = area;
                }
            }
        }
    }
    return output as u32;
}

fn main() {
    let lines: Vec<String> = fs::read_to_string("file.txt")
        .expect("Unable to read file")
        .lines()
        .map(String::from)
        .collect();

    let mut captures: Vec<(i32, i32, i32)> = Vec::new();
    for line in lines {
        let x: i32 = line
            .split("x=")
            .nth(1)
            .unwrap()
            .split(",")
            .nth(0)
            .unwrap()
            .parse()
            .unwrap();
        let y: i32 = line
            .split("y=")
            .nth(1)
            .unwrap()
            .split(" ")
            .nth(0)
            .unwrap()
            .parse()
            .unwrap();
        let r: i32 = line
            .split("reach=")
            .nth(1)
            .unwrap()
            .split("ft")
            .nth(0)
            .unwrap()
            .parse()
            .unwrap();

        captures.push((x, y, r));
    }
    println!("Part 1 Result: {:?}", part1(&captures));
    println!("Part 2 Result: {:?}", part2(&captures));
}
