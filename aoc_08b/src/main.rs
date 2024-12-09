use aoc_08::file_reader::read_file_to_lines;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    /* Utility functions */

    // function to take a vector of tuples of ints (coordinates), compare each set of coordinatws with every other set, add an antinode ('#') to the antinode_grid the same number of places in both directions beyond each coordinate in repeating pattern, and at each coordinate
    fn add_antinodes(antinode_grid: &mut Vec<Vec<char>>, coords: &Vec<(i32, i32)>, width: i32, height: i32) {
        // first add the coordinates themselves to the antinode_grid
        for (y, x) in coords.iter() {
            if *x >= 0 && *x < width && *y >= 0 && *y < height {
                antinode_grid[*y as usize][*x as usize] = '#';
            }
        }

        // then add the antinodes in the repeating pattern
        for (i, coord) in coords.iter().enumerate() {
            for (j, other_coord) in coords.iter().enumerate() {
                if i != j {
                    let (y1, x1) = coord.clone();
                    let (y2, x2) = other_coord.clone();
                    let x_diff = x2 as i32 - x1 as i32;
                    let y_diff = y2 as i32 - y1 as i32;
                    let mut x1_antinode = x1 - x_diff;
                    let mut y1_antinode = y1 - y_diff;
                    let mut x2_antinode = x2 + x_diff;
                    let mut y2_antinode = y2 + y_diff;
                    while x1_antinode >= 0 && x1_antinode < width && y1_antinode >= 0 && y1_antinode < height {
                        antinode_grid[y1_antinode as usize][x1_antinode as usize] = '#';
                        x1_antinode -= x_diff;
                        y1_antinode -= y_diff;
                    }
                    while x2_antinode >= 0 && x2_antinode < width && y2_antinode >= 0 && y2_antinode < height {
                        antinode_grid[y2_antinode as usize][x2_antinode as usize] = '#';
                        x2_antinode += x_diff;
                        y2_antinode += y_diff;
                    }
                }
            }
        }

    }

    /* end of utility functions */

    // read file into a vector of strings
    let lines = read_file_to_lines("input_08.txt".to_string());

    // get dimensions of the grid
    let width = lines[0].len();
    let height = lines.len();

    // create a grid of '.' characters
    let mut grid = vec![vec!['.'; width]; height];

    // create a hashmap to store all antenna types (alphanumeric chars) against a vector of their coordinates
    let mut antenna_types = std::collections::HashMap::new();

    // populate the grid with the input data
    for (index, line) in lines.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            grid[index][i] = c;
            if c.is_alphanumeric() {
                let coords = antenna_types.entry(c).or_insert(Vec::new());
                coords.push((index as i32, i as i32));
            }
        }
    }

    // create a new grid of the same size as the input grid to store the generated antinodes
    let mut antinode_grid = vec![vec!['.'; width]; height];

    // add antinodes to the antinode_grid
    for coords in antenna_types.values() {
        add_antinodes(&mut antinode_grid, coords, width as i32, height as i32);
    }

    // count the number of anitnodes in the antinode_grid
    let mut antinode_count = 0;
    for row in antinode_grid.iter() {
        for c in row.iter() {
            if *c == '#' {
                antinode_count += 1;
            }
        }
    }

    // print the number of antinodes
    println!("Number of antinodes: {}", antinode_count);
    // microsecond timer
    println!("Time: {}us: " , now.elapsed().as_micros());
}
