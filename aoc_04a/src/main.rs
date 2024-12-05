use aoc_04::file_reader::read_file_to_lines;
use std::sync::Mutex;

fn main() {
    // can optimise this by discounting strings that are too short
    
    use std::time::Instant;
    let now = Instant::now();
    // **********  Setup  **********

    let h_strings = read_file_to_lines("input_04.txt".to_string());
    let target = "XMAS";
    let reverse_target: &str = &target.chars().rev().collect::<String>();
    let _target_length = target.len();  // for use later in optimising by discounting strings that are too short
    let grid_width = h_strings[0].len();
    let grid_height = h_strings.len();

    println!("Grid width: {}, Grid height: {}", grid_width, grid_height);

    // create a data structure to hold a fixed number of strings
    let v_strings = Mutex::new(vec!["".to_string(); grid_height]);
    let nwse_strings = Mutex::new(vec!["".to_string(); grid_width + grid_height - 1]);
    let nesw_strings = Mutex::new(vec!["".to_string(); grid_width + grid_height - 1]);

    // **********  Utility functions **********

    // function takes row number, column number and adds the character to the appropriate string
    fn add_char_to_strings(
        row: usize,
        col: usize,
        character: char,
        v_strings: &mut Vec<String>,
        nwse_strings: &mut Vec<String>,
        nesw_strings: &mut Vec<String>,
        grid_width: usize,
    ) {
        v_strings[col].insert(row, character);
        nwse_strings[row + col].push(character);
        nesw_strings[((grid_width - 1) + row) - col].push(character);
    }

    fn check_string_for_target_reducer_counter(
        string: &str,
        target: &str,
        reverse_target: &str,
        counter: &mut i32,
    ) {
        // increment by the number of times the target is found
        *counter += string.matches(target).count() as i32;
        *counter += string.matches(reverse_target).count() as i32;
    }


    // **********  Main loop **********

    h_strings.clone()
    .into_iter().enumerate().for_each(|(row, line)| {
        for (col, character) in line.char_indices() {
            add_char_to_strings(
                row,
                col,
                character,
                &mut v_strings.lock().unwrap(),
                &mut nwse_strings.lock().unwrap(),
                &mut nesw_strings.lock().unwrap(),
                grid_width,
            );
        }
    });

    let nwse_strings = nwse_strings.lock().unwrap();
    let nesw_strings = nesw_strings.lock().unwrap();
    let v_strings = v_strings.lock().unwrap();

    // set counter then check each vector of strings for the target term
    let mut counter = 0;
    // use a reducer to check each string in the vector for the target term and maintain a count of the occurences
    h_strings.iter().for_each(|string| check_string_for_target_reducer_counter(string, target, reverse_target, &mut counter));
    v_strings.iter().for_each(|string| check_string_for_target_reducer_counter(string, target, reverse_target, &mut counter));
    nwse_strings.iter().for_each(|string| check_string_for_target_reducer_counter(string, target, reverse_target, &mut counter));
    nesw_strings.iter().for_each(|string| check_string_for_target_reducer_counter(string, target, reverse_target, &mut counter));

    println!("Counter: {}", counter);
    println!("Elapsed time: {:.2?}", now.elapsed());
}
