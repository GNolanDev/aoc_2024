use aoc_04::file_reader::read_file_to_lines;

fn main() {
    // could maybe be parrallelized?
    use std::time::Instant;
    let now = Instant::now();

    let input_data = read_file_to_lines("input_04.txt".to_string());
    // convert each string to a vector of characters
    let input_data: Vec<Vec<char>> = input_data.iter().map(|x| x.chars().collect()).collect();

    //function to check the content of neighboutinf element in the vector for a pattern & return true or false
    fn check_neighbours(input_data: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
        let top_left = input_data[row - 1][col - 1];
        if top_left != 'M' && top_left != 'S' {
            return false;
        }
        let top_right = input_data[row - 1][col + 1];
        if top_right != 'M' && top_right != 'S' {
            return false;
        }
        let bottom_left = input_data[row + 1][col - 1];
        if bottom_left != 'M' && bottom_left != 'S' {
            return false;
        }
        let bottom_right = input_data[row + 1][col + 1];
        if bottom_right != 'M' && bottom_right != 'S' {
            return false;
        }
        // top right must be not equal to bottom left
        if top_right == bottom_left {
            return false;
        }
        // top left must be not equal to bottom right
        if top_left == bottom_right {
            return false;
        }
        return true;
    }

    // iterate all characters in all strings, if the character is an 'A' check the neighbours and increment the counter
    let mut counter = 0;
    for row in 1..input_data.len() - 1 {
        for col in 1..input_data[row].len() - 1 {
            if input_data[row][col] == 'A' {
                if check_neighbours(&input_data, row, col) {
                    counter += 1;
                }
            }
        }
    }

    println!("Counter: {}", counter);
    println!("Time: {}ms", now.elapsed().as_millis());
    // get in microseconds
    println!("Time: {}µs", now.elapsed().as_micros());
}
