use aoc_06::file_reader::read_file_to_lines;

fn main() {
    /* Utility functions */

    fn set_next_direction(current_direction: &String, directions: [&str; 4]) -> String {
        let mut next_direction = "".to_string();
        for (i, direction) in directions.iter().enumerate() {
            if current_direction == direction {
                next_direction = directions[(i + 1) % 4].to_string();
                break;
            }
        }
        next_direction
    }

    // read file into a vector of strings
    let lines = read_file_to_lines("input_06.txt".to_string());

    // get line length
    let line_length = lines[0].len();

    // create an array to hold the cardinal directions
    let directions = ["N", "E", "S", "W"];

    // create a struct to hold the cursor's position and direction
    struct Cursor {
        x: i32,
        y: i32,
        direction: String,
    }

    let mut cursor = Cursor {
        x: 0,
        y: 0,
        direction: directions[0].to_string(),
    };

    // create a grid of '.' characters
    let mut grid = vec![vec!['.'; line_length]; lines.len()];

    // iterate over the lines, and for any '#' character, update the grid with a '#' character
    // for the single '^' character, add a 'X' character to the grid & set the coordinates of the cursor to the position of the 'X' character
    for (index, line) in lines.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            match c {
                '^' => {
                    grid[index][i] = 'X';
                    cursor.x = i as i32;
                    cursor.y = index as i32;
                }
                '#' => {
                    grid[index][i] = '#';
                }
                _ => (),
            }
        }
    }

    // attempt to move the cursor one step in the direction it is facing:
    // if the next position holds a '#' character, turn the cursor to the right
    // if the cursor is still in the grid, set the character to a 'X'
    // if the cursor is out of bounds, break the loop
    loop {
        let mut next_x = cursor.x;
        let mut next_y = cursor.y;
        match cursor.direction.as_str() {
            "N" => next_y -= 1,
            "E" => next_x += 1,
            "S" => next_y += 1,
            "W" => next_x -= 1,
            _ => (),
        }
        if next_x < 0 || next_x >= line_length as i32 || next_y < 0 || next_y >= lines.len() as i32
        {
            break;
        }
        if grid[next_y as usize][next_x as usize] == '#' {
            cursor.direction = set_next_direction(&cursor.direction, directions);
            continue;
        }
        cursor.x = next_x;
        cursor.y = next_y;
        grid[cursor.y as usize][cursor.x as usize] = 'X';
    }

    // count the total number of 'X' characters
    let mut count = 0;
    for row in grid {
        for c in row {
            if c == 'X' {
                count += 1;
            }
        }
    }

    println!("The total number of 'X' characters is: {}", count);
}
