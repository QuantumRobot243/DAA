use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Job {
    id: String,
    deadline: usize,
    profit: u32,
}

fn main() {
    let mut jobs = Vec::new();

    // 1. Get the number of jobs
    let num_jobs = input("Enter the number of jobs: ").parse::<usize>().unwrap_or(0);

    for i in 0..num_jobs {
        println!("\n--- Job {} ---", i + 1);
        let id = input("Enter Job ID (e.g., J1): ");
        let deadline = input("Enter Deadline (integer): ").parse::<usize>().unwrap_or(0);
        let profit = input("Enter Profit (integer): ").parse::<u32>().unwrap_or(0);

        jobs.push(Job { id, deadline, profit });
    }

    // 2. Sort jobs by profit (Descending)
    jobs.sort_by(|a, b| b.profit.cmp(&a.profit));

    // 3. Find max deadline to size the schedule
    let max_deadline = jobs.iter().map(|j| j.deadline).max().unwrap_or(0);
    
    // We use a Vector of Options to represent empty slots (None) or filled slots (Some)
    let mut schedule: Vec<Option<String>> = vec![None; max_deadline + 1];
    let mut total_profit = 0;

    // 4. Greedy Selection
    for job in jobs {
        // Look for a slot from 'deadline' down to 1
        for slot in (1..=job.deadline).rev() {
            if schedule[slot].is_none() {
                schedule[slot] = Some(job.id);
                total_profit += job.profit;
                break;
            }
        }
    }

    // 5. Display Results
    println!("\n--- Optimal Schedule ---");
    for (time, job_id) in schedule.iter().enumerate().skip(1) {
        match job_id {
            Some(id) => println!("Time Slot {}: Job {}", time, id),
            None => println!("Time Slot {}: EMPTY", time),
        }
    }
    println!("Total Profit: ${}", total_profit);
}

// Helper function to read user input easily
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure the prompt shows up before input
    let mut result = String::new();
    io::stdin().read_line(&mut result).expect("Failed to read line");
    result.trim().to_string()
}