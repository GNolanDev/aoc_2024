use aoc_14::file_reader::read_file_to_coords_vec;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    /* Utility functions */

    fn push_nth_element_positions(
        start: ((i32, i32), (i32, i32)),
        width: i32,
        height: i32,
        result_vector: &mut Vec<Vec<(i32, i32)>>,
    ) -> () {
        let (pos, vel) = start;
        let (mut x, mut y) = pos;
        let (dx, dy) = vel;
        // length of result vector
        let iterations = result_vector.len() as i32;

        for i in 0..iterations {
            x += dx;
            y += dy;

            if x < 0 {
                x = width + x;
            } else if x >= width {
                x = x - width;
            }

            if y < 0 {
                y = height + y;
            } else if y >= height {
                y = y - height;
            }

            // push coordinates into the result vector at index = iteration
            result_vector[i as usize].push((x, y));
        }
    }

    // given a vector of tuples of x, y coordinates and grid dimensions, return a tuple of how many elements are in (NW, NE, SE, SW) quadrants
    fn count_quadrants(coords: Vec<(i32, i32)>, width: i32, height: i32) -> (i32, i32, i32, i32) {
        let mut nw = 0;
        let mut ne = 0;
        let mut se = 0;
        let mut sw = 0;

        for (x, y) in coords {
            // do not count points in the middle coordinates of x or y, round down to integer division
            if x == (width - 1) / 2 || y == (height - 1) / 2 {
                continue;
            }

            if x < width / 2 && y < height / 2 {
                nw += 1;
            } else if x >= width / 2 && y < height / 2 {
                ne += 1;
            } else if x >= width / 2 && y >= height / 2 {
                se += 1;
            } else {
                sw += 1;
            }
        }
        (nw, ne, se, sw)
    }

    fn safety_factor(quadrants: (i32, i32, i32, i32)) -> i32 {
        let (nw, ne, se, sw) = quadrants;
        nw * ne * se * sw
    }

    // end of utility functions

    // read file into a vector of tuples, each represents a start point and a velocity
    let coords = read_file_to_coords_vec("input_14.txt".to_string());

    // set width & height & number of iterations
    let width = 101;
    let height = 103;
    let iterations = 101 * 103;

    let mut results_vector = vec![vec![]; iterations as usize];

    for c in coords.iter() {
        push_nth_element_positions(*c, width, height, &mut results_vector);
    }

    let safety_factors = results_vector
        .iter()
        .map(|v| count_quadrants(v.to_vec(), width, height))
        .map(|q| safety_factor(q))
        .collect::<Vec<i32>>();

    // find the index of the iteration with the smallest safety factor
    let min_safety = safety_factors.iter().min().unwrap();
    let min_safety_index = safety_factors.iter().position(|&x| x == *min_safety).unwrap();

    println!("The index of iteration with the smallest safety factor is: {}", min_safety_index); // don't forget to add 1 for the time elapsed

    // output an array of dots and Xs to show the position of elements in the result iteration with the lowest safety factor
    let mut grid = vec![vec!['.'; width as usize]; height as usize];
    for (x, y) in results_vector[min_safety_index].iter() {
        grid[*y as usize][*x as usize] = 'X';
    }

    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }

    println!("Elapsed time: {:.2?}", now.elapsed());
}
