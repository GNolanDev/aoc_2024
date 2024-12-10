use aoc_10::file_reader::read_file_to_vector_of_vectors_of_i32s;
use recursive::recursive;

fn main() {
    /* Utility functions */

    // recursive function 'trail_hunter': takes a vector of coordinate tuples, the 'topo_grid', and a target int
    // for each tuple, search the adjacent coordinates in the topo_grid for the target int, push all the coords into a new vector, increment the target & call the function again
    // when the target is 9, return the number of targets found instead
    #[recursive]
    fn trail_hunter(coords: Vec<(usize, usize)>, topo_grid: Vec<Vec<i32>>, target: i32) -> i32 {
        
        let mut new_coords: Vec<(usize, usize)> = Vec::new();

        for coord in coords {
            let (x, y) = coord;
            let mut adj_coords: Vec<(usize, usize)> = Vec::new();

            if x > 0 {
                adj_coords.push((x - 1, y));
            }
            if x < topo_grid.len() - 1 {
                adj_coords.push((x + 1, y));
            }
            if y > 0 {
                adj_coords.push((x, y - 1));
            }
            if y < topo_grid[0].len() - 1 {
                adj_coords.push((x, y + 1));
            }

            for adj_coord in adj_coords {
                let (adj_x, adj_y) = adj_coord;
                if topo_grid[adj_x][adj_y] == target {
                    new_coords.push(adj_coord);
                }
            }
        }

        if target == 9 {
            return new_coords.len() as i32;
        }

        trail_hunter(new_coords.into_iter().collect(), topo_grid, target + 1)
    }

    // 'zero_hunter' function to find the coords of all zeroes in the grid, return them as a vector of tuples
    fn zero_hunter(topo_grid: Vec<Vec<i32>>) -> Vec<(usize, usize)> {
        let mut zero_coords: Vec<(usize, usize)> = Vec::new();

        for (x, row) in topo_grid.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if *cell == 0 {
                    zero_coords.push((x, y));
                }
            }
        }

        zero_coords
    }

    // end of utility functions

    // fetch input data into vector of vectors of ints (a 'topo_grid')
    let topo_grid = read_file_to_vector_of_vectors_of_i32s("input_10.txt".to_string());

    // call the zero_hunter function on the topo_grid, pass the result to the trail_hunter function and keep a running total of the sum of the returns
    let zero_coords = zero_hunter(topo_grid.clone());

    let mut total = 0;

    for zero_coord in zero_coords {
        total += trail_hunter(vec![zero_coord], topo_grid.clone(), 1);
    }

    println!("Total: {}", total);

}
