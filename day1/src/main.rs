use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    match read_lines("./data.txt") {
        Ok(lines) => {
            let sum = lines.fold(vec![0, -9999], |mut acc, line| {
                if let Ok(n) = line {
                    let n = n.parse().unwrap();
                    if acc[1] != -9999 && n > acc[1] {
                        acc[0] += 1;
                    }
                    acc[1] = n;
                }
                acc
            });

            println!("{}", sum[0]);
        }
        _ => {
            println!("Could not open the file");
            ()
        },
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}