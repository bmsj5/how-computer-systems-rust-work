//! Memory Management Demo
//!
//! Demonstrates virtual memory, stack vs heap, and memory access patterns.
//! Run with: cargo run --bin memory-management

use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::time::Instant;

fn demonstrate_stack_vs_heap() {
    println!("📚 Stack vs Heap Allocation");
    println!("===========================");

    // Stack allocation (automatic, fast)
    let stack_start = Instant::now();
    let mut stack_data = [0u64; 100_000];
    for i in 0..100_000 {
        stack_data[i] = i as u64;
    }
    let stack_time = stack_start.elapsed();

    // Heap allocation (manual, flexible)
    let heap_start = Instant::now();
    let mut heap_data = Vec::with_capacity(100_000);
    for i in 0..100_000 {
        heap_data.push(i as u64);
    }
    let heap_time = heap_start.elapsed();

    println!("Stack allocation (automatic): {:?}", stack_time);
    println!("Heap allocation (manual): {:?}", heap_time);
    println!("Stack is ~{}x faster for fixed-size data\n", heap_time.as_nanos() / stack_time.as_nanos());
}

fn demonstrate_virtual_memory() {
    println!("🗺️  Virtual Memory Addressing");
    println!("============================");

    // Allocate some memory
    let layout = Layout::new::<u64>();
    unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            panic!("Failed to allocate memory");
        }

        // Show virtual address
        println!("Allocated virtual address: {:p}", ptr);

        // Write to it
        ptr::write(ptr as *mut u64, 42);
        let value = ptr::read(ptr as *const u64);
        println!("Value at address: {}", value);

        // Free it
        dealloc(ptr, layout);
    }

    println!("Note: Virtual addresses are translated to physical RAM by the OS\n");
}

fn demonstrate_memory_access_patterns() {
    println!("🔄 Memory Access Patterns");
    println!("========================");

    const SIZE: usize = 10_000;
    let mut array = vec![0u32; SIZE];

    // Row-major access (cache-friendly)
    let start = Instant::now();
    for row in 0..100 {
        for col in 0..100 {
            array[row * 100 + col] += 1;
        }
    }
    let sequential_time = start.elapsed();

    // Column-major access (cache-unfriendly)
    let start = Instant::now();
    for col in 0..100 {
        for row in 0..100 {
            array[row * 100 + col] += 1;
        }
    }
    let random_time = start.elapsed();

    println!("Sequential access (row-major): {:?}", sequential_time);
    println!("Random access (column-major): {:?}", random_time);
    println!("Sequential is ~{}x faster due to cache locality\n", random_time.as_nanos() / sequential_time.as_nanos());
}

fn demonstrate_stack_growth() {
    println!("📈 Stack Growth and Limits");
    println!("==========================");

    fn recursive_function(depth: usize) -> usize {
        if depth >= 1000 {
            return depth;
        }
        // Each call adds to stack (return address, local vars)
        recursive_function(depth + 1)
    }

    let start = Instant::now();
    let result = recursive_function(0);
    let time = start.elapsed();

    println!("Deep recursion completed in: {:?}", time);
    println!("Reached depth: {}", result);
    println!("Each function call uses stack space for:");
    println!("  • Return address");
    println!("  • Local variables");
    println!("  • Function parameters");
    println!("Stack typically grows downward from high memory\n");
}

fn main() {
    println!("🧠 Memory Management Demo");
    println!("==========================");
    println!("Understanding how programs use memory.\n");

    demonstrate_stack_vs_heap();
    demonstrate_virtual_memory();
    demonstrate_memory_access_patterns();
    demonstrate_stack_growth();

    println!("🎯 Key Takeaways:");
    println!("• Stack: Fast, automatic, limited size, LIFO");
    println!("• Heap: Slower, manual, flexible size, complex patterns");
    println!("• Virtual memory: Every process has its own address space");
    println!("• Memory access patterns dramatically affect performance");
    println!("• Cache locality is crucial for performance");
}