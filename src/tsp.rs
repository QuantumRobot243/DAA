use std::io;

fn main() {
    let mut input = String::new();

    // Read number of cities
    println!("Enter number of cities:");
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Read distance matrix
    println!("Enter distance matrix row by row (space-separated):");
    let mut dist: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        dist.push(row);
    }

    // Generate all permutations of cities except starting city (0)
    let mut cities: Vec<usize> = (1..n).collect();
    let mut min_cost = i32::MAX;
    let mut best_path = Vec::new();

    permute(&mut cities, 0, &dist, &mut min_cost, &mut best_path);

    // Print result
    println!("Minimum cost: {}", min_cost);
    print!("Best path: 0 ");
    for city in &best_path {
        print!("-> {} ", city);
    }
    println!("-> 0");
}

// Function to generate all permutations
fn permute(
    cities: &mut Vec<usize>,
    start: usize,
    dist: &Vec<Vec<i32>>,
    min_cost: &mut i32,
    best_path: &mut Vec<usize>,
) {
    if start == cities.len() {
        // Calculate cost of this permutation
        let mut cost = dist[0][cities[0]]; // from start to first city
        for i in 0..cities.len() - 1 {
            cost += dist[cities[i]][cities[i + 1]];
        }
        cost += dist[cities[cities.len() - 1]][0]; // back to start

        if cost < *min_cost {
            *min_cost = cost;
            *best_path = cities.clone();
        }
        return;
    }

    for i in start..cities.len() {
        cities.swap(start, i);
        permute(cities, start + 1, dist, min_cost, best_path);
        cities.swap(start, i); // backtrack
    }
}
