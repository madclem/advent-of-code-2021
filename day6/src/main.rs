use std::fs::File;
use std::io::prelude::*;

fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

fn main() {
    let mut file = File::open("data.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    let mut lanterns: [i64; 9] = [0; 9];
    contents
        .split(",")
        .map(|n| remove_whitespace(n).parse::<u8>().unwrap())
        .for_each(|n| {
            lanterns[n as usize] += 1;
        });

    let mut lanterns_prev = lanterns.clone();

    (0..256).for_each(|_| {
        lanterns_prev
            .iter_mut()
            .enumerate()
            .rev()
            .for_each(|(i, n)| {
                let mut new_index = (i) as i64 - 1;
                if new_index < 0 {
                    new_index = 6;
                    lanterns[8] = *n;
                    lanterns[new_index as usize] += *n;
                } else {
                    lanterns[new_index as usize] = *n;
                }
            });
        lanterns_prev = lanterns.clone();
    });

    let sum = lanterns.iter().sum::<i64>();
    println!("SUM {}", sum);
}
