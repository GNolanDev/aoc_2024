pub mod file_reader {
    use std::fs;
    use queues::*;

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

    pub fn read_file_to_queue_of_integers(filename: String) -> Queue<i64> {
        let contents = read_file(filename);
        let mut queue = Queue::new();
        for s in contents.split_whitespace() {
            let _ = queue.add(s.parse::<i64>().unwrap());
        }
        queue
    }
}
