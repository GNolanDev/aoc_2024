use aoc_01::file_reader::read_file_to_lines;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let contents = read_file_to_lines("input_01.txt".to_string());
    let mut arr1: Vec<i32> = Vec::new();
    let mut arr2: Vec<i32> = Vec::new();

    let number_of_lines = contents.len();

    // for each line of contents, split and add to arr1 & arr2
    for line in contents {
        split_pair_and_add_to_separate_arrays(&line, &mut arr1, &mut arr2);
    }

    // create lookup hashmap to avoid unnecessary extra function calls
    let mut lookup: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    // number of successful lookups
    let mut successful_lookups: i32 = 0;

    // create running total similarity score
    let mut result: i32 = 0;

    // iterate arr1, check if it is already in the hashmap & add to result, otherwise use occurences_function to find how many times it occurs in arr2, multiply this number by the arr1 value, add to the results & store in hashmap
    for i in arr1 {
        if lookup.contains_key(&i) {
            result += lookup[&i];
            successful_lookups += 1;
        } else {
            let occurences = occurences_function(&arr2, i);
            result += occurences * i;
            lookup.insert(i, occurences);
        }
    }

    // occurences function to find how many times a value occurs in an array
    fn occurences_function(arr: &Vec<i32>, value: i32) -> i32 {
        let mut occurences: i32 = 0;
        for i in arr {
            if *i == value {
                occurences += 1;
            }
        }
        occurences
    }

    fn split_pair_and_add_to_separate_arrays(s: &str, arr1: &mut Vec<i32>, arr2: &mut Vec<i32>) {
        let split: Vec<&str> = s.split_whitespace().collect();
        assert_eq!(split.len(), 2);
        arr1.push(split[0].parse().unwrap());
        arr2.push(split[1].parse().unwrap());
    }

    println!("{}", result);
    println!("Time: {:?}", now.elapsed());
    println!("Number of successful lookups: {}", successful_lookups);
    println!("Number of items in the array: {}", number_of_lines);

    // After benchmarking & finding out this implementation is ~1ms slower than without using the hashmap,
    // I checked if there were actually any repeated values in arr1 as the example demonstrates - there are none!!!
    // Hence why the hashmap is not needed in this case & only slows things down
    // (but I like it for solving the original problem better!).
}
