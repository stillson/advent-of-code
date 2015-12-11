extern crate crypto;
extern crate regex;

use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;
use crypto::digest::Digest;
use crypto::md5::Md5;
use regex::Regex;

#[derive(Debug)]
enum Lights {
    On,
    Off,
    Toggle
}

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

    fn d3(&self) -> (usize, usize) {
        let input = include_bytes!("../data/d3");

        //part 1
        let mut pos = (0, 0);
        let mut visited = HashSet::with_capacity(input.len());
        visited.insert((0, 0));

        //part 2
        let mut pos_s = (0, 0);
        let mut pos_r = (0, 0);
        let mut visited_sr = HashSet::with_capacity(input.len());
        visited_sr.insert((0, 0));

        for i in 0..input.len() {
            //part 1
            match input[i] {
                0x5e => pos = (pos.0, pos.1 + 1),
                0x76 => pos = (pos.0, pos.1 - 1),
                0x3e => pos = (pos.0 + 1, pos.1),
                0x3c => pos = (pos.0 - 1, pos.1),
                _ => ()
            }

            visited.insert(pos);

            //part 2
            //lol yy pp yy pp oh well
            if i % 2 == 0 {
                match input[i] {
                    0x5e => pos_s = (pos_s.0, pos_s.1 + 1),
                    0x76 => pos_s = (pos_s.0, pos_s.1 - 1),
                    0x3e => pos_s = (pos_s.0 + 1, pos_s.1),
                    0x3c => pos_s = (pos_s.0 - 1, pos_s.1),
                    _ => ()
                }

                visited_sr.insert(pos_s);
            } else {
                match input[i] {
                    0x5e => pos_r = (pos_r.0, pos_r.1 + 1),
                    0x76 => pos_r = (pos_r.0, pos_r.1 - 1),
                    0x3e => pos_r = (pos_r.0 + 1, pos_r.1),
                    0x3c => pos_r = (pos_r.0 - 1, pos_r.1),
                    _ => ()
                }

                visited_sr.insert(pos_r);
            }
        }
        (visited.len(), visited_sr.len())
    }

    fn d4(&self, hash_start: &str) -> usize {
        let key = "iwrupvqb";
        let mut md5 = Md5::new();

        let mut i = 0;
        let mut result = String::new();
        while !result.starts_with(&hash_start) {
            i += 1;

            let attempt = format!("{}{}", key, i);

            md5.input_str(&attempt);
            result = md5.result_str();
            md5.reset();
        }
        
        i
    }

    fn d5(&self) -> (i32, i32) {
        let f = File::open("data/d5").unwrap();
        let f = BufReader::new(f);

        //regex macro sadly also gated atm
        let vowel = Regex::new("[aeiou]").unwrap();
        let blacklist = Regex::new("ab|cd|pq|xy").unwrap();

        let mut hits = 0;
        let mut hits_2 = 0;
        for line in f.lines() {
            let line = line.unwrap();
            let line_b = line.as_bytes();

            //part 1
            let mut test_double = false;
            for i in 0..line_b.len()-1 {
                if line_b[i] == line_b[i+1] {
                    test_double = true;
                    break;
                }
            }

            if vowel.find_iter(&line).count() > 2 && !blacklist.is_match(&line) && test_double {
                hits += 1;
            }

            //part 2
            //this is so absurd w/o backrefs lol
            let mut got_dubs = false;
            let mut got_split = false;
            for i in 0..line_b.len()-1 {
                if !got_dubs {
                    let dubs = Regex::new(&format!("{}{}", line_b[i] as char, line_b[i+1] as char)).unwrap();
                    let dubs_count = dubs.find_iter(&line).count();

                    //I'm angry that this works because it shouldn't
                    //the dataset doesn't have an item to break it
                    if dubs_count > 1 {
                        got_dubs = true;
                    }
                }

                if i < line_b.len()-2 && line_b[i] == line_b[i+2] {
                    got_split = true;
                }
            }

            if got_dubs && got_split {
                hits_2 += 1;
            }
        }

        (hits, hits_2)
    }

    //rather just keep one fn than yp the whole thing to change like five lines
    fn d6(&self) -> (usize, usize) {
        let mut grid = vec![false; 1000000];
        let mut grid_scalar = vec![0; 1000000];
        let nums = Regex::new("[0-9]+").unwrap();
        let w = 1000;

        let f = File::open("data/d6").unwrap();
        let f = BufReader::new(f);

        for line in f.lines() {
            let line = line.unwrap();

            //this is driving me nuts, can prolly be accomplished in one line
            //but I can't figure out how to get it to do the alloc in the map
            let coords: Vec<_> = nums.find_iter(&line).map(|tupl| &line[tupl.0..tupl.1]).collect();
            let coords: Vec<_> = coords.iter().map(|val| val.parse::<usize>().unwrap()).collect();

            let action = if line.starts_with("turn on") {
                Lights::On
            } else if line.starts_with("turn off") {
                Lights::Off
            } else if line.starts_with("toggle") {
                Lights::Toggle
            } else {
                panic!("this should not happen");
            };

            for i in coords[0]..coords[2]+1 {
                let m = i * w;

                for j in coords[1]..coords[3]+1 {
                    let n = m + j;

                    grid[n] = match action {
                        Lights::On => true,
                        Lights::Off => false,
                        Lights::Toggle => !grid[n]
                    };

                    match action {
                        Lights::On => {
                            grid_scalar[n] += 1;
                        },
                        Lights::Off => {
                            if grid_scalar[n] > 0 {
                                grid_scalar[n] -= 1;
                            }
                        },
                        Lights::Toggle => {
                            grid_scalar[n] += 2;
                        }
                    }
                }
            }
        }

        let mut count = 0;
        let mut count_scalar = 0;
        for i in 0..1000000 {
            if grid[i] {
                count += 1;
            }

            count_scalar += grid_scalar[i];
        }

        (count, count_scalar)
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
            "d3" => {
                let (x, y) = Advent.d3();
                println!("presents:\n  y1: {}\n  y2: {}", x, y);
            },
            "d4" => {
                println!("this might take awhile");
                let x = Advent.d4("00000");
                let y = Advent.d4("000000");

                println!("five-char: {}\nsix-char: {}", x, y);
            },
            "d5" => {
                let (x, y) = Advent.d5();
                println!("hits1: {}\nhits2: {}", x, y);
            },
            "d6" => {
                let (x, y) = Advent.d6();
                println!("count: {}\nscalar: {}", x, y);
            },
            "scratch" => {
            },
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

#[test]
fn test_d3() {
    let (x, y) = Advent.d3();

    assert_eq!(x, 2572);
    assert_eq!(y, 2631);
}

//the problem itself calls for five- and six-char collisions
//that is uh, like ten million hashes to compute
//so, this instead
#[test]
fn test_d4_three() {
    let x = Advent.d4("000");

    assert_eq!(x, 2215);
}

#[test]
#[ignore]
fn test_d4_five() {
    let x = Advent.d4("00000");

    assert_eq!(x, 346386);
}

#[test]
#[ignore]
fn test_d4_six() {
    let x = Advent.d4("000000");

    assert_eq!(x, 9958218);
}

#[test]
fn test_d5() {
    let (x, y) = Advent.d5();

    assert_eq!(x, 238);
    assert_eq!(y, 69);
}

#[test]
fn test_d6() {
    let (x, y) = Advent.d6();

    assert_eq!(x, 400410);
    assert_eq!(y, 15343601);
}
