use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

static BOARD_SIZE: usize = 5;
fn insert_or(vecs: &mut Vec<Vec<u32>>, index: usize, value: u32) {
    match vecs.get_mut(index) {
        None => {
            let mut v = Vec::new();
            v.push(value);
            vecs.push(v);
        }
        Some(vecs) => {
            vecs.push(value);
        }
    }
}
fn main() {
    let mut file = File::open("data.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    // turns the string into data [list of numbers to draw, grid1, grid2, ...]
    let data_vec = contents.split("\n\n").collect::<Vec<&str>>();

    let numbers_to_draw = data_vec[0]
        .split(",")
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let grids = data_vec[1..]
        .into_iter()
        .map(|a| {
            let mut columns: Vec<Vec<u32>> = Vec::new();
            let mut lines: Vec<Vec<u32>> = Vec::new();
            a.split_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .enumerate()
                .for_each(|(i, c)| {
                    let x = i % BOARD_SIZE;
                    let y = i / BOARD_SIZE;

                    insert_or(&mut columns, x, c);
                    insert_or(&mut lines, y, c);
                });

            columns
                .iter()
                .cloned()
                .chain(lines.iter().cloned().collect::<Vec<Vec<u32>>>())
                .collect()
        })
        .collect::<Vec<Vec<Vec<u32>>>>();

    let mut grids_copy = grids.clone(); // to edit this one in the nested loop...
    let mut winning_boards: HashSet<usize> = HashSet::new();
    let mut first_board: u32 = 0;
    let mut last_board: u32 = 0;

    numbers_to_draw.into_iter().for_each(|n| {
        for (i_grid, grid) in grids.iter().enumerate() {
            let mut has_already_won = winning_boards.contains(&i_grid);
            if has_already_won {
                continue;
            };

            for i_seq in 0..grid.len() {
                if has_already_won {
                    continue;
                };
                let seq = &grid[i_seq];
                for (i, line_n) in seq.iter().enumerate() {
                    if has_already_won || winning_boards.contains(&i_grid) {
                        has_already_won = true;
                        continue;
                    }
                    if *line_n == n {
                        grids_copy[i_grid][i_seq][i] = 0;
                        let sum = grids_copy[i_grid][i_seq].iter().sum::<u32>();

                        if sum == 0 {
                            winning_boards.insert(i_grid);

                            let index_start = if i < BOARD_SIZE { 0 } else { BOARD_SIZE };
                            let index_end = if i < BOARD_SIZE { BOARD_SIZE } else { 10 };

                            let mut tot = 0;
                            for final_i in index_start..index_end {
                                grids_copy[i_grid][final_i].iter().for_each(|f| {
                                    tot += f;
                                })
                            }

                            if first_board == 0 {
                                first_board = tot * n;
                            }

                            last_board = tot * n;
                        }
                    }
                }
            }
        }
    });

    println!("first_board - {:?}", first_board);
    println!("last_board - {:?}", last_board);
}
