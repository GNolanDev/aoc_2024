use aoc_09::file_reader::read_file;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    /* Utility functions */

    fn reorder_blocks(files: &mut Vec<(Vec<i64>, Vec<i64>)>, free_space: &mut Vec<i64>) {
        // for each element in files working from last to first
        for i in (0..files.len()).rev() {
            // get the length of the first part of the tuple
            let num_blocks_needed = files[i].0.len();
            // find the index of the first free space that is equal to or longer than this
            let index = free_space
                .iter()
                .position(|&x| x >= num_blocks_needed as i64);
            // if there is not a free space that is long enough, or the index isn't < i - 1, continue to the next iteration
            if index.is_none() || index.unwrap() + 1 > i {
                continue;
            }
            // add the contents of the first part of the tuple to the second part of the tuple in files[index of the free space we just found out]
            let mut block_being_moved = files[i].0.clone();
            files[index.unwrap()].1.append(&mut block_being_moved);
            // empty the first part of the tuple
            files[i].0.clear();
            // reduce the length of the free space by the number of blocks needed
            free_space[index.unwrap()] -= num_blocks_needed as i64;
            // increase the free space at the index the blocks were moved from by the number of blocks needed
            if i > 1 {
                free_space[i - 1] += num_blocks_needed as i64;
            }
        }
    }

    /* end of utility functions */

    // read file into a string
    let input = read_file("input_09.txt".to_string());

    // each 'file' now contains the original file id the correct number of times, and a space for added file ids when building the final file system
    let mut files: Vec<(Vec<i64>, Vec<i64>)> = Vec::new();
    let mut free_space: Vec<i64> = Vec::new();

    input.chars().enumerate().for_each(|(i, c)| {
        if c.is_digit(10) {
            let num = c.to_string().parse::<i64>().unwrap();
            // alternately push digits into 'files' and 'free_space' vectors
            if i % 2 == 0 {
                let index = i / 2;
                // put 'file id' into first vector 'num' times
                files.push((vec![index as i64; num as usize], Vec::new()));
            } else {
                free_space.push(num);
            }
        }
    });

    reorder_blocks(&mut files, &mut free_space);

    // create final file system by iterating over the files, adding each integer from both parts of the tuple, then adding zeros in the amount indicated by the free space entry for the same index
    let final_file_system: Vec<i64> =
        files
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, (v1, v2))| {
                // push each i64 from both parts of the tuple individually
                for j in v1.iter() {
                    acc.push(*j);
                }
                for j in v2.iter() {
                    acc.push(*j);
                }
                if i < free_space.len() {
                    // push zeros in the amount indicated by the free space entry for the same index
                    for _ in 0..free_space[i] {
                        acc.push(0);
                    }
                }
                acc
            });

    // find the total of the products of each index with its value
    let total = final_file_system.iter().enumerate().fold(0, |acc, (i, &v)| acc + i as i64 * v);
    println!("{}", total);
    println!("Time: {}ms", now.elapsed().as_millis());
}
