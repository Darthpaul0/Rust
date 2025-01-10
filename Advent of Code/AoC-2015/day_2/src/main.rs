use std::fs;

// Calculate how many wrapping paper the elves need.
fn main() {
    println!("Wrapping paper needed {} sqft", calculate_total_surface());
    println!("Ribbon needed {} ft", calculate_total_ribbon());
}

pub fn extract_presents(path: String) -> Vec<(i32, i32, i32)> {
    let content = fs::read_to_string(path).expect("Failed to read file");

    // Process each line, format it into a tuple of integers
    let presents_list: Vec<_> = content
        .lines() // Split the content into lines
        .filter(|line| !line.trim().is_empty()) // Ignore empty lines
        .map(|line| {
            let parts: Vec<i32> = line
                .split('x') // Split by 'x'
                .map(|num| num.parse::<i32>().expect("Invalid number")) // Parse each part into i32
                .collect();

            (parts[0], parts[1], parts[2]) // Convert the parsed integers into a tuple
        })
        .collect(); // Collect the results into a vector

    presents_list
}

pub fn calculate_total_surface() -> i32 {
    let presents_list = extract_presents("./src/presents.txt".to_string());

    // Calculate surface for each present
    let mut total_surface: i32 = 0;

    for present in presents_list {
        total_surface += calculate_surface(present.0, present.1, present.2);
    }
    total_surface
}

pub fn calculate_surface(l: i32, w: i32, h: i32) -> i32 {
    let min = (l * w).min(w * h).min(h * l);
    let surface = 2 * l * w + 2 * w * h + 2 * h * l + min;
    surface
}

// PART 2: Calculate how many ribbon do the elves need
// the feet of ribbon required for the perfect bow is equal to the cubic feet of volume of the present
// The ribbon required to wrap a present is the shortest distance around its sides, or the smallest perimeter of any one face

pub fn calculate_ribbon(l: i32, w: i32, h: i32) -> i32 {
    // Calculate ribbon required to wrap
    let mut min = [l, w, h];
    min.sort();
    let ribbon_to_wrap: i32 = min[0] * 2 + min[1] * 2;
    // Calculate volume for the bow
    let ribbon_to_bow: i32 = l * w * h;
    ribbon_to_wrap + ribbon_to_bow
}

pub fn calculate_total_ribbon() -> i32 {
    let presents_list = extract_presents("./src/presents.txt".to_string());

    // Calculate surface for each present
    let mut total_ribbon: i32 = 0;

    for present in presents_list {
        total_ribbon += calculate_ribbon(present.0, present.1, present.2);
    }
    total_ribbon
}
