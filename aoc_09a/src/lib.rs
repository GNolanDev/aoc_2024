pub mod file_reader {
    use std::fs;

    pub fn read_file(filename: String) -> String {
        let contents = fs::read_to_string(format!("../assets/{}", filename))
            .expect("Something went wrong reading the file");

        contents
    }
}
