//! Cache Line Demonstration
//!
//! Shows why cache lines are 64 bytes and how they affect performance.
//! Run with: cargo run --bin cache-line-demo

use std::time::Instant;

const CACHE_LINE_SIZE: usize = 64;
const ARRAY_SIZE: usize = 1024 * 1024; // 1M elements

#[repr(C, align(64))]
struct AlignedStruct {
    data: [u8; CACHE_LINE_SIZE],
}

fn demonstrate_cache_line_size() {
    println!("📏 Cache Line Size: Why 64 Bytes?");
    println!("===================================");

    // Allocate a large array
    let mut array = vec![0u8; ARRAY_SIZE];

    // Test 1: Sequential access (cache-friendly)
    let start = Instant::now();
    for i in (0..ARRAY_SIZE).step_by(CACHE_LINE_SIZE) {
        array[i] += 1;
    }
    let sequential_time = start.elapsed();

    // Test 2: Cache line boundary access (worst case)
    let start = Instant::now();
    for i in 0..ARRAY_SIZE / CACHE_LINE_SIZE {
        let index = (i * CACHE_LINE_SIZE) + (CACHE_LINE_SIZE - 1);
        if index < ARRAY_SIZE {
            array[index] += 1;
        }
    }
    let boundary_time = start.elapsed();

    println!("Sequential access (every {} bytes): {:?}", CACHE_LINE_SIZE, sequential_time);
    println!("Boundary access (end of cache lines): {:?}", boundary_time);
    println!("Boundary access is ~{}x slower", boundary_time.as_nanos() / sequential_time.as_nanos());
    println!();
}

fn demonstrate_false_sharing() {
    println!("🚫 False Sharing Demonstration");
    println!("=============================");

    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Arc;
    use std::thread;

    const NUM_THREADS: usize = 4;
    const ITERATIONS: u64 = 1_000_000;

    // Shared data with false sharing (variables close together)
    let counters_false: Arc<Vec<AtomicU64>> = Arc::new(
        (0..NUM_THREADS).map(|_| AtomicU64::new(0)).collect()
    );

    // Shared data without false sharing (pad to cache line boundaries)
    #[repr(align(64))]
    struct PaddedCounter {
        value: AtomicU64,
        _padding: [u8; 56], // Pad to 64 bytes total
    }

    let counters_padded: Arc<Vec<PaddedCounter>> = Arc::new(
        (0..NUM_THREADS).map(|_| PaddedCounter {
            value: AtomicU64::new(0),
            _padding: [0; 56],
        }).collect()
    );

    // Test with false sharing
    let start = Instant::now();
    let mut handles = vec![];

    for thread_id in 0..NUM_THREADS {
        let counters = Arc::clone(&counters_false);
        let handle = thread::spawn(move || {
            for _ in 0..ITERATIONS {
                counters[thread_id].fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let false_sharing_time = start.elapsed();

    // Test without false sharing
    let start = Instant::now();
    let mut handles = vec![];

    for thread_id in 0..NUM_THREADS {
        let counters = Arc::clone(&counters_padded);
        let handle = thread::spawn(move || {
            for _ in 0..ITERATIONS {
                counters[thread_id].value.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let padded_time = start.elapsed();

    println!("With false sharing: {:?}", false_sharing_time);
    println!("With padding (no false sharing): {:?}", padded_time);
    println!("False sharing makes it ~{}x slower", false_sharing_time.as_nanos() / padded_time.as_nanos());
    println!();
}

fn demonstrate_struct_layout() {
    println!("🏗️  Struct Layout & Cache Lines");
    println!("==============================");

    // Bad layout: fields likely share cache lines
    struct BadLayout {
        a: u8,
        b: u8,
        c: u8,
        d: u8,
        counter: u64,  // Frequently accessed
    }

    // Good layout: frequently accessed fields separated
    struct GoodLayout {
        counter: u64,  // Frequently accessed
        _padding: [u8; 56], // Pad to cache line boundary
        a: u8,
        b: u8,
        c: u8,
        d: u8,
    }

    println!("Bad layout size: {} bytes", std::mem::size_of::<BadLayout>());
    println!("Good layout size: {} bytes", std::mem::size_of::<GoodLayout>());
    println!("Good layout prevents false sharing of counter field");
    println!();
}

fn demonstrate_prefetching() {
    println!("🔮 Hardware Prefetching");
    println!("======================");

    let size = 1024 * 1024;
    let mut array = vec![0u64; size];

    // Sequential access (hardware can prefetch)
    let start = Instant::now();
    for i in 0..size {
        array[i] += 1;
    }
    let sequential = start.elapsed();

    // Strided access (harder for hardware to prefetch)
    let start = Instant::now();
    for i in (0..size).step_by(64) {  // Skip cache lines
        array[i] += 1;
    }
    let strided = start.elapsed();

    println!("Sequential access: {:?}", sequential);
    println!("Strided access (every 64 elements): {:?}", strided);
    println!("Hardware prefetching helps sequential access");
    println!();
}

fn main() {
    println!("📏 Cache Line Size Demonstration");
    println!("=================================");
    println!("Understanding why 64 bytes matters for performance.\n");

    demonstrate_cache_line_size();
    demonstrate_false_sharing();
    demonstrate_struct_layout();
    demonstrate_prefetching();

    println!("🎯 Key Takeaways:");
    println!("• Cache lines are 64 bytes (not because of word size!)");
    println!("• False sharing can destroy multi-threaded performance");
    println!("• Struct layout affects cache line utilization");
    println!("• Hardware prefetching helps sequential access patterns");
    println!("• Cache-aware programming is crucial for performance");

    println!("\n💡 Pro tip: Use `#[repr(align(64))]` for frequently accessed shared data");
}