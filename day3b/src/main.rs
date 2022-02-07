use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};


fn to_u32(slice: &[u32]) -> u32 {
    slice.iter().fold(0, |acc, &b| acc*2 + b as u32)
}

fn get_bit_criteria(lines: &Vec<Vec<u32>>, index: usize, is_co2_scrubber: Option<bool>) -> Vec<u32> {
    if lines.len() == 1 {
        return lines.get(0).unwrap().to_vec();
    } else {
        let sum = lines.iter().map(|l| {
            l.get(index).unwrap()
        }).sum::<u32>();

        let n_index = match is_co2_scrubber {
            Some(_) => if sum as f32 >= lines.len() as f32 / 2.0 {
                0
            } else {
                1
            },
            _ => if sum as f32 >= lines.len() as f32 / 2. {
                1
            } else {
                0
            }
        };

        let lines_new: Vec<Vec<u32>> = lines.iter().filter(|l| l.get(index).unwrap() == &n_index).cloned().collect();

        get_bit_criteria(
            &lines_new, 
            index + 1,
            is_co2_scrubber
        )
    }
}

fn main() {

    match read_lines("data.txt") {
        Ok(lines) => {
            let mut lines_vec:Vec<Vec<u32>> = Vec::new();
            lines.for_each(|f| {
                let mut v = Vec::new();
                for c in  f.unwrap().chars() {
                    v.push(c.to_digit(10).unwrap());
                }

                lines_vec.push(v);

            });

            let v = get_bit_criteria(&lines_vec, 0, None);
            let v_inversed = get_bit_criteria(&lines_vec, 0, Some(true));
            
            let mult = to_u32(&v_inversed) * to_u32(&v);
            println!("{:?}", mult);
        },
        _ => ()
    }
}

fn read_lines<P>(filename: P) -> io::Result::<io::Lines<io::BufReader<File>>> where P: AsRef<Path>{
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}