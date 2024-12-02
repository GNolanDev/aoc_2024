use aoc_01::file_reader::read_file_to_lines;

fn main() {
    let contents = read_file_to_lines("input_01.txt".to_string());
    let mut arr1: Vec<i32> = Vec::new();
    let mut arr2: Vec<i32> = Vec::new();

    // for each line of contents, split and add to arr1 & arr2
    for line in contents {
        split_pair_and_add_to_separate_arrays(&line, &mut arr1, &mut arr2);
    }

    // sort each array into numerical order
    arr1.sort();
    arr2.sort();

    // function to split string on whitespace, ensure only two elements, then ensure each element is an integer, then add the first to one array & the other to a second
    fn split_pair_and_add_to_separate_arrays(s: &str, arr1: &mut Vec<i32>, arr2: &mut Vec<i32>) {
        let split: Vec<&str> = s.split_whitespace().collect();
        assert_eq!(split.len(), 2);
        arr1.push(split[0].parse().unwrap());
        arr2.push(split[1].parse().unwrap());
    }

    // iterate over arr 1 & 2 using a reducer, adding the absolute difference of each pair to a new array
    let result: i32 = arr1.iter().zip(arr2.iter()).map(|(a, b)| (a - b).abs()).sum();
    println!("{}", result);
}
