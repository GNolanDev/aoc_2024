pub mod file_reader {
    use std::fs;
    use regex::Regex;

    pub fn read_file(filename: String) -> String {
        let contents = fs::read_to_string(format!("../assets/{}", filename))
            .expect("Something went wrong reading the file");

        contents
    }

    pub fn read_file_to_lines(filename: String) -> Vec<String> {
        let contents = read_file(filename);
        let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

        lines
    }

    /* given an input txt file of the form:
    Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400
    using regex to match the integers, parse and return as ((i32, i32), (i32, i32), (i32, i32)), giving A, B, Prize X & Y values */
    pub fn read_file_to_coords(filename: String) -> ((i32, i32), (i32, i32), (i32, i32)) {
        let lines = read_file_to_lines(filename);
        let re = Regex::new(r"(-?\d+)").unwrap();
        let mut coords: Vec<i32> = Vec::new();

        for line in lines {
            for cap in re.captures_iter(&line) {
                coords.push(cap[0].parse().unwrap());
            }
        }

        ((coords[0], coords[1]), (coords[2], coords[3]), (coords[4], coords[5]))
    }

    /* given an input txt file of the form:
    Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

    which repeats a number of times, seprated by a new line

    using regex to match the integers, parse and return as a vector of ((i64, i64), (i64, i64), (i64, i64)), giving A, B, Prize X & Y values */
    pub fn read_file_to_coords_vec(filename: String) -> Vec<((i64, i64), (i64, i64), (i64, i64))> {
        let lines = read_file_to_lines(filename);
        let re = Regex::new(r"(-?\d+)").unwrap();
        let mut coords: Vec<i64> = Vec::new();
        let mut coords_vec: Vec<((i64, i64), (i64, i64), (i64, i64))> = Vec::new();

        for line in lines {
            for cap in re.captures_iter(&line) {
                coords.push(cap[0].parse().unwrap());
            }

            if coords.len() == 6 {
                coords_vec.push(((coords[0], coords[1]), (coords[2], coords[3]), (coords[4], coords[5])));
                coords.clear();
            }
        }

        coords_vec
    }


}
