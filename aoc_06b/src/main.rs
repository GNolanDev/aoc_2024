use aoc_06::file_reader::read_file_to_lines;

fn main() {
    /* Utility functions */

    fn set_next_direction(current_direction: &char, directions: [char; 4]) -> char {
        let mut next_direction = ' ';
        for (i, direction) in directions.iter().enumerate() {
            if current_direction == direction {
                next_direction = directions[(i + 1) % 4];
                break;
            }
        }
        next_direction
    }

    // read file into a vector of strings
    let lines = read_file_to_lines("input_06.txt".to_string());

    // get line length
    let line_length = lines[0].len();

    // max number of possible steps (rough), every square * every direction - otherwise it must be in a loop
    // cast to i32
    let max_steps = (line_length * lines.len()) as i32 * 4;

    // create an array to hold the cardinal directions
    let directions = ['N', 'E', 'S', 'W'];

    // create a struct to hold the cursor's position and direction
    #[derive(Clone)]
    struct Cursor {
        x: i32,
        y: i32,
        direction: char,
    }

    let mut cursor = Cursor {
        x: 0,
        y: 0,
        direction: directions[0],
    };

    // create a grid of '.' characters
    let mut grid = vec![vec![('.', ' '); line_length]; lines.len()];

    // iterate over the lines, and for any '#' character, update the grid with a '#' character
    // for the single '^' character, add a 'X' character to the grid & set the coordinates of the cursor to the position of the 'X' character
    for (index, line) in lines.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            match c {
                '^' => {
                    grid[index][i] = ('X', ' ');
                    cursor.x = i as i32;
                    cursor.y = index as i32;
                }
                '#' => {
                    grid[index][i] = ('#', ' ');
                }
                _ => (),
            }
        }
    }
    
    // keep a copy of the original grid to use in further iterations
    let _original_grid = grid.clone();
    let original_cursor = cursor.clone();

    // function to run the simulation on one grid
    fn run_simulation(
        grid: &mut Vec<Vec<(char, char)>>,
        cursor: &mut Cursor,
        directions: [char; 4],
        max_steps: i32,
    ) -> (Vec<Vec<(char, char)>>, bool) {
        // attempt to move the cursor one step in the direction it is facing:
        // if the next position holds a '#' character, turn the cursor to the right
        // if the cursor is still in the grid, set the character to a 'X'
        // if the cursor is out of bounds, break the loop
        let num_steps = 0;
        let mut has_exceeded_max_steps = false;
        loop {
            if num_steps > max_steps {
                has_exceeded_max_steps = true;
                break;
            }
            let mut next_x = cursor.x;
            let mut next_y = cursor.y;
            match cursor.direction {
                'N' => next_y -= 1,
                'E' => next_x += 1,
                'S' => next_y += 1,
                'W' => next_x -= 1,
                _ => (),
            }
            if next_x < 0
                || next_x >= grid[0].len() as i32
                || next_y < 0
                || next_y >= grid.len() as i32
            {
                break;
            }
            if grid[next_y as usize][next_x as usize].0 == '#' {
                cursor.direction = set_next_direction(&cursor.direction, directions);
                continue;
            }
            cursor.x = next_x;
            cursor.y = next_y;
            // if grid element already has a non-empty second char (the direction) and it is the same as the current direction, set has_exceeded_max_steps to true
            if grid[cursor.y as usize][cursor.x as usize].0 == 'X' && grid[cursor.y as usize][cursor.x as usize].1 != ' ' && grid[cursor.y as usize][cursor.x as usize].1 == cursor.direction {
                has_exceeded_max_steps = true;
                break;
            }
            grid[cursor.y as usize][cursor.x as usize] = ('X', cursor.direction);
        }
        (grid.to_vec(), has_exceeded_max_steps)
    }

    // run the simulation on the original grid & store the result as 'plain_run'
    let (plain_run, _) = run_simulation(&mut grid, &mut cursor, directions, max_steps);

    // for each place where there is an 'X', store the coordinates in a vector
    let mut x_positions = Vec::new();
    for (y, row) in plain_run.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c.0 == 'X' {
                x_positions.push((x, y));
            }
        }
    }

    // for each x_position, add an obstacle '#' to the grid, run the simulation again & if it has exceeded the max steps, store the result
    let mut successful_loop_results = Vec::new();
    for (x, y) in x_positions {
        let mut grid = _original_grid.clone();
        let mut cursor = original_cursor.clone();
        grid[y][x] = ('#', ' ');
        let (loop_run, has_exceeded_max_steps) = run_simulation(&mut grid, &mut cursor, directions, max_steps);
        if has_exceeded_max_steps {
            successful_loop_results.push(loop_run);
        }
    }

    // print the number of successful loops
    println!("Number of successful loops: {}", successful_loop_results.len());
    
}
