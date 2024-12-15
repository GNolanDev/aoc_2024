use aoc_11::file_reader::read_file_to_vector_of_integers;
use std::{collections::HashMap, time::Instant};

fn main() {
    /* Utility functions */

    // recursive function - take an int & counter and apply the rules, then return the sum of the returns of calling the same function on each resulting element until counter reaches 0, then return 1
    fn transform(input: i64, counter: i32, memo: &mut HashMap<(i64, i32), i64>) -> i64 {
        if let Some(&result) = memo.get(&(input, counter)) {
            return result;
        }
        if counter == 0 {
            return 1;
        }
        if input == 0 {
            return transform(1, counter - 1, memo);
        }
        let number_of_digits = (input as f64).log10().floor() as i32 + 1;
        let result = if number_of_digits % 2 == 0 {
            let first_half = input / 10_i64.pow((number_of_digits / 2) as u32);
            let second_half = input % 10_i64.pow((number_of_digits / 2) as u32);
            transform(first_half, counter - 1, memo) + transform(second_half, counter - 1, memo)
        } else {
            transform(input * 2024, counter - 1, memo)
        };
        memo.insert((input, counter), result);
        return result;
    }

    // end of utility functions

    // measure execution time
    let now = Instant::now();

    // import data from file
    let initial_vector = read_file_to_vector_of_integers("input_11.txt".to_string());

    // add up the returns of applying the transform function to each element of the vector
    let mut memo = HashMap::new();
    let final_count: i64 = initial_vector.iter().map(|&x| transform(x, 75, &mut memo)).sum();
    

    println!("Final count: {}", final_count);
    println!("Execution time: {:?}", now.elapsed());
}
