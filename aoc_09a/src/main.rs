use aoc_09::file_reader::read_file;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    /* Utility functions */

    // function to add values to the final_file_system array
    fn add_values(files: &mut Vec<i64>, free_space: &mut Vec<i64>, final_file_system: &mut Vec<i64>) {
        let mut file_id = 0;
        let mut j = files.len() - 1;
        let mut final_index = 0;
        let mut l = 0;
        let final_system_length = final_file_system.len();

        while file_id < files.len() && l < free_space.len() {
            while files[file_id] > 0 && final_index < final_system_length {
                final_file_system[final_index] = file_id as i64;
                files[file_id] -= 1;
                final_index += 1;
            }
            file_id += 1;
            while free_space[l] > 0 && final_index < final_system_length {
                if files[j] == 0 {
                    j -= 1;
                }
                final_file_system[final_index] = j as i64;
                files[j] -= 1;
                free_space[l] -= 1;
                final_index += 1;
            }
            l += 1;
        }
    }

    /* end of utility functions */

    // read file into a string
    let input = read_file("input_09.txt".to_string());

    // iterate over the string, casting each character to an integer and alternately putting characters into 'file_lengths' and 'free_spaces' vectors
    // create an array of integers 'final_file_system' of length equal to the sum of values in 'files' vector
    // begin filling final_file_system with values taken from the key of 'files', the number of times to add it is the value; after adding the first
    // value, then add values from the last element of the 'files' vector, add this the number of times equal to the value of the first element of the 'free_space' vector
    // reduce the value of the last element each time it is added to the 'final_file_system' array
    // reduce the value of the first element of the free_space vector also
    // once the files value reaches zero, delete the element and move on to the next to last element
    // once the free_space value reaches zero, go back to adding values from the next element in the files vector

    let mut files: Vec<i64> = Vec::new();
    let mut free_space: Vec<i64> = Vec::new();

    input.chars().enumerate().for_each(|(i, c)| {
        if c.is_digit(10) {
            let num = c.to_string().parse::<i64>().unwrap();
            // alternately push digits into 'files' and 'free_space' vectors
            if i % 2 == 0 {
                files.push(num);
            } else {
                free_space.push(num);
            }
        }
    });

    let mut final_file_system = vec![0; files.iter().sum::<i64>() as usize];

    add_values(&mut files, &mut free_space, &mut final_file_system);

    // println!("{:?}", final_file_system);

    // find the total of the products of each index with its value
    let total = final_file_system.iter().enumerate().fold(0, |acc, (i, &v)| acc + i as i64 * v);
    println!("{}", total);

    println!("Time: {}ms", now.elapsed().as_millis());
}
