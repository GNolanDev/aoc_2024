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
    p=85,41 v=83,95
    using regex to match the integers, parse and return as ((i32, i32), (i32, i32)) for each line */
    pub fn read_file_to_coords_vec(filename: String) -> Vec<((i32, i32), (i32, i32))> {
        let lines = read_file_to_lines(filename);
        let re = Regex::new(r"(-?\d+)").unwrap();
        let mut coords: Vec<i32> = Vec::new();
        let mut coords_vec: Vec<((i32, i32), (i32, i32))> = Vec::new();

        for line in lines {
            for cap in re.captures_iter(&line) {
                coords.push(cap[0].parse().unwrap());
            }

            if coords.len() == 4 {
                coords_vec.push(((coords[0], coords[1]), (coords[2], coords[3])));
                coords.clear();
            }
        }

        coords_vec
    }
}
