use aoc_02::file_reader::read_file_to_lines;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let contents = read_file_to_lines("input_02.txt".to_string());
    const SAFE_RANGE: std::ops::RangeInclusive<i32> = 1..=3;

    // variable to hold the number of 'safe' reports
    let mut result = 0;

    /**
     * Utility functions
     */

    fn split_string_to_ints(s: String) -> Vec<i32> {
        s.split_whitespace().map(|s| s.parse().unwrap()).collect()
    }

    fn out_of_safe_increment_range(a: i32, b: i32) -> bool {
        let difference = a - b;
        !SAFE_RANGE.contains(&difference.abs())
    }

    fn has_changed_trend(a: i32, b: i32, is_increasing: bool) -> bool {
        if is_increasing {
            a > b
        } else {
            a < b
        }
    }

    // function to iterate over a vector of integers, apply condition check between adjacent elements, then return true if all conditions are met
    fn is_safe_report(v: Vec<i32>) -> bool {
        let response = is_safe_report_inner(&v);

        if response.0 == true {
            return true;
        }

        // if failure, try again once each with each of the possible offending elements removed

        let mut new_vector_1 = v.clone();
        //remove the 1st element that may have caused the problem
        new_vector_1.remove(response.1 as usize);
        if is_safe_report_inner(&new_vector_1).0 == true {
            return true;
        }

        // if the first element removal didn't work, try the second element
        let mut new_vector_2 = v.clone();
        new_vector_2.remove(response.1 as usize + 1);
        if is_safe_report_inner(&new_vector_2).0 == true {
            return true;
        }

        // finally, it might still be the first element causing the trend direction to be incorrect
        if response.1 == 1 {
            let mut new_vector_3 = v.clone();
            new_vector_3.remove(0);
            if is_safe_report_inner(&new_vector_3).0 == true {
                return true;
            }
        }
        
        false
    }

    fn is_safe_report_inner(v: &Vec<i32>) -> (bool, i32) {
        let mut result: (bool, i32) = (true, -1);
        let is_increasing = v[0] < v[1];

        for i in 0..v.len() - 1 {
            if out_of_safe_increment_range(v[i], v[i + 1]) {
                result = (false, i as i32);
                break;
            }

            if has_changed_trend(v[i], v[i+1], is_increasing) {
                result = (false, i as i32);
                break;
            }
        }
        result
    }

    // end of utility functions

    // for each contents, apply the functions above and increment the result variable if true
    for line in contents {
        let numbers = split_string_to_ints(line);
        if is_safe_report(numbers) {
            result += 1;
        }
    }

    println!("Number of safe results: {}", result);
    println!("Time taken: {}ms", now.elapsed().as_millis());
}
