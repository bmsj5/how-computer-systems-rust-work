// Demonstration of array/vec indexing and usize

fn demonstrate_array_indexing() {
    println!("=== Array Indexing Demo ===\n");
    
    let array: [u32; 5] = [10, 20, 30, 40, 50];
    
    println!("Array: {:?}", array);
    println!("Length: {} (stored as usize)", array.len());
    println!("usize can hold: 0 to {} (2^64 - 1)", usize::MAX);
    println!();
    
    println!("Valid indices: 0, 1, 2, 3, 4");
    println!("Maximum index: {} (length - 1)", array.len() - 1);
    println!();
    
    for i in 0..array.len() {
        println!("array[{}] = {}", i, array[i]);
    }
    println!();
    
    println!("Indexing formula:");
    println!("  Element address = base_address + (index * element_size)");
    println!("  For u32: element_size = 4 bytes");
    println!("  array[2] = base + (2 * 4) = base + 8 bytes offset");
    println!();
}

fn demonstrate_vec_indexing() {
    println!("=== Vec Indexing Demo ===\n");
    
    let vec: Vec<u8> = vec![1, 2, 3, 4, 5];
    
    println!("Vec: {:?}", vec);
    println!("Length: {} (stored as usize)", vec.len());
    println!("Capacity: {} (stored as usize)", vec.capacity());
    println!();
    
    println!("Vec structure (simplified):");
    println!("  struct Vec<T> {{");
    println!("      pointer: *mut T,      // Points to heap data");
    println!("      length: usize,        // Current elements: {}", vec.len());
    println!("      capacity: usize,      // Allocated: {}", vec.capacity());
    println!("  }}");
    println!();
    
    for i in 0..vec.len() {
        println!("vec[{}] = {}", i, vec[i]);
    }
    println!();
}

fn demonstrate_usize_vs_length() {
    println!("=== usize vs Array Length ===\n");
    
    println!("Confusion: 'if usize is 2, this means 2 elements'");
    println!("❌ WRONG! usize is a TYPE, not a value!");
    println!();
    
    println!("Correct understanding:");
    println!("  usize = TYPE (64 bits, can hold 0 to 2^64-1)");
    println!("  Array length = VALUE of type usize (e.g., 2, 5, 100)");
    println!();
    
    let small_array: [u8; 2] = [10, 20];
    let large_array: [u8; 1000] = [0; 1000];
    
    println!("small_array: length = {} (value of type usize)", small_array.len());
    println!("large_array: length = {} (value of type usize)", large_array.len());
    println!("Both use usize type, but have different length VALUES");
    println!();
}

fn demonstrate_indexing_calculation() {
    println!("=== Indexing Calculation ===\n");
    
    let array: [u8; 5] = [1, 2, 3, 4, 5];
    
    println!("Array: {:?}", array);
    println!("Element type: u8 (1 byte)");
    println!();
    
    println!("Accessing array[2]:");
    println!("  1. Base address: (some address, e.g., 0x1000)");
    println!("  2. Index: 2 (value of type usize)");
    println!("  3. Element size: 1 byte (u8)");
    println!("  4. Offset = index * element_size = 2 * 1 = 2 bytes");
    println!("  5. Element address = base + 2");
    println!("  6. Read 1 byte from that address");
    println!("  7. Result: {}", array[2]);
    println!();
    
    let array_u32: [u32; 5] = [10, 20, 30, 40, 50];
    
    println!("Array: {:?}", array_u32);
    println!("Element type: u32 (4 bytes)");
    println!();
    
    println!("Accessing array_u32[2]:");
    println!("  1. Base address: (some address, e.g., 0x1000)");
    println!("  2. Index: 2 (value of type usize)");
    println!("  3. Element size: 4 bytes (u32)");
    println!("  4. Offset = index * element_size = 2 * 4 = 8 bytes");
    println!("  5. Element address = base + 8");
    println!("  6. Read 4 bytes from that address");
    println!("  7. Result: {}", array_u32[2]);
    println!();
    
    println!("Key point: usize is just the TYPE of the index!");
    println!("The calculation uses the INDEX VALUE, not usize itself!");
    println!();
}

fn demonstrate_maximum_capacity() {
    println!("=== Maximum Capacity ===\n");
    
    println!("usize::MAX = {} (2^64 - 1 on 64-bit)", usize::MAX);
    println!("isize::MAX = {} (2^63 - 1, Rust's safety limit)", isize::MAX);
    println!();
    
    println!("Theoretical maximum Vec capacity:");
    println!("  usize::MAX elements (but impractical)");
    println!("  Rust limits to isize::MAX for safety");
    println!();
    
    println!("Practical limits:");
    println!("  - Available RAM (16 GB = ~4 billion u32 elements)");
    println!("  - OS address space limits");
    println!("  - You'll run out of RAM before hitting usize::MAX!");
    println!();
}

fn main() {
    println!("=== Array/Vector Indexing and usize ===\n");
    
    demonstrate_array_indexing();
    demonstrate_vec_indexing();
    demonstrate_usize_vs_length();
    demonstrate_indexing_calculation();
    demonstrate_maximum_capacity();
    
    println!("=== Key Takeaways ===");
    println!("1. usize is a TYPE (64 bits), not a value");
    println!("2. Array length is a VALUE of type usize");
    println!("3. Indexing formula: address = base + (index * element_size)");
    println!("4. Maximum index = length - 1 (0-based indexing)");
    println!("5. usize is just the type - calculation uses index value");
}
