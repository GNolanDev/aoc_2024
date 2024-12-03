use aoc_03::file_reader::read_file;
use regex::Regex;

fn main() {
    let contents = read_file("input_03.txt".to_string());

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

    // println!("{}", contents);
    println!("{}", result);
}
