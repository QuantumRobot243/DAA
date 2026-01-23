use std::io;

#[derive(Debug)]
struct Item {
    id: usize,
    value: f64,
    weight: f64,
}

impl Item {
    fn ratio(&self) -> f64 {
        self.value / self.weight
    }
}

fn main() {
    let mut input = String::new();

    // 1. Get total number of items
    println!("Enter the number of items:");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let n: usize = input.trim().parse().expect("Please enter a number");
    input.clear();

    // 2. Get total capacity
    println!("Enter the knapsack capacity:");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let capacity: f64 = input.trim().parse().expect("Please enter a number");
    input.clear();

    let mut items = Vec::new();

    // 3. Get item details
    for i in 1..=n {
        println!("Enter value and weight for item {} (separated by space):", i);
        io::stdin().read_line(&mut input).expect("Failed to read");
        
        let parts: Vec<f64> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();

        items.push(Item {
            id: i,
            value: parts[0],
            weight: parts[1],
        });
        input.clear();
    }

    // 4. Sort items by ratio (Greedy Step)
    // We use total_cmp to handle floating point comparisons safely
    items.sort_by(|a, b| b.ratio().total_cmp(&a.ratio()));

    // 5. Calculate Maximum Value
    let mut current_capacity = capacity;
    let mut total_value = 0.0;

    println!("\n--- Solving ---");
    for item in items {
        if current_capacity <= 0.0 { break; }

        if item.weight <= current_capacity {
            current_capacity -= item.weight;
            total_value += item.value;
            println!("Included Item {}: 100% (Full)", item.id);
        } else {
            let fraction = current_capacity / item.weight;
            total_value += item.value * fraction;
            println!("Included Item {}: {:.2}% (Fractional)", item.id, fraction * 100.0);
            current_capacity = 0.0;
        }
    }

    println!("---------------------------");
    println!("Maximum Value: {:.2}", total_value);
}