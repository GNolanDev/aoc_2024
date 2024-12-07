pub mod file_reader {
    use std::fs;

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

    // similar function, but read lines into a vector until reaching an empty line, then start a new vector, return a vector of vectors
    pub fn read_file_to_lines_vector_on_dblnewline(filename: String) -> Vec<Vec<String>> {
        let contents = read_file(filename);
        // get all lines
        let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
        // create a vector of vectors to hold the lines
        let mut lines_vector: Vec<Vec<String>> = vec![];
        // iterate over lines, pushing each line into an element of the lines_vector, moving on to the next index of the lines_vector when an empty line is encountered
        let mut i = 0;
        for line in lines {
            if line == "" {
                i += 1;
            } else {
                if lines_vector.len() <= i {
                    lines_vector.push(vec![]);
                }
                lines_vector[i].push(line);
            }
        }

        lines_vector
    }
}
