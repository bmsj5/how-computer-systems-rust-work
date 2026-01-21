// Demonstration of optimization levels and their impact

use std::time::Instant;

// Function that benefits from optimization
fn compute_sum(n: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..n {
        sum = sum.wrapping_add(i.wrapping_mul(i));
    }
    sum
}

// Function with loop that can be unrolled/vectorized
fn vector_add(a: &[f64], b: &[f64], result: &mut [f64]) {
    for i in 0..a.len().min(b.len()).min(result.len()) {
        result[i] = a[i] + b[i];
    }
}

fn main() {
    println!("=== Optimization Levels Demo ===\n");
    
    // Test computation
    let n = 10_000_000u64;
    let start = Instant::now();
    let result = compute_sum(n);
    let duration = start.elapsed();
    
    println!("Computation: sum of squares from 0 to {}", n);
    println!("Result: {}", result);
    println!("Time taken: {:?}\n", duration);
    
    // Test vector addition
    let size = 1_000_000;
    let a: Vec<f64> = (0..size).map(|i| i as f64).collect();
    let b: Vec<f64> = (0..size).map(|i| (i * 2) as f64).collect();
    let mut result_vec = vec![0.0; size];
    
    let start = Instant::now();
    vector_add(&a, &b, &mut result_vec);
    let duration = start.elapsed();
    
    println!("Vector addition: {} elements", size);
    println!("Time taken: {:?}\n", duration);
    
    println!("=== Optimization Levels Explained ===");
    println!("opt-level=0: No optimization (debug builds)");
    println!("opt-level=1: Basic optimizations");
    println!("opt-level=2: Default for --release (good balance)");
    println!("opt-level=3: Aggressive optimizations (may increase compile time)");
    println!("opt-level=s: Optimize for size");
    println!("opt-level=z: Optimize for size aggressively");
    
    println!("\n=== CPU-Specific Optimizations ===");
    println!("target-cpu=native: Optimize for your current CPU");
    println!("target-cpu=skylake: Optimize for Intel Skylake");
    println!("target-cpu=haswell: Optimize for Intel Haswell");
    println!("target-cpu=x86-64-v3: Optimize for AVX2-capable CPUs");
    
    println!("\n=== Why Not Max by Default? ===");
    println!("1. Compile time: Higher levels take longer to compile");
    println!("2. Binary size: Aggressive optimization can increase size");
    println!("3. Debugging: Harder to debug optimized code");
    println!("4. Diminishing returns: Level 2 is often good enough");
}
