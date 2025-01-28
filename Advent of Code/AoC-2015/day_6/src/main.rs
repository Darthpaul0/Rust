fn main() {
    // Define the size of the grid
    let grid_size = 1000;

    // Initialize the lights grid as a vector of vectors
    let mut lights_grid: Vec<Vec<i32>> = vec![vec![0; grid_size]; grid_size];

    // Example usage of the extract_instructions function
    let instructions = extract_instructions("./src/instructions.txt".to_string());

    // Process each instruction
    for (action, start, end) in instructions {
        match action.as_str() {
            "turn on" => turn_on_lights(start, end, &mut lights_grid),
            "turn off" => turn_off_lights(start, end, &mut lights_grid),
            "toggle" => toggle_lights(start, end, &mut lights_grid),
            _ => (),
        }
    }
    // Count how many lights are on
    // let lights_on = lights_grid.iter().flatten().filter(|&&l| l == 1).count();
    // println!("Lights on: {}", lights_on);
    // Calculate the total brightness
    let total_brightness: i32 = lights_grid.iter().flatten().sum();
    println!("Total brightness: {}", total_brightness);
}

// Read instructions from a file
fn extract_instructions(path: String) -> Vec<(String, (usize, usize), (usize, usize))> {
    let content = std::fs::read_to_string(path).expect("Failed to read file");

    // Process each line, split it, and collect into a vector of tuples
    let instructions_list: Vec<(String, (usize, usize), (usize, usize))> = content
        .lines() // Split the content into lines
        .filter(|line| !line.trim().is_empty()) // Ignore empty lines
        .map(|line| {
            let mut parts = line.split_whitespace();
            let action = parts.next().unwrap();
            let instruction;
            let start;
            let end;

            if action == "toggle" {
                instruction = action.to_string();
                let start_coords: Vec<usize> = parts
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();
                parts.next(); // Skip "through"
                let end_coords: Vec<usize> = parts
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();
                start = (start_coords[0], start_coords[1]);
                end = (end_coords[0], end_coords[1]);
            } else {
                instruction = format!("{} {}", action, parts.next().unwrap());
                let start_coords: Vec<usize> = parts
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();
                parts.next(); // Skip "through"
                let end_coords: Vec<usize> = parts
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();
                start = (start_coords[0], start_coords[1]);
                end = (end_coords[0], end_coords[1]);
            }

            (instruction, start, end)
        })
        .collect(); // Collect the results into a vector

    instructions_list
}

fn turn_on_lights(start: (usize, usize), end: (usize, usize), lights_grid: &mut Vec<Vec<i32>>) {
    // Turn on lights from start to end
    for i in start.0..=end.0 {
        for j in start.1..=end.1 {
            // Turn on light
            lights_grid[i][j] += 1;
        }
    }
}

fn toggle_lights(start: (usize, usize), end: (usize, usize), lights_grid: &mut Vec<Vec<i32>>) {
    // Toggle lights from start to end
    for i in start.0..=end.0 {
        for j in start.1..=end.1 {
            // Toggle light
            lights_grid[i][j] += 2;
        }
    }
}

fn turn_off_lights(start: (usize, usize), end: (usize, usize), lights_grid: &mut Vec<Vec<i32>>) {
    // Turn off lights from start to end
    for i in start.0..=end.0 {
        for j in start.1..=end.1 {
            if lights_grid[i][j] > 0 {
                lights_grid[i][j] -= 1;
            }
        }
    }
}
