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
}
