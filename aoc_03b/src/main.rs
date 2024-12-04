use aoc_03::file_reader::read_file;
use regex::Regex;
use rayon::prelude::*;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let contents = read_file("input_03.txt".to_string());

    fn execute_multiplier(contents: &str) -> i32 {
        // create regex to match "mul(i,j)" where i, j are integers
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        // capture i & j as groups for all incidences of "mut(i,j)"
        let result = re
            .captures_iter(&contents)
            .map(|cap| {
                let i: i32 = cap[1].parse().unwrap();
                let j: i32 = cap[2].parse().unwrap();
                i * j
            })
            .sum::<i32>();

        result
    }

    // split string on separator "don't()"
    fn split_on_dont(contents: &str) -> Vec<&str> {
        contents.split("don't(").collect()
    }

    // split string on the first occurence of "do()", then return only the second element
    fn split_on_do(contents: &str) -> Vec<&str> {
        contents.split("do()").collect::<Vec<&str>>()[1..].to_vec()
    }
    

    // function to take input string, apply split_on_dont, save first element to do_vector, apply split_on_do to the remaining elements saving the bit after the "do()" to the doable_vector
    fn split_on_dont_and_do(contents: &str) -> Vec<&str> {
        let chunks_between_donts = split_on_dont(contents);
        // add first element to a new vector
        let mut doable_vector = vec![chunks_between_donts[0]];

        // apply split_on_do to the remaining elements and add all the returns to the doable_vector
        for chunk in chunks_between_donts.iter().skip(1) {
            doable_vector.extend(split_on_do(chunk));
        }

        doable_vector        
    }

    // apply execute_multiplier to each element of the doable_vector
    fn execute_doable_vector(doable_vector: Vec<&str>) -> i32 {
        // doable_vector.iter().map(|s| execute_multiplier(s)).sum::<i32>()
        // parallel version - improves from ~ 120ms to ~ 30ms ( ~ 8ms in release build)
        doable_vector.par_iter().map(|s| execute_multiplier(s)).sum::<i32>()
    }

    let result = execute_doable_vector(split_on_dont_and_do(&contents));

    println!("{}", result);
    println!("Time: {:?}", now.elapsed());
}
