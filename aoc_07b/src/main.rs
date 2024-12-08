use aoc_07::file_reader::read_file_to_lines;
use std::cmp::Ordering;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    /* Utility functions */

    // quick function with 3 return states - takes a tuple of target and vector of integers, returns -1,0,+1 to show if sum of ints is less than, eqaul to or greater than target
    fn function_compare(data: (i64, Vec<i64>), operator: char) -> i32 {
        let target = data.0;
        let ints = data.1;
        let mut function_output = 0;
        if operator == '+' {
            function_output = ints.iter().sum();
        } else if operator == '*' {
            function_output = ints.iter().product();
        } else {
            panic!("Invalid operator");
        }
        match function_output.cmp(&target) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    // function to find every variation of addition and multiplication of integers from left to right, check for equality with a target and return true/false
    fn target_can_be_matched(data: (i64, Vec<i64>)) -> bool {
        // quick escape clause if case fails 2 simple tests
        if function_compare(data.clone(), '+') == -1 || function_compare(data.clone(), '*') == -1 {
            println!("No need to check: {:?}", data);
            return false;
        }

        let mut running_totals = vec![];
        running_totals.push(data.1[0] as i64);
        for i in 1..data.1.len() {
            let mut new_running_totals = vec![];
            for total in running_totals {
                new_running_totals.push(total + data.1[i] as i64);
                new_running_totals.push(total * data.1[i] as i64);
            }
            running_totals = new_running_totals;
        }
        running_totals.contains(&data.0)
    }

    /* end of utility functions */



    // read file into a vector of strings
    let lines = read_file_to_lines("input_07.txt".to_string());
    
    // input lines are in format: "123: 4 5 6" where 123 is the target and any number of integers follow the colon
    // we need to parse the target and the integers into a tuple
    let data: Vec<(i64, Vec<i64>)> = lines
        .iter()
        .map(|line| {
            let mut parts = line.split(": ");
            let target = parts.next().unwrap().parse::<i64>().unwrap();
            let ints = parts
                .next()
                .unwrap()
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            (target, ints)
        })
        .collect();

    // iterate over the data, checking each tuple using target_can_be_matched, if true add the target to a running total
    let mut running_total = 0;
    for d in data {
        if target_can_be_matched(d.clone()) {
            running_total += d.0;
        }
    }

    // print the running total
    println!("Total: {}", running_total);
    println!("Time: {}ms", now.elapsed().as_millis());
}
