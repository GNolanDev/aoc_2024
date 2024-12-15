use std::collections::HashSet;

use aoc_12::file_reader::read_file_to_vector_of_vectors_of_chars;
use recursive::recursive;

fn main() {
    /* Utility functions */

    //
    #[recursive]
    fn plot_finder(coords: (usize, usize), plants_grid: &Vec<Vec<char>>, assigned_coords_map: &mut HashSet<(usize, usize)>, plot_index: i32, plots: &mut Vec<(i32, i32)>) -> bool {
        // first check against hashmap of these coordinates if it has already been assigned to a plot
        // if so, return early
        if assigned_coords_map.contains(&coords) {
            return false;
        }

        // add this coordinate to the hashset
        assigned_coords_map.insert(coords);

        // update the plots at plot_index by adding one to the area, or create and set area to 1 if it doesn't exist
        if let Some((_sides, area)) = plots.get_mut(plot_index as usize) {
            *area += 1;
        } else {
            plots.push((0, 1));
        }

        // find the sides to add and the next coordinates to check
        let mut new_coords: Vec<(usize, usize)> = Vec::new();

        let mut adj_coords: Vec<(usize, usize)> = Vec::new();

        let this_char = plants_grid[coords.0][coords.1];
        
        if coords.0 > 0 {
            adj_coords.push((coords.0 - 1, coords.1));
        }
        if coords.0 < plants_grid.len() - 1 {
            adj_coords.push((coords.0 + 1, coords.1));
        }
        if coords.1 > 0 {
            adj_coords.push((coords.0, coords.1 - 1));
        }
        if coords.1 < plants_grid[0].len() - 1 {
            adj_coords.push((coords.0, coords.1 + 1));
        }

        for adj_coord in adj_coords {
            if plants_grid[adj_coord.0][adj_coord.1] == this_char {
                new_coords.push(adj_coord);
                plot_finder(adj_coord, plants_grid, assigned_coords_map, plot_index, plots);
            }
        }

        // sides to add = 4 - number of new_coords
        let sides = 4 - new_coords.len();

        // update the sides at plot_index by adding the sides
        if let Some((s, _area)) = plots.get_mut(plot_index as usize) {
            *s += sides as i32;
        }
                
        true
    }

    // reducer function to return the product of tuple ints and add to accumulator
    fn plot_cost_reducer(acc: i32, tuple: &(i32, i32)) -> i32 {
        acc + tuple.0 * tuple.1
    }

    // end of utility functions

    // fetch input data into vector of vectors of chars (a 'plants_grid')
    let plants_grid = read_file_to_vector_of_vectors_of_chars("input_12.txt".to_string());

    // create a hashset to store assigned coordinates
    let mut assigned_coords_map: HashSet<(usize, usize)> = HashSet::new();

    // create a vector to store plots
    let mut plots: Vec<(i32, i32)> = Vec::new();

    // iterate over the plants_grid
    let mut plot_index = 0;
    for i in 0..plants_grid.len() {
        for j in 0..plants_grid[0].len() {
            // call the plot_finder function on each coordinate
            if plot_finder((i, j), &plants_grid, &mut assigned_coords_map, plot_index, &mut plots) {
                plot_index += 1;
            }
        }
    }

    // find the sum of the product of sides and area for each plot
    let sum: i32 = plots.iter().fold(0, plot_cost_reducer);

    println!("The sum of the product of sides and area for each plot is: {}", sum);
}
