//! Compilation & Optimization Demo
//!
//! Shows how LLVM optimizations affect performance and code generation.
//! Run with: cargo run --bin compilation-optimization

use std::time::Instant;

#[inline(never)] // Prevent inlining for demonstration
fn fibonacci_recursive(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

fn demonstrate_optimization_levels() {
    println!("⚡ Optimization Level Comparison");
    println!("===============================");

    println!("Calculating Fibonacci(35)...\n");

    // Test recursive version (optimization helps a lot here)
    let start = Instant::now();
    let recursive_result = fibonacci_recursive(35);
    let recursive_time = start.elapsed();

    // Test iterative version (already optimized)
    let start = Instant::now();
    let iterative_result = fibonacci_iterative(35);
    let iterative_time = start.elapsed();

    println!("Recursive Fibonacci(35) = {} in {:?}", recursive_result, recursive_time);
    println!("Iterative Fibonacci(35) = {} in {:?}", iterative_result, iterative_time);
    println!("Recursive is ~{}x slower", recursive_time.as_nanos() / iterative_time.as_nanos());
    println!("(With optimization, LLVM can optimize tail recursion)\n");
}

fn demonstrate_constant_folding() {
    println!("🔢 Constant Folding");
    println!("==================");

    // LLVM will pre-calculate these at compile time
    const COMPILE_TIME_COMPUTATION: i32 = 5 * 10 + 3 * 4;

    println!("Compile-time constant: {}", COMPILE_TIME_COMPUTATION);
    println!("This was calculated when you compiled, not when you run!");
    println!("Check the assembly: it just loads {}", COMPILE_TIME_COMPUTATION);
    println!();
}

fn demonstrate_dead_code_elimination() {
    println!("🗑️  Dead Code Elimination");
    println!("========================");

    let used_variable = 42;
    let _dead_variable = 999; // This will be removed by LLVM

    println!("Used variable: {}", used_variable);
    println!("Dead variable (_dead_variable) is removed by LLVM");
    println!("It won't appear in the final binary\n");
}

fn demonstrate_loop_optimization() {
    println!("🔄 Loop Optimization");
    println!("===================");

    let mut sum = 0i64;
    let start = Instant::now();

    // This loop can be optimized by LLVM
    for i in 0..1_000_000 {
        sum += i as i64;
    }

    let time = start.elapsed();
    let expected = (999_999i64 * 1_000_000) / 2; // Gauss formula

    println!("Sum of 0..1,000,000 = {}", sum);
    println!("Expected (Gauss): {}", expected);
    println!("Time taken: {:?}", time);
    println!("LLVM may optimize this to: sum = n*(n-1)/2");
    println!();
}

fn demonstrate_vectorization() {
    println!("🚀 SIMD Vectorization");
    println!("====================");

    let size = 100_000;
    let mut a = vec![1.0f64; size];
    let mut b = vec![2.0f64; size];
    let mut result = vec![0.0f64; size];

    let start = Instant::now();

    // This loop can be vectorized by LLVM (if target CPU supports SIMD)
    for i in 0..size {
        result[i] = a[i] + b[i] * 3.0;
    }

    let time = start.elapsed();

    println!("Vector addition/multiplication of {} elements", size);
    println!("Time taken: {:?}", time);
    println!("With SIMD support, this processes multiple elements per instruction");
    println!("Target CPU affects this: sandybridge+ enables AVX instructions\n");
}

fn demonstrate_function_inlining() {
    println!("📦 Function Inlining");
    println!("===================");

    #[inline(always)]
    fn small_function(x: i32) -> i32 {
        x + 1
    }

    let start = Instant::now();
    let mut result = 0i64;

    // LLVM may inline small_function call
    for i in 0..1_000_000 {
        result += small_function(i) as i64;
    }

    let time = start.elapsed();

    println!("Called small_function 1,000,000 times");
    println!("Time taken: {:?}", time);
    println!("#[inline(always)] forces LLVM to replace the call with: x + 1");
    println!("No function call overhead!\n");
}

fn main() {
    println!("⚙️  Compilation & Optimization Demo");
    println!("====================================");
    println!("How LLVM makes your Rust code faster.\n");

    demonstrate_optimization_levels();
    demonstrate_constant_folding();
    demonstrate_dead_code_elimination();
    demonstrate_loop_optimization();
    demonstrate_vectorization();
    demonstrate_function_inlining();

    println!("🎯 Key Takeaways:");
    println!("• LLVM performs extensive optimizations at compile time");
    println!("• Optimization level affects compilation speed vs runtime speed");
    println!("• Target CPU architecture enables better instructions (AVX, SIMD)");
    println!("• Many optimizations happen regardless of --release flag");
    println!("• Profile your code to see what optimizations help most");

    println!("\n💡 Try running with different optimization levels:");
    println!("   cargo run --release --bin compilation-optimization  # Optimized");
    println!("   cargo run --bin compilation-optimization           # Debug");
}