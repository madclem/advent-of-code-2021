use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

struct Submarine {
    depth: i32,
    x: i32,
    aim: i32
}

impl Submarine {
    fn new()->Self {
        Self {
            depth: 0,
            x: 0,
            aim: 0
        }
    }

    fn command(&mut self, cmd: String) {
        let cmd_props = cmd.split(" ").collect::<Vec<&str>>();
        let cmd_key = cmd_props[0];
        let cmd_value = match cmd_props.get(1) {
            Some(n) => n.parse::<i32>().unwrap_or(0),
            _ => 0,
        };

        match cmd_key {
            "forward" => {
                self.x += cmd_value;
                self.depth += self.aim * cmd_value;
            }
            "down" => self.aim += cmd_value,
            "up" => self.aim -= cmd_value,
            _ => unreachable!()
        }
    }

    fn get_result(&self)->i32 {
        self.depth * self.x
    }
}

fn main() {
    match read_lines("data.txt") {
        Ok(lines) => {

            let mut submarine = Submarine::new();

            lines.for_each(|f| {
                submarine.command(f.unwrap());
            });

            println!("result - {}", submarine.get_result());
        },
        _ => {
            print!("Error");
            ()
        }
    }
}

fn read_lines<P>(filename: P)->io::Result::<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}