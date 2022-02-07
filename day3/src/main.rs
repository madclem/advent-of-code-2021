use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};


fn to_u32(slice: &[u32]) -> u32 {
    slice.iter().fold(0, |acc, &b| acc*2 + b as u32)
}

fn main() {

    match read_lines("data.txt") {
        Ok(lines) => {
            let mut columns:Vec<Vec<u32>> = Vec::new();
            lines.for_each(|f| {
                for (i, c) in  f.unwrap().chars().enumerate() {
                    match columns.get_mut(i) {
                        None => {
                            let mut v = Vec::new();
                            v.push(c.to_digit(10).unwrap());
                            columns.push(v);
                        }
                        Some(n)=> {
                            n.push(c.to_digit(10).unwrap());
                        }
                    } 
                }
            });

            let gamma = columns.iter().map(|c| {
                if c.len() as u32 / 2 >= c.iter().sum::<u32>() {
                    0
                } else {
                    1
                } 
            }).collect::<Vec<u32>>(); 
            
            let epsilon = gamma.iter().map(|c| 1 - c).collect::<Vec<u32>>();
            println!("{:?}", to_u32(&gamma) * to_u32(&epsilon));
        },
        _ => ()
    }
}

fn read_lines<P>(filename: P) -> io::Result::<io::Lines<io::BufReader<File>>> where P: AsRef<Path>{
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}