use std::env;

struct Advent;

impl Advent {
    fn d1a(&self) {
        let input = include_bytes!("../data/d1a");
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
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        //this is silly but whatevs
        match &*args[1] {
            "d1a" => Advent.d1a(),
            _ => println!("something happened")
        }
    }
}
