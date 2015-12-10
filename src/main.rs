use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;

struct Advent;

impl Advent {
    fn d1(&self) {
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

        println!("floor {}\nbasement {}", floor, basement);
    }

    fn d2(&self) {
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
        
        println!("paper: {}\nribbon: {}", paper, ribbon);
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
            "d1" => Advent.d1(),
            "d2" => Advent.d2(),
            _ => println!("something happened")
        }
    }
}
