//! Hardware Fundamentals Demo
//!
//! This demo explores CPU registers, cache systems, and hardware threads.
//! Run with: cargo run --bin hardware-fundamentals

use std::time::Instant;

fn demonstrate_registers() {
    println!("🖥️  CPU Registers & Memory Access");
    println!("=================================");

    // Demonstrate register usage vs memory access
    let start = Instant::now();

    // This loop uses registers heavily
    let mut register_var = 0u64;
    for i in 0..1_000_000 {
        register_var += i;
    }

    let register_time = start.elapsed();
    println!("Register-heavy loop: {:?}", register_time);

    // This loop accesses memory
    let start = Instant::now();
    let mut memory_array = [0u64; 1_000_000];
    for i in 0..1_000_000 {
        memory_array[i % 1000] += i as u64;
    }

    let memory_time = start.elapsed();
    println!("Memory access loop: {:?}", memory_time);
    println!("Memory is ~{}x slower than registers\n", memory_time.as_nanos() / register_time.as_nanos());
}

fn demonstrate_cache_lines() {
    println!("📏 Cache Line Size Demonstration");
    println!("===============================");

    const ARRAY_SIZE: usize = 64 * 1024 * 1024; // 64MB
    let mut array: Vec<u64> = vec![0; ARRAY_SIZE];

    // Sequential access (good for cache)
    let start = Instant::now();
    for i in (0..ARRAY_SIZE).step_by(8) {  // Every 8th element (cache line friendly)
        array[i] += 1;
    }
    let sequential_time = start.elapsed();

    // Random access (bad for cache)
    let start = Instant::now();
    for i in 0..ARRAY_SIZE / 8 {
        let random_index = (i * 997) % ARRAY_SIZE;  // Pseudo-random access
        array[random_index] += 1;
    }
    let random_time = start.elapsed();

    println!("Sequential access: {:?}", sequential_time);
    println!("Random access: {:?}", random_time);
    println!("Random access is ~{}x slower\n", random_time.as_nanos() / sequential_time.as_nanos());
}

fn demonstrate_cpu_threads() {
    println!("🧵 Hardware Threads vs Cores");
    println!("===========================");

    println!("Physical CPU cores: {}", num_cpus::get_physical());
    println!("Logical CPU cores: {}", num_cpus::get());

    if num_cpus::get() > num_cpus::get_physical() {
        println!("✓ Hyperthreading/SMT detected!");
        println!("  Logical cores = {} × physical cores",
                 num_cpus::get() / num_cpus::get_physical());
    } else {
        println!("✗ No hyperthreading detected");
    }

    println!("\nTesting parallel computation...");

    use std::thread;
    let start = Instant::now();

    let handles: Vec<_> = (0..num_cpus::get()).map(|_| {
        thread::spawn(|| {
            let mut sum = 0u64;
            for i in 0..100_000 {
                sum += i;
            }
            sum
        })
    }).collect();

    let mut total = 0u64;
    for handle in handles {
        total += handle.join().unwrap();
    }

    let parallel_time = start.elapsed();
    println!("Parallel computation with {} threads: {:?}", num_cpus::get(), parallel_time);
}

fn main() {
    println!("🖥️  Hardware Fundamentals Demo");
    println!("================================");
    println!("This demo shows how hardware affects your code performance.\n");

    demonstrate_registers();
    demonstrate_cache_lines();
    demonstrate_cpu_threads();

    println!("🎯 Key Takeaways:");
    println!("• Registers are ~100x faster than memory");
    println!("• Sequential memory access is ~10x faster than random");
    println!("• Hardware threads help with parallel workloads");
    println!("• Cache line size (64 bytes) affects data structure performance");
}