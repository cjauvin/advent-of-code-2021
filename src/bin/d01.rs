use std::fs::File;
use std::io::{BufRead, BufReader};

static S: &str = "199
200
208
210
200
207
240
269
260
263
";

fn part1(vec: &Vec<i32>) -> i32 {
    let mut n_increases = 0;
    for (i, _n) in vec.iter().enumerate() {
        if i > 0 && vec[i] > vec[i - 1] {
            n_increases += 1;
        }
    }
    n_increases
}

fn part2(vec: &Vec<i32>) -> i32 {
    let mut prev: Option<i32> = None;
    let mut curr: Option<i32> = None;
    let mut n_increases = 0;

    for (i, _n) in vec.iter().enumerate() {
        if i > 1 {
            curr = Some(vec[i-2..i+1].iter().sum());
            // println!("{} ({}): {}", prev, i, curr);
        }
        if let Some(c) = curr {
            if let Some(p) = prev {
                if c > p {
                    n_increases += 1;
                }
            }
            prev = curr;
        }
    }
    n_increases
}

fn get_input(filename: Option<&str>) -> Vec<i32> {
    let mut vec = Vec::new();
    match filename {
        None => {
            for line in S.lines() {
                vec.push(line.parse::<i32>().unwrap());
            }
            vec
        }
        Some(filename) => {
            let f = File::open(filename).unwrap();
            let f = BufReader::new(f);
            for line in f.lines() {
                let line = line.unwrap();
                let n: i32 = line.parse().unwrap();
                vec.push(n);
            }
            vec
        }
    }
}

fn main() {
    let vec = get_input(Some("data/d01.txt"));
    // let vec = get_input(None);

    // println!("{}", part1(&vec));
    // println!("{}", part2(&vec));

    assert_eq!(part1(&vec), 1121);
    assert_eq!(part2(&vec), 1065);

    println!("ok!");
}
