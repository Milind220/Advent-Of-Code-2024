use std::fs::read_to_string;

// Algorithm:
// 1. Sort both lists
// 2. Get absolute difference of corresponding elements and sum them up
// 3. Return the total distance
fn main() {
    // Read input file
    let input = read_to_string("input.txt").expect("Failed to read input file");
    
    // Parse input into two vectors
    let (left_list, right_list) = parse_input(&input);
    
    // Calculate total distance
    let total_distance = calculate_distance(left_list, right_list);
    
    println!("Total distance: {}", total_distance);
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    // Initialize vectors to store numbers
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    
    // Process each line
    for line in input.lines() {
        // Split line by whitespace and convert to numbers
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() == 2 {
            // Parse numbers and add to respective lists
            if let (Ok(left), Ok(right)) = (numbers[0].parse(), numbers[1].parse()) {
                left_list.push(left);
                right_list.push(right);
            }
        }
    }
    (left_list, right_list)
}

fn calculate_distance(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    // Sort both lists
    left.sort_unstable();
    right.sort_unstable();
    
    // Calculate total distance by pairing corresponding elements
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}
