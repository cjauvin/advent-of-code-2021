use std::fs::File;
use std::io::{BufRead, BufReader};
use counter::Counter;

static S: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

fn get_input(filename: Option<&str>) -> Vec<Vec<u32>> {
    let mut vec = Vec::new();
    match filename {
        None => {
            for line in S.lines() {
                vec.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
            }
            vec
        }
        Some(filename) => {
            let f = File::open(filename).unwrap();
            let f = BufReader::new(f);
            for line in f.lines() {
                let line = line.unwrap();
                vec.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
            }
            vec
        }
    }
}

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn get_bigger(cntr: &Counter<u32>) -> u32 {
    if cntr[&0] > cntr[&1] {
        0
    } else if cntr[&0] < cntr[&1] {
        1
    } else {
        1
    }
}

fn get_smaller(cntr: &Counter<u32>) -> u32 {
    if cntr[&0] > cntr[&1] {
        1
    } else if cntr[&0] < cntr[&1] {
        0
    } else {
        0
    }
}

fn get_value(vec: &Vec<u32>) -> u32 {
    let mut v = 0;
    let n = vec.len() as u32;
    for i in 0..n {
        let p = 2_u32.pow(n - i - 1);
        v += vec[i as usize] * p;
    }
    v
}

fn part1(vec: &Vec<Vec<u32>>) -> u32 {
    let mut cntrs = Vec::new();

    let n = vec[0].len() as u32;

    for j in 0..n {
        let mut vjs: Vec<u32> = Vec::new();
        for v in vec {
            vjs.push(v[j as usize]);
        }
        let cntr = vjs.iter().cloned().collect::<Counter<_>>();
        cntrs.push(cntr);
    }

    let fns = [get_bigger, get_smaller];
    let mut values = [0, 0];

    for (fi, f) in fns.iter().enumerate() {
        for i in 0..n {
            let p = 2_u32.pow(n - i - 1);
            values[fi] += f(&cntrs[i as usize]) * p;
        }
    }

    values[0] * values[1]
}

fn part2(vec: &Vec<Vec<u32>>) -> u32 {
    let n = vec[0].len() as u32;

    let fns = [get_bigger, get_smaller];
    let mut values = [0, 0];

    for (fi, f) in fns.iter().enumerate() {
        let mut rem = vec.clone();
        for j in 0..n {
            let mut vjs: Vec<u32> = Vec::new();
            for v in &rem {
                vjs.push(v[j as usize]);
            }
            let cntr = vjs.iter().cloned().collect::<Counter<_>>();
            let b = f(&cntr);
            // println!("{:?}: {}", cntr, b);

            rem.retain(|x| x[j as usize] == b);
            if rem.len() == 1 {
                break;
            }
            //println!("{}", rem.len());
        }
        // println!("----");
        assert_eq!(rem.len(), 1);
        values[fi] = get_value(&rem[0]);
        // println!("{:?}, {}", &rem[0], values[fi]);
    }
    values[0] * values[1]
}

fn main() {
    let vec = get_input(Some("data/d03.txt"));
    // let vec = get_input(None);

    assert_eq!(part1(&vec), 2595824);
    assert_eq!(part2(&vec), 2135254);

    println!("ok!");
}
