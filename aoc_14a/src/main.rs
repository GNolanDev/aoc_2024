use aoc_14::file_reader::read_file_to_coords_vec;

fn main() {
    /* Utility functions */

    // given a tuple of two tuples(position and velocity), a number of iterations and a width & height of grid, return the final position
    // when position would overflow, wrap to the start/end as appropriate
    fn final_position(
        start: ((i32, i32), (i32, i32)),
        iterations: i32,
        width: i32,
        height: i32,
    ) -> (i32, i32) {
        let (pos, vel) = start;
        let (mut x, mut y) = pos;
        let (dx, dy) = vel;

        for _ in 0..iterations {
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
        }

        (x, y)
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
    let iterations = 100;

    // get the final position of each point after iterations
    let final_positions: Vec<(i32, i32)> = coords
        .iter()
        .map(|c| final_position(*c, iterations, width, height))
        .collect();

    // count how many points are in each quadrant
    let (nw, ne, se, sw) = count_quadrants(final_positions, width, height);

    // calculate the safety factor
    let safety = safety_factor((nw, ne, se, sw));

    println!("The safety factor is: {}", safety);
}
