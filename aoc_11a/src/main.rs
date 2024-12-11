use aoc_11::file_reader::read_file_to_queue_of_integers;
use queues::*;

fn main() {
    /* Utility functions */

    // function to take an int, apply the rules and return a vector of integers
    fn apply_rules(n: i64) -> Vec<i64> {
        let mut result = Vec::new();
        if n == 0 {
            result.push(1);
        } else if n.to_string().len() % 2 == 0 {
            let s = n.to_string();
            let half = s.len() / 2;
            let (left, right) = s.split_at(half);
            result.push(left.parse::<i64>().unwrap());
            result.push(right.parse::<i64>().unwrap());
        } else {
            result.push(n * 2024);
        }
        result
    }

    // function to take a queue of integers, apply the rules to each, unpack the resulting vector and add to a new queue
    fn apply_rules_to_queue(mut queue: Queue<i64>) -> Queue<i64> {
        let mut new_queue = Queue::new();
        while queue.size() > 0 {
            let n = queue.remove().unwrap();
            let results = apply_rules(n);
            for r in results {
                let _ = new_queue.add(r);
            }
        }
        new_queue
    }

    // function to apply the rules a given number of times
    fn apply_rules_n_times(mut queue: Queue<i64>, n: i32) -> Queue<i64> {
        for _ in 0..n {
            queue = apply_rules_to_queue(queue);
        }
        queue
    }

    // end of utility functions

    // import data from file
    let initial_queue = read_file_to_queue_of_integers("input_11.txt".to_string());

    // apply rules to the initial queue
    let new_queue = apply_rules_n_times(initial_queue, 25);

    println!("Size of the new queue: {}", new_queue.size());

}
