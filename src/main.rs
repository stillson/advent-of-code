use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;

struct Advent;

impl Advent {
    fn d1(&self) -> (i32, usize) {
        let input = include_bytes!("../data/d1");
        let mut floor = 0;
        let mut basement = 0;

        for i in 0..input.len() {
            if input[i] == 0x28 {
                floor += 1;
            } else if input[i] == 0x29 {
                floor -= 1;
            }

            if floor == -1 && basement == 0 {
                basement = i+1;
            }
        }

        (floor, basement)
    }

    fn d2(&self) -> (i32, i32) {
        let f = File::open("data/d2").unwrap();
        let f = BufReader::new(f);

        let mut paper = 0;
        let mut ribbon = 0;

        for line in f.lines() {
            let line = line.unwrap();

            //zzz can't [l,w,h] yet sadly 
            let lwh = line.split("x")
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let (a, b, c) = (lwh[0] * lwh[1], lwh[0] * lwh[2], lwh[1] * lwh[2]);
            paper += 2 * (a + b + c) + least(a, b, c);

            let (p1, p2, p3) = (2 * (lwh[0] + lwh[1]), 2 * (lwh[0] + lwh[2]), 2 * (lwh[1] + lwh[2]));
            ribbon += least(p1, p2, p3) + lwh[0] * lwh[1] * lwh[2];
        }

        (paper, ribbon)
    }

    fn d3(&self) {
        let input = include_bytes!("../data/d3");

        for i in 0..20 {
            println!("{}", input[i]);
        }
    }
}

fn least(x: i32, y: i32, z: i32) -> i32 {
    if x < y && x < z {x} else if y < z {y} else {z}
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        //this is silly but whatevs
        match &*args[1] {
            "d1" => {
                let (x, y) = Advent.d1();
                println!("floor: {}\nbasement: {}", x, y);
            },
            "d2" => {
                let (x, y) = Advent.d2();
                println!("paper: {}\nribbon: {}", x, y);
            },
            "d3" => Advent.d3(),
            _ => println!("something happened")
        }
    }
}

#[test]
fn test_d1() {
    let (x, y) = Advent.d1();

    assert_eq!(x, 232);
    assert_eq!(y, 1783);
}

#[test]
fn test_d2() {
    let (x, y) = Advent.d2();

    assert_eq!(x, 1606483);
    assert_eq!(y, 3842356);
}
