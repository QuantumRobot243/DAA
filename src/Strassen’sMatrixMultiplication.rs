use std::io;

// Function to add two matrices
fn add(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = a.len();
    let mut c = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            c[i][j] = a[i][j] + b[i][j];
        }
    }
    c
}

// Function to subtract two matrices
fn subtract(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = a.len();
    let mut c = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            c[i][j] = a[i][j] - b[i][j];
        }
    }
    c
}

// Strassen’s multiplication function
fn strassen(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = a.len();

    // Base case: 1x1 matrix
    if n == 1 {
        return vec![vec![a[0][0] * b[0][0]]];
    }

    let k = n / 2;

    // Divide matrices into quadrants
    let mut a11 = vec![vec![0; k]; k];
    let mut a12 = vec![vec![0; k]; k];
    let mut a21 = vec![vec![0; k]; k];
    let mut a22 = vec![vec![0; k]; k];

    let mut b11 = vec![vec![0; k]; k];
    let mut b12 = vec![vec![0; k]; k];
    let mut b21 = vec![vec![0; k]; k];
    let mut b22 = vec![vec![0; k]; k];

    for i in 0..k {
        for j in 0..k {
            a11[i][j] = a[i][j];
            a12[i][j] = a[i][j + k];
            a21[i][j] = a[i + k][j];
            a22[i][j] = a[i + k][j + k];

            b11[i][j] = b[i][j];
            b12[i][j] = b[i][j + k];
            b21[i][j] = b[i + k][j];
            b22[i][j] = b[i + k][j + k];
        }
    }

    // Compute the 7 products (Strassen’s formulas)
    let p1 = strassen(add(&a11, &a22), add(&b11, &b22));
    let p2 = strassen(add(&a21, &a22), b11.clone());
    let p3 = strassen(a11.clone(), subtract(&b12, &b22));
    let p4 = strassen(a22.clone(), subtract(&b21, &b11));
    let p5 = strassen(add(&a11, &a12), b22.clone());
    let p6 = strassen(subtract(&a21, &a11), add(&b11, &b12));
    let p7 = strassen(subtract(&a12, &a22), add(&b21, &b22));

    // Combine into result matrix
    let mut c = vec![vec![0; n]; n];
    for i in 0..k {
        for j in 0..k {
            c[i][j] = p1[i][j] + p4[i][j] - p5[i][j] + p7[i][j];        // c11
            c[i][j + k] = p3[i][j] + p5[i][j];                           // c12
            c[i + k][j] = p2[i][j] + p4[i][j];                           // c21
            c[i + k][j + k] = p1[i][j] - p2[i][j] + p3[i][j] + p6[i][j]; // c22
        }
    }

    c
}

fn main() {
    let mut input = String::new();

    // Read size of matrix (must be power of 2 for simplicity)
    println!("Enter matrix size (n x n), n must be a power of 2:");
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Read first matrix
    println!("Enter first matrix row by row (space-separated):");
    let mut a: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        a.push(row);
    }

    // Read second matrix
    println!("Enter second matrix row by row (space-separated):");
    let mut b: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        b.push(row);
    }

    // Multiply matrices
    let result = strassen(a, b);

    // Print result
    println!("Resultant matrix:");
    for row in result {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
}
