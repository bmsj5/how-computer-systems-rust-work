//! Operating System Concepts Demo
//!
//! Demonstrates OS-level concepts: processes, threads, scheduling, I/O.
//! Run with: cargo run --bin operating-system-concepts

use std::thread;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

fn demonstrate_processes_vs_threads() {
    println!("🔄 Processes vs Threads");
    println!("=======================");

    println!("Process: Independent memory space, heavier to create");
    println!("Thread: Shared memory space, lighter to create\n");

    let start = Instant::now();

    // Spawn multiple threads (lightweight)
    let mut handles = vec![];

    for i in 0..4 {
        let handle = thread::spawn(move || {
            let mut sum = 0u64;
            for j in 0..1_000_000 {
                sum += (i * j) as u64;
            }
            println!("Thread {} completed with sum: {}", i, sum);
            sum
        });
        handles.push(handle);
    }

    let mut total = 0u64;
    for handle in handles {
        total += handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Total threads time: {:?}", duration);
    println!("Threads share memory efficiently!\n");
}

fn demonstrate_thread_scheduling() {
    println!("📅 Thread Scheduling");
    println!("===================");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    println!("Starting 3 threads competing for a shared counter...");

    for i in 0..3 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
                println!("Thread {} incremented to: {}", i, *num);
                drop(num); // Release lock
                thread::sleep(Duration::from_millis(10)); // Simulate work
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = *counter.lock().unwrap();
    println!("Final counter value: {}", final_count);
    println!("OS scheduler managed thread execution and synchronization\n");
}

fn demonstrate_io_operations() {
    println!("💾 I/O Operations");
    println!("================");

    use std::fs;
    use std::io::Write;

    let filename = "demo_file.txt";

    // Synchronous file I/O
    let start = Instant::now();

    // Write to file
    let mut file = fs::File::create(filename).expect("Failed to create file");
    for i in 0..1000 {
        writeln!(file, "Line {}", i).expect("Failed to write");
    }
    file.flush().expect("Failed to flush");

    // Read from file
    let content = fs::read_to_string(filename).expect("Failed to read");
    let lines = content.lines().count();

    let duration = start.elapsed();

    // Cleanup
    fs::remove_file(filename).expect("Failed to remove file");

    println!("Wrote and read {} lines in {:?}", lines, duration);
    println!("I/O operations are expensive - avoid them in performance-critical code\n");
}

fn demonstrate_memory_mapping() {
    println!("🗺️  Memory-Mapped Files");
    println!("======================");

    use std::fs::OpenOptions;
    use std::io::{Seek, SeekFrom, Write};

    let filename = "memory_mapped_demo.txt";

    // Create a file with some data
    {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
            .expect("Failed to create file");

        file.write_all(b"Hello, Memory-Mapped World!").expect("Failed to write");
        file.flush().expect("Failed to flush");
    }

    // Memory map the file (concept demonstration)
    println!("File '{}' created with content", filename);
    println!("In a real OS, this file could be memory-mapped for efficient access");

    // Read traditionally
    let content = std::fs::read_to_string(filename).expect("Failed to read");
    println!("Read content: '{}'", content.trim());

    // Cleanup
    std::fs::remove_file(filename).expect("Failed to remove file");

    println!("Memory mapping allows files to appear in process address space\n");
}

fn demonstrate_process_isolation() {
    println!("🔒 Process Isolation");
    println!("===================");

    println!("Each process has its own:");
    println!("• Virtual address space");
    println!("• File descriptors");
    println!("• Environment variables");
    println!("• Current working directory");

    println!("
Process ID: {}", std::process::id());
    println!("Parent PID: {:?}", std::os::unix::process::parent_id());

    // Environment variables
    for (key, value) in std::env::vars() {
        if key.contains("PATH") || key.contains("HOME") || key.contains("USER") {
            println!("{} = {}", key, value);
        }
    }

    println!("Processes are isolated for security and stability\n");
}

fn main() {
    println!("💻 Operating System Concepts Demo");
    println!("===================================");
    println!("How the OS manages processes, threads, and resources.\n");

    demonstrate_processes_vs_threads();
    demonstrate_thread_scheduling();
    demonstrate_io_operations();
    demonstrate_memory_mapping();
    demonstrate_process_isolation();

    println!("🎯 Key Takeaways:");
    println!("• Processes: Heavyweight, isolated memory spaces");
    println!("• Threads: Lightweight, shared memory within a process");
    println!("• OS scheduler: Manages thread execution and CPU time");
    println!("• Synchronization: Prevents race conditions with locks");
    println!("• I/O operations: Expensive, should be minimized in hot paths");
    println!("• Memory mapping: Efficient file access through virtual memory");
    println!("• Process isolation: Security through memory protection");
}