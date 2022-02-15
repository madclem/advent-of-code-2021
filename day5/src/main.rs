use std::cmp;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

fn get_index(x: i32, y: i32, width: i32) -> usize {
    (x + width * y) as usize
}

fn main() {
    match read_lines("./data.txt") {
        Ok(lines_txt) => {
            let mut grid: HashMap<usize, u32> = HashMap::new();

            let mut max_x = 0;
            let mut max_y = 0;

            let lines = lines_txt
                .map(|f| {
                    let v = f
                        .unwrap()
                        .split("->")
                        .map(|part| {
                            let p = part
                                .split(",")
                                .map(remove_whitespace)
                                .map(|p| p.parse::<i32>().unwrap())
                                .collect::<Vec<i32>>();

                            max_x = cmp::max(max_x, p[0]);
                            max_y = cmp::max(max_y, p[1]);

                            p
                        })
                        .flatten()
                        .collect::<Vec<i32>>();

                    v
                })
                .collect::<Vec<Vec<i32>>>();

            lines.iter().for_each(|l| {
                let mut x = l[0];
                let mut y = l[1];

                let dx = (l[2] - l[0]).signum();
                let dy = (l[3] - l[1]).signum();

                while (x, y) != (l[2] + dx, l[3] + dy) {
                    let index = get_index(x, y, max_x);
                    *grid.entry(index).or_default() += 1;

                    x += dx;
                    y += dy;
                }

                /*
                    with slope,
                    not working for horizontal / vertical lines
                */
                // let slope = if l[2] - l[0] == 0 {
                //     0
                // } else {
                //     (l[3] - l[1]) / (l[2] - l[0])
                // };
                // let b = l[1] - (slope * l[0]);
                // let y = slope * x + b;
            });

            let len = grid
                .into_values()
                .filter(|v| v > &1)
                .collect::<Vec<_>>()
                .len();

            println!("{}", len);
        }
        _ => (),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
