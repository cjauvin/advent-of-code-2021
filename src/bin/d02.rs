use std::fs::File;
use std::io::{BufRead, BufReader};

static S: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";

fn get_input(filename: Option<&str>) -> Vec<(String, i32)> {
    let mut vec = Vec::new();
    match filename {
        None => {
            for line in S.lines() {
                let parts: Vec<&str> = line.split(' ').collect();
                vec.push((parts[0].to_string(), parts[1].parse::<i32>().unwrap()));
            }
            vec
        }
        Some(filename) => {
            let f = File::open(filename).expect("Unable to open file");
            let f = BufReader::new(f);
            for line in f.lines() {
                let line = line.expect("Unable to read line");
                let parts: Vec<&str> = line.split(' ').collect();
                vec.push((parts[0].to_string(), parts[1].parse::<i32>().unwrap()));
            }
            vec
        }
    }
}

fn part1(vec: &Vec<(String, i32)>) -> i32 {
    let mut horiz = 0;
    let mut depth = 0;
    for parts in vec {
        //println!("{}", line);
        // assert_eq!(parts.len(), 2);
        match parts.0.as_str() {
            "forward" => {
                horiz += parts.1;
            }
            "down" => {
                depth += parts.1;
            }
            "up" => {
                depth -= parts.1;
            }
            _ => {
                panic!("??");
            }
        }
    }
    horiz * depth
}

fn part2(vec: &Vec<(String, i32)>) -> i32 {
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;
    for parts in vec {
        //println!("{}", line);
        // assert_eq!(parts.len(), 2);
        match parts.0.as_str() {
            "forward" => {
                horiz += parts.1;
                depth += aim * parts.1;
            }
            "down" => {
                aim += parts.1;
            }
            "up" => {
                aim -= parts.1;
            }
            _ => {
                panic!("??");
            }
        }
    }
    horiz * depth
}

fn main() {
    let vec = get_input(Some("data/d02.txt"));
    // let vec = get_input(None);

    assert_eq!(part1(&vec), 1524750);
    assert_eq!(part2(&vec), 1592426537);

    println!("ok!");
}
