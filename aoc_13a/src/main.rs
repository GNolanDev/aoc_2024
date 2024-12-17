use aoc_13::file_reader::read_file_to_coords_vec;
use nalgebra::Matrix2;

fn main() {
    /* Utility functions */

    // given a tuple of form ((i32, i32), (i32, i32), (i32, i32)), split into 3 tuple, A, B, and Prize
    fn split_coords(coords: ((i32, i32), (i32, i32), (i32, i32))) -> ((i32, i32), (i32, i32), (i32, i32)) {
        let ((a_x, a_y), (b_x, b_y), (prize_x, prize_y)) = coords;

        ((a_x, a_y), (b_x, b_y), (prize_x, prize_y))
    }

    // given a tuple of 3 sets of coordinates, a, b, prize, return num_a, num_a, the numbers of transforms of a and b that will add up to give prize
    fn find_a_and_b_quantities(coords: ((i32, i32), (i32, i32), (i32, i32))) -> (f64, f64) {
        let (a, b, prize) = coords;

        // num_a, num_b are the numbers of a and b that add to give prize coordinates
        // i.e. num_a * a.0 + num_b * b.0 = prize.0 and num_a * a.1 + num_b * b.1 = prize.1
        // set up a matrix solution to find num_a and num_b given a, b, and prize

        // initialise a 2x2 matrix with first row of a.0 and b.0, and second row of a.1 and b.1
        // second matrix is column matrix with num_a and num_b
        // final matrix is column of prize.0 and prize.1
        // multiply the inverse of the first matrix by the final matrix to get the solution matrix

        // first initialise a 2x2 matrix with first row of a.0 and b.0, and second row of a.1 and b.1
        let a_matrix = Matrix2::new(a.0 as f64, b.0 as f64, a.1 as f64, b.1 as f64);

        // initialise a column matrix with prize.0 and prize.1
        let prize_matrix = nalgebra::Vector2::new(prize.0 as f64, prize.1 as f64);

        // find the inverse of a_matrix
        let a_matrix_inv = a_matrix.try_inverse().unwrap();

        // multiply the inverse of a_matrix by prize_matrix to get the solution matrix
        let solution_matrix = a_matrix_inv * prize_matrix;

        // return the solution matrix as a tuple
        (solution_matrix[(0, 0)] as f64, solution_matrix[(1, 0)] as f64)        
    }

    // reducer function to take a tuple and return 3 * first calue plus second value
    fn reducer((a, b): (i32, i32)) -> i32 {
        3 * a + b
    }

    // check a tuple to see if the values are both integers
    fn is_int((x, y): (f64, f64)) -> bool {
        (x.fract() < 0.001 || x.fract() >0.999) && (y.fract() < 0.01 || y.fract() > 0.999)
    }

    // filter function to remove any entries in a vector of tuples (i32, i32) to remove any with a negative value
    fn filter_negatives(vec: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        vec.into_iter().filter(|&(x, y)| x >= 0 && y >= 0).collect()
    }

    // function to remove any tuples where either number is greater than 100
    fn filter_over_100(vec: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        vec.into_iter().filter(|&(x, y)| x <= 100 && y <= 100).collect()
    }

    // end of utility functions


    // get a vector of solutions
    let mut solutions: Vec<(f64, f64)> = Vec::new();

    for coords in read_file_to_coords_vec("input_13.txt".to_string()) {
        let coords = split_coords(coords);
        let (num_a, num_b) = find_a_and_b_quantities(coords);
        solutions.push((num_a, num_b));
    }

    // filter with is_int to get only integer solutions, round to the nearest integer
    // let solutions: Vec<(i32, i32)> = solutions.into_iter().filter(|&x| is_int(x)).map(|(x, y)| (x as i32, y as i32)).collect();
    let solutions: Vec<(i32, i32)> = solutions.into_iter().filter(|&x| is_int(x)).map(|(x, y)| (x.round() as i32, y.round() as i32)).collect();

    // filter out any negative values and any values over 100
    let solutions = filter_negatives(solutions);
    
    let solutions = filter_over_100(solutions);

    // reduce the solutions to a single value
    let answer = solutions.iter().map(|&x| reducer(x)).sum::<i32>();

    println!("The answer is: {}", answer);
}
