use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use itertools::{Itertools, TupleWindows};

fn main() {
    match read_lines("./data.txt") {
        Ok(lines) => {
            let lines = lines
                .map(|l| l.unwrap().parse::<i32>().expect("Could not parse line"))
                .collect::<Vec<i32>>();
            
            let len = lines.len() - 3;
            let mut last_sum_vector = 0;
            let mut sum = 0;
            for n in 0..=len  {
                let sum_vector: i32 = [lines[n] + lines[n+1] + lines[n+2]].iter().sum();
                if n > 0 && sum_vector > last_sum_vector {
                    sum = sum + 1;
                }
                last_sum_vector = sum_vector;
            }

            // let mut arr = Vec::new();
            // let sum = arr.windows(2)
            // .map(|pair| {
            //     if pair[1] > pair[0] {
            //         1
            //     } else {
            //         0
            //     }
            // })
            // .sum::<i32>();

            println!("sum - {:?}", sum);
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


/* 
    Nothing to do with that, misread the problem... But struggled to get 
    the data into tupples, so I need to keep it here for my own sake
*/

// match read_lines("./data.txt") {
//     Ok(lines) => {
//         let sum = lines
//             .map(|l| l.unwrap().parse::<i32>().expect("Could not parse line"))
//             .tuples::<(i32,i32,i32)>()
//             .collect::<Vec<_>>()
//             .windows(2)
//             .map(|pair| {
//                 if (pair[1].0 + pair[1].1 + pair[1].2) > (pair[0].0 + pair[0].1 + pair[0].2) {
//                     1
//                 } else {
//                     0
//                 }
//             })
//             .sum::<i32>();
        
//         println!("sum - {:?}", sum);
//     }
//     _ => {
//         println!("Could not open the file");
//         ()
//     },
// }