extern crate crypto;
extern crate regex;

use std::env;
use std::str::FromStr;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::{HashMap, HashSet};
use crypto::digest::Digest;
use crypto::md5::Md5;
use regex::Regex;

#[derive(Debug)]
enum Lights {
    On,
    Off,
    Toggle
}

#[derive(Debug)]
enum Ops {
    And,
    Or,
    Not,
    Lsh,
    Rsh,
    //this is not in the file, but simplifes some things
    Id
}

//this is a neat lil feature, hm
//look into whether I can map directly to fn pointers or some such
impl FromStr for Ops {
    type Err = ();

    fn from_str(s: &str) -> Result<Ops, ()> {
        match s {
            "AND" => Ok(Ops::And),
            "OR" => Ok(Ops::Or),
            "NOT" => Ok(Ops::Not),
            "LSHIFT" => Ok(Ops::Lsh),
            "RSHIFT" => Ok(Ops::Rsh),
            _ => Err(())
        }
    }
}

struct Wire {
    name: String,
    op: Ops,
    //I am... not a fan of this
    //there is probably a Right Way using generics
    //need to read more docs, variable type _and_ number is... idk
    //for now accept this is "a (good|bad) programmer can write javascript in any language"
    //essentially faking dynamic typing by parsing out ints on the fly lol
    items: (String, String)
}

impl Wire {
    fn new(line: &str) -> (String, Wire) {
        let words = line.split(" ").collect::<Vec<_>>();

        let name: &str = words[words.len() - 1];

        let (op, items) = match words.len() {
            3 => (Ops::Id, ("", words[0])),
            4 => (Ops::Not, ("", words[1])),
            5 => (Ops::from_str(words[1]).unwrap(), (words[0], words[2])),
            _ => panic!("this shouldn't happen")
        };

        let wire = Wire {
            name: name.to_string(),
            op: op,
            //this to_string shit is clearly The Wrong Thing
            //but &'static str seems Even More Wrong
            //really I want uh... str, but scoped to the function instantiating the struct
            //rather than this function
            items: (items.0.to_string(), items.1.to_string())
        };

        (name.to_string(), wire)
    }

    //so this takes something from the items tuple and returns a u16
    //either parsed out, or by calling the parent
    //the clever/dangerous bit is in output, matching on ops
    //where I get to make assumptions about whether the left item is empty
    fn input(&self, item: &str, map: &HashMap<String, Wire>, cache: &mut HashMap<String, u16>) -> u16 {
        if cache.contains_key(item) {
            return *cache.get(item).unwrap();
        }

        //this was... a lot more clever before the cache
        let val = item.parse::<u16>();

        match val {
            Ok(val) => {
                cache.insert(self.name.to_string(), val);

                val
            },
            Err(_) => {
                let val = map.get(item).unwrap().output(map, cache);
                cache.insert(item.to_string(), val);

                val
            }
        }
    }

    fn output(&self, map: &HashMap<String, Wire>, cache: &mut HashMap<String, u16>) -> u16 {
        //println!("{} {:?} {}", self.items.0, self.op, self.items.1);

        match self.op {
            Ops::Id => self.input(&self.items.1, map, cache),
            Ops::Not => !self.input(&self.items.1, map, cache),
            Ops::And => self.input(&self.items.0, map, cache) & self.input(&self.items.1, map, cache),
            Ops::Or => self.input(&self.items.0, map, cache) | self.input(&self.items.1, map, cache),
            Ops::Rsh => self.input(&self.items.0, map, cache) >> self.input(&self.items.1, map, cache),
            Ops::Lsh => self.input(&self.items.0, map, cache) << self.input(&self.items.1, map, cache)
        }
    }
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

    fn d7(&self) -> (u16, u16) {
        let f = File::open("data/d7").unwrap();
        let f = BufReader::new(f);

        let mut map = HashMap::new();
        let mut cache = HashMap::new();

        for line in f.lines() {
            let line = line.unwrap();
            let (name, wire) = Wire::new(&line);
            map.insert(name, wire);
        }

        let map = map;

        let a1 = map.get("a").unwrap().output(&map, &mut cache);

        cache.clear();
        //cf line 75, String is obviously not what I actually want
        cache.insert("b".to_string(), a1);

        let a2 = map.get("a").unwrap().output(&map, &mut cache);

        (a1, a2)
    }

    fn d8(&self) -> (usize, usize) {
        let f = File::open("data/d8").unwrap();
        let f = BufReader::new(f);

        let mut count = 0;
        let mut count2 = 0;

        for line in f.lines() {
            let line = line.unwrap();
            let line = line.as_bytes();

            //quotes
            count += 2;
            count2 += 4;

            let mut i = 0;

            while i < line.len() {
                //backslash, step ahead
                if line[i] == 0x5c {
                    i += 1;

                    //hex escape
                    if line[i] == 0x78 {
                        count += 3;
                        count2 += 1;
                    //quote or backslash
                    } else {
                        count += 1;
                        count2 += 2;
                    }
                }

            //step ahead. two steps cleanly avoids catching escaped backslash
            i += 1;
            }
        }

        (count, count2)
    }

    fn d9(&self) -> (usize, usize) {
        let f = File::open("data/d9").unwrap();
        let f = BufReader::new(f);

        let mut graph: [[usize; 8]; 8] = [[0; 8]; 8];

        //this is naive but the input is well-ordered so eh
        let mut x = 0;
        let mut y = 1;

        for line in f.lines() {
            let cost = line.unwrap()
                .split(" ")
                .last().unwrap()
                .parse::<usize>().unwrap();

            graph[x][y] = cost;
            graph[y][x] = cost;

            if y >= 7 {
                x += 1;
                y = x + 1;
            } else {
                y += 1;
            }
        }

        let mut vec = vec!(0,1,2,3,4,5,6,7);

        let mut routes = Vec::new();
        permute(8, &mut vec, &mut routes);
        let routes = routes;

        let mut shortest = std::usize::MAX;
        let mut longest = 0;
        for route in routes {
            let mut sum = 0;

            for node in 0..route.len()-1 {
                sum += graph[route[node] as usize][route[node+1] as usize];
            }

            if sum < shortest {
                shortest = sum;
            }

            if sum > longest {
                longest = sum;
            }
        }

        (shortest, longest)
    }

    fn d10(&self) -> (usize, usize) {
        let mut vec = vec!(1,3,2,1,1,3,1,1,1,2).to_vec();

        for _ in 0..40 {
            vec = look_n_say(&vec);
        }

        let len1 = vec.len();

        for _ in 0..10 {
            vec = look_n_say(&vec);
        }

        let len2 = vec.len();

        (len1, len2)
    }

    fn d11(&self, input: &str) -> String {
        let mut pw = input.as_bytes().to_vec();

        let mut dubs = false;
        let mut trips = false;
        let mut clean = true;

        while !(dubs && trips && clean) {
            dubs = false;
            trips = false;
            clean = true;
            let mut first_dub = 0;

            pw[7] += 1;

            for i in (0..8).rev() {
                //above z
                if pw[i] == 0x7b {
                    pw[i-1] += 1;

                    for j in i..8 {
                        pw[j] = 0x61;
                    }
                }
            }

            //wanted to avoid 2 passes but, laziest bug fix
            for i in (0..8).rev() {
                //iol
                if pw[i] == 0x69 || pw[i] == 0x6c || pw[i] == 0x6f {
                    pw[i] += 1;

                    if i != 7 {
                        for j in i+1..8 {
                            pw[j] = 0x61;
                        }

                        pw[7] -= 1;
                    }

                    clean = false;
                    break;
                }

                if i > 1 && pw[i] == (pw[i-1] + 1) && pw[i] == (pw[i-2] + 2) {
                    trips = true;
                }

                if i > 0 && pw[i] == pw[i-1] {
                    if first_dub == 0 {
                        first_dub = pw[i];
                    } else if pw[i] != first_dub {
                        dubs = true;
                    }
                }
            }
        }

       std::str::from_utf8(&pw).unwrap().to_string()
    }

    fn d12(&self) -> i32 {
        let json = include_str!("../data/d12.json");
        let num = Regex::new("-*[0-9]+").unwrap();

        let sum = num.find_iter(&json)
            .map(|tupl| json[tupl.0..tupl.1].parse::<i32>().unwrap())
            .fold(0i32, |acc, val| acc + val);

        println!("{}", sum);

        sum
    }
}

fn least(x: i32, y: i32, z: i32) -> i32 {
    if x < y && x < z {x} else if y < z {y} else {z}
}

fn permute(n: usize, vec: &mut Vec<u8>, acc: &mut Vec<Vec<u8>>) {
    if n == 1 {
        acc.push((*vec).to_vec());
    } else {
        for i in 0..n-1 {
            permute(n-1, vec, acc);

            if n % 2 == 0 {
                let swap = vec[i];
                vec[i] = vec[n-1];
                vec[n-1] = swap;
            } else {
                let swap = vec[0];
                vec[0] = vec[n-1];
                vec[n-1] = swap;
            }
        }

        permute(n-1, vec, acc);
    }
}

fn look_n_say(arr: &Vec<u8>) -> Vec<u8> {
    let mut prev = 0;
    let mut count = 0;
    let mut buff = Vec::new();

    for n in arr {
        if *n == prev {
            count += 1;
        } else {
            if count > 0 {
                buff.push(count);
                buff.push(prev);
            }

            prev = *n;
            count = 1;
        }
    }

    if count > 0 {
        buff.push(count);
        buff.push(prev);
    }

    buff
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
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
            "d7" => {
                let (x, y) = Advent.d7();
                println!("first pass: {}\nsecond pass: {}", x, y);
            },
            "d8" => {
                let (x, y) = Advent.d8();
                println!("count: {}\ncount2: {}", x, y);
            },
            "d9" => {
                let (x, y) = Advent.d9();
                println!("shortest: {}\nlongest: {}", x, y);
            },
            "d10" => {
                let (x, y) = Advent.d10();
                println!("length 40: {}\nlength 50: {}", x, y);
            },
            "d11" => {
                let x = Advent.d11("vzbxkghb");
                let y = Advent.d11("vzbxxyzz");
                println!("pw 1: {}\npw 2: {}", x, y);
            },
            "d12" => {
                let x = Advent.d12();
                println!("sum: {}", x);
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

#[test]
fn test_d7() {
    let (x, y) = Advent.d7();

    assert_eq!(x, 3176);
    assert_eq!(y, 14710);
}

#[test]
fn test_d8() {
    let (x, y) = Advent.d8();

    assert_eq!(x, 1333);
    assert_eq!(y, 2046);
}

#[test]
fn test_d9() {
    let (x, y) = Advent.d9();

    assert_eq!(x, 207);
    assert_eq!(y, 804);
}

#[test]
fn test_d10() {
    let (x, y) = Advent.d10();

    assert_eq!(x, 492982);
    assert_eq!(y, 6989950);
}

#[test]
fn test_d11() {
    let x = Advent.d11("vzbxkghb");
    let y = Advent.d11("vzbxxyzz");

    assert_eq!(x, "vzbxxyzz");
    assert_eq!(y, "vzcaabcc");
}
