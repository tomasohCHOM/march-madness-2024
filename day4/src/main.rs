use std::f64::consts::PI;
use std::fs;

fn area_of_intersection(c1_x: f64, c1_y: f64, r1: f64, c2_x: f64, c2_y: f64, r2: f64) -> f64 {
    let distance = ((c1_x - c2_x).powi(2) + (c1_y - c2_y).powi(2)).sqrt();
    if distance > r1 + r2 {
        return 0.0;
    } else if distance < f64::abs(r1 - r2) {
        return PI * f64::min(r1, r2).powi(2);
    }
    let area_c1 = PI * r1.powi(2);
    let area_c2 = PI * r2.powi(2);
    let a = r1;
    let b = r2;
    let c = distance;
    let angle = (a.powi(2) + c.powi(2) - b.powi(2)) / (2.0 * a * c);
    let theta = 2.0 * f64::acos(angle);

    let overlap_sector = (theta / (2.0 * PI)) * area_c1;

    let triangle_area =
        0.5 * distance * f64::sqrt(f64::max(0.0, r1.powi(2) + distance.powi(2) - r2.powi(2)));

    return overlap_sector - triangle_area;
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

fn part2(captures: &Vec<(i32, i32, i32)>) -> f64 {
    let mut output = 0.0;
    for i in 0..captures.len() {
        for j in (i + 1)..captures.len() {
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
    return output;
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
    println!("{}", part1(&captures));
    println!("{}", part2(&captures));
}
