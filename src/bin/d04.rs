use std::fs;
use regex::Regex;

static S: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

type Grid = [[(u32, bool); 5]; 5];

fn search_number(grids: &mut Vec<Grid>, n: u32) {
    for g in grids.iter_mut() {
        for i in 0..5 {
            for j in 0..5 {
                if g[i][j].0 == n {
                    g[i][j].1 = true;
                }
            }
        }
    }
}

fn has_winning_row(g: &Grid) -> bool {
    for i in 0..5 {
        let mut n_checked = 0;
        for j in 0..5 {
            if g[i][j].1 {
                n_checked += 1;
            }
        }
        if n_checked == 5 {
            return true
        }
    }
    false
}

fn has_winning_col(g: &Grid) -> bool {
    for j in 0..5 {
        let mut n_checked = 0;
        for i in 0..5 {
            if g[i][j].1 {
                n_checked += 1;
            }
        }
        if n_checked == 5 {
            return true
        }
    }
    false
}

fn is_winning_grid(g: &Grid) -> bool {
    has_winning_row(g) || has_winning_col(g)
}

fn get_grid_score(g: &Grid) -> u32 {
    let mut score = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !g[i][j].1 {
                score += g[i][j].0;
            }
        }
    }
    score
}

fn search_for_winner(grids: &Vec<Grid>) -> Option<u32> {
    for g in grids.iter() {
        if is_winning_grid(g) {
            return Some(get_grid_score(g))
        }
    }
    None
}

fn search_for_winners(grids: &Vec<Grid>) -> Vec<(u32, u32)> {
    let mut winners = Vec::new();
    for (i, g) in grids.iter().enumerate() {
        if is_winning_grid(g) {
            winners.push((i as u32, get_grid_score(g)));
        }
    }
    winners
}

fn part1(grids: &mut Vec<Grid>, numbers1: &Vec<u32>) -> u32 {
    let mut final_score = 0;

    for n in numbers1.iter() {
        search_number(grids, *n);
        if let Some(score) = search_for_winner(&grids) {
            final_score = score * n;
            break;
        }
    }
    //println!("{}", final_score);
    final_score
}

fn part2(grids: &mut Vec<Grid>, numbers1: &Vec<u32>) -> u32 {
    let mut winning_grids = Vec::new();
    let mut winning_scores = Vec::new();

    for n in numbers1.iter() {
        search_number(grids, *n);
        let winners = search_for_winners(&grids);
        for (i, score) in winners.iter() {
            if !winning_grids.contains(i) {
                winning_grids.push(*i);
                winning_scores.push(score * n);
            }
        }
    }
    println!("{:?}", winning_grids);
    println!("{:?}", winning_scores);
    winning_scores.last().copied().unwrap()
}

fn main() {
    let mut re = Regex::new(r"(.+)\n(?s)(.+)").unwrap();
    let data = fs::read_to_string("data/d04.txt").unwrap();
    let caps = re.captures(&data).unwrap();
    // let caps = re.captures(S).unwrap();
    let first_line = caps.get(1).unwrap().as_str();
    let rest = caps.get(2).unwrap().as_str();

    re = Regex::new(r"\d+").unwrap();

    let numbers1: Vec<u32> = re.find_iter(first_line).map(|x| x.as_str().parse::<u32>().unwrap()).collect();
    let numbers2: Vec<u32> = re.find_iter(rest).map(|x| x.as_str().parse::<u32>().unwrap()).collect();

    println!("{:?} numbers1", numbers1.len());
    println!("{:?} numbers2", numbers2.len());

    let mut grids: Vec<Grid> = Vec::new();

    let mut idx = 0;

    loop {
        let mut g = [[(0u32, false); 5]; 5];
        for j in 0..5 {
            for i in 0..5 {
                g[i][j] = (numbers2[idx], false);
                idx += 1;
            }
        }
        grids.push(g);
        if idx >= numbers2.len() {
            break;
        }
    }

    println!("{} grids", grids.len());

    let mut part1_grids = grids.clone();
    let mut part2_grids = grids.clone();

    // println!("{}", part1(&mut part1_grids, &numbers1));
    // println!("{}", part2(&mut part2_grids, &numbers1));

    assert_eq!(part1(&mut part1_grids, &numbers1), 51034);
    assert_eq!(part2(&mut part2_grids, &numbers1), 5434);

    println!("ok!");
}
