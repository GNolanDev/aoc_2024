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

    pub fn read_file_to_vector_of_vectors_of_chars(filename: String) -> Vec<Vec<char>> {
        let contents = read_file_to_lines(filename);
        let mut vec_of_vecs: Vec<Vec<char>> = Vec::new();

        for line in contents {
            let vec: Vec<char> = line.chars().collect();
            vec_of_vecs.push(vec);
        }

        vec_of_vecs
    }
}
