use std::collections::HashSet;

use aoc_12::file_reader::read_file_to_vector_of_vectors_of_chars;
use recursive::recursive;

fn main() {
    /* Utility functions */

    #[recursive]
    fn plot_finder(
        coords: (usize, usize),
        plants_grid: &Vec<Vec<char>>,
        assigned_coords_map: &mut HashSet<(usize, usize)>,
        plot_index: i32,
        plots: &mut Vec<(i32, i32)>,
    ) -> bool {
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
        let mut adj_coords: Vec<(usize, usize)> = Vec::new();

        let this_char = plants_grid[coords.0][coords.1];

        let mut sides = 0;

        // does each adjacent element match?
        let (mut n, mut ne, mut e, mut se, mut s, mut sw, mut w, mut nw) =
            (false, false, false, false, false, false, false, false);

        if coords.0 > 0 {
            // N
            adj_coords.push((coords.0 - 1, coords.1));
            let value = plants_grid[coords.0 - 1][coords.1];
            if value == this_char {
                n = true;
                plot_finder(
                    (coords.0 - 1, coords.1),
                    plants_grid,
                    assigned_coords_map,
                    plot_index,
                    plots,
                );
            }
        } else {
            (n, nw, ne) = (false, false, false);
        }
        if coords.0 < plants_grid.len() - 1 {
            // S
            adj_coords.push((coords.0 + 1, coords.1));
            let value = plants_grid[coords.0 + 1][coords.1];
            if value == this_char {
                s = true;
                plot_finder(
                    (coords.0 + 1, coords.1),
                    plants_grid,
                    assigned_coords_map,
                    plot_index,
                    plots,
                );
            }
        } else {
            (s, sw, se) = (false, false, false);
        }
        if coords.1 > 0 {
            // W
            adj_coords.push((coords.0, coords.1 - 1));
            let value = plants_grid[coords.0][coords.1 - 1];
            if value == this_char {
                w = true;
                plot_finder(
                    (coords.0, coords.1 - 1),
                    plants_grid,
                    assigned_coords_map,
                    plot_index,
                    plots,
                );
            }
        } else {
            (w, sw, nw) = (false, false, false);
        }
        if coords.1 < plants_grid[0].len() - 1 {
            // E
            adj_coords.push((coords.0, coords.1 + 1));
            let value = plants_grid[coords.0][coords.1 + 1];
            if value == this_char {
                e = true;
                plot_finder(
                    (coords.0, coords.1 + 1),
                    plants_grid,
                    assigned_coords_map,
                    plot_index,
                    plots,
                );
            }
        } else {
            (e, ne, se) = (false, false, false);
        }

        // check if nw, ne, sw, se match
        if coords.0 > 0 && coords.1 > 0 {
            // NW
            let value = plants_grid[coords.0 - 1][coords.1 - 1];
            if value == this_char {
                nw = true;
            }
        }
        if coords.0 > 0 && coords.1 < plants_grid[0].len() - 1 {
            // NE
            let value = plants_grid[coords.0 - 1][coords.1 + 1];
            if value == this_char {
                ne = true;
            }
        }
        if coords.0 < plants_grid.len() - 1 && coords.1 > 0 {
            // SW
            let value = plants_grid[coords.0 + 1][coords.1 - 1];
            if value == this_char {
                sw = true;
            }
        }
        if coords.0 < plants_grid.len() - 1 && coords.1 < plants_grid[0].len() - 1 {
            // SE
            let value = plants_grid[coords.0 + 1][coords.1 + 1];
            if value == this_char {
                se = true;
            }
        }

        // conditions for finding corners (which is the same as num of sides
        // inside corners
        if w && n && !nw {
            sides += 1;
        }
        if n && e && !ne {
            sides += 1;
        }
        if e && s && !se {
            sides += 1;
        }
        if s && w && !sw {
            sides += 1;
        }
        // external corners
        if !(n || w) {
            sides += 1;
        }
        if !(n || e) {
            sides += 1;
        }
        if !(s || e) {
            sides += 1;
        }
        if !(s || w) {
            sides += 1;
        }

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
            if plot_finder(
                (i, j),
                &plants_grid,
                &mut assigned_coords_map,
                plot_index,
                &mut plots,
            ) {
                plot_index += 1;
            }
        }
    }

    // find the sum of the product of sides and area for each plot
    let sum: i32 = plots.iter().fold(0, plot_cost_reducer);

    println!(
        "The sum of the product of sides and area for each plot is: {}",
        sum
    );
}
