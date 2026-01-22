use std::io; // Input/Output library

fn main() {
    // Mutable string variable named input
    let mut input = String::new();

    // Read array size
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();
    // _n is array size (underscore avoids unused warning)

    // Read array elements
    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let arr: Vec<i32> = input
        .split_whitespace()   // Split input by spaces
        .map(|x| x.parse::<i32>().unwrap()) // Convert to i32
        .collect();           // Store in vector

    // Read target element
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let target: i32 = input.trim().parse().unwrap();

    // Brute Force Search
    let mut found = false;

    for num in arr {
        if num == target {
            found = true;
            break;
        }
    }

    // Output
    if found {
        println!("Found");
    } else {
        println!("Not Found");
    }
}
