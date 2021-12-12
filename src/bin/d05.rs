use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

static S: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

//type Pair = (usize, usize);

struct Coord {
    x: usize,
    y: usize
}

type Grid = [[u32; 1000]; 1000];

fn increment_grid(g: &mut Grid, x: usize, y: usize) {
    g[y][x] += 1;
}

fn get_input() -> Vec<(Coord, Coord)> {
    let mut vec = Vec::new();
    // let mut re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let re = Regex::new(r"\d+").unwrap();
    // let caps = re.captures(S).unwrap();

    let f = File::open("data/d05.txt").unwrap();
    let f = BufReader::new(f);
    for line in f.lines() {
    // for line in S.lines() {
        let ns: Vec<usize> = re.find_iter(&line.unwrap()).map(|m| m.as_str().parse::<usize>().unwrap()).collect();
        // let ns: Vec<usize> = re.find_iter(line).map(|m| m.as_str().parse::<usize>().unwrap()).collect();
        assert_eq!(ns.len(), 4);
        // let caps = re.captures(line).unwrap();
        // let mut ns = [0; 4];
        // for i in 0..4 {
        //     ns[i] = caps.get(i + 1).unwrap().as_str().parse::<u32>().unwrap();
        // }
        vec.push(((Coord{x: ns[0], y: ns[1]}), Coord{x: ns[2], y: ns[3]}));
    }
    vec
}

fn is_straight(p1: &Coord, p2: &Coord) -> bool {
    p1.x == p2.x || p1.y == p2.y
}

fn get_range(init: usize, end: usize) -> Box<dyn Iterator<Item=usize>> {
    if init < end {
        Box::new(init..=end)
    } else {
        Box::new((end..=init).rev())
    }
}

fn mark(g: &mut Grid, p1: &Coord, p2: &Coord) {
    if p1.x == p2.x {
        for y in get_range(p1.y, p2.y) {
            increment_grid(g, p1.x, y);
        }
    } else if p1.y == p2.y {
        for x in get_range(p1.x, p2.x) {
            increment_grid(g, x, p1.y);
        }
    } else {
        let rg_x: Vec<usize> = get_range(p1.x, p2.x).collect();
        let rg_y: Vec<usize> = get_range(p1.y, p2.y).collect();
        for (i, x) in rg_x.iter().enumerate() {
            increment_grid(g, *x, rg_y[i]);
        }
    }
}

fn get_number_of_overlaps(g: &Grid) -> u32 {
    let mut n = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if g[i][j] >= 2 {
                n += 1;
            }
        }
    }
    n
}

fn draw_grid(g: &Grid) {
    for i in 0..1000 {
        let line: String = g[i].map(|e| match e { 0 => '.', _ => char::from_digit(e as u32, 10).unwrap() }).iter().collect();
        println!("{}", line);
    }
}

fn part1(vec: &Vec<(Coord, Coord)>) -> u32 {
    let mut g = [[0_u32; 1000]; 1000];

    for (p1, p2) in vec.iter() {
        if is_straight(p1, p2) {
            // println!("{:?} -> {:?}", p1, p2);
            mark(&mut g, p1, p2);
        }
    }
    get_number_of_overlaps(&g)
}

fn part2(vec: &Vec<(Coord, Coord)>) -> u32 {
    let mut g = [[0_u32; 1000]; 1000];

    for (p1, p2) in vec.iter() {
        mark(&mut g, p1, p2);
    }
    get_number_of_overlaps(&g)
}

fn main() {
    let vec = get_input();

    // draw_grid(&g);

    // println!("{}", part1(&vec));
    // println!("{}", part2(&vec));

    assert_eq!(part1(&vec), 5373);
    assert_eq!(part2(&vec), 21514);

    println!("ok!");
}
