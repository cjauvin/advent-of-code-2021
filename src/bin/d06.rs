// use std::fs::File;
// use std::io::{BufRead, BufReader};
use regex::Regex;

//static S: &str = "3,4,3,1,2";
static S: &str = "1,1,3,1,3,2,1,3,1,1,3,1,1,2,1,3,1,1,3,5,1,1,1,3,1,2,1,1,1,1,4,4,1,2,1,2,1,1,1,5,3,2,1,5,2,5,3,3,2,2,5,4,1,1,4,4,1,1,1,1,1,1,5,1,2,4,3,2,2,2,2,1,4,1,1,5,1,3,4,4,1,1,3,3,5,5,3,1,3,3,3,1,4,2,2,1,3,4,1,4,3,3,2,3,1,1,1,5,3,1,4,2,2,3,1,3,1,2,3,3,1,4,2,2,4,1,3,1,1,1,1,1,2,1,3,3,1,2,1,1,3,4,1,1,1,1,5,1,1,5,1,1,1,4,1,5,3,1,1,3,2,1,1,3,1,1,1,5,4,3,3,5,1,3,4,3,3,1,4,4,1,2,1,1,2,1,1,1,2,1,1,1,1,1,5,1,1,2,1,5,2,1,1,2,3,2,3,1,3,1,1,1,5,1,1,2,1,1,1,1,3,4,5,3,1,4,1,1,4,1,4,1,1,1,4,5,1,1,1,4,1,3,2,2,1,1,2,3,1,4,3,5,1,5,1,1,4,5,5,1,1,3,3,1,1,1,1,5,5,3,3,2,4,1,1,1,1,1,5,1,1,2,5,5,4,2,4,4,1,1,3,3,1,5,1,1,1,1,1,1";

// naive!!
fn generation(fish: &mut Vec<u32>) {
    for i in 0..fish.len() {
        if fish[i] > 0 {
            fish[i] -= 1;
        } else {
            fish[i] = 6;
            fish.push(8);
        }
    }
}

fn main() {

    let re = Regex::new(r"\d+").unwrap();
    let mut fish: Vec<usize> = re.find_iter(S).map(|m| m.as_str().parse::<usize>().unwrap()).collect();

    // println!("{:?}", fish);

    let mut d = [0_u64; 9];
    let mut e = [0_u64; 9];

    for f in fish {
        d[f] += 1;
    }

    for g in 0..256 {
        for i in 0..9 {
            e[i] = 0;
        }
        for a in 1..9 {
            e[a - 1] = d[a];
        }
        let v = d[0];
        e[8] += v;
        e[6] += v;
        d = e;
        //println!("{}: {}", g + 1, d.iter().sum::<u64>());
    }

    println!("{}", d.iter().sum::<u64>());

    // for d in 0..40 {
    //     // generation(&mut fish);
    //     println!("Day {}: {:?}", d, fish);
    // }

    //println!("ok!");
}
