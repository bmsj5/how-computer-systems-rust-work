// Demonstration of register usage and limitations

fn demonstrate_register_usage() {
    println!("=== Register Usage Demo ===\n");
    
    // Simple function - variables likely in registers
    let a = 1;  // Likely in RAX
    let b = 2;  // Likely in RBX
    let c = 3;  // Likely in RCX
    let d = 4;  // Likely in RDX
    
    let sum = a + b + c + d;
    println!("Sum of 4 variables: {}", sum);
    println!("These 4 variables likely fit in registers\n");
    
    // Many variables - some will spill to stack
    let v1 = 1;
    let v2 = 2;
    let v3 = 3;
    let v4 = 4;
    let v5 = 5;
    let v6 = 6;
    let v7 = 7;
    let v8 = 8;
    let v9 = 9;
    let v10 = 10;
    let v11 = 11;
    let v12 = 12;
    let v13 = 13;
    let v14 = 14;
    let v15 = 15;
    let v16 = 16;
    let v17 = 17;  // This might spill to stack!
    
    let total = v1 + v2 + v3 + v4 + v5 + v6 + v7 + v8 + 
                v9 + v10 + v11 + v12 + v13 + v14 + v15 + v16 + v17;
    println!("Sum of 17 variables: {}", total);
    println!("Some variables likely spilled to stack (slower!)\n");
}

fn demonstrate_byte_access() {
    println!("=== Byte-Level Register Access ===\n");
    
    // x86-64 registers can access different sizes
    let value: u64 = 0x1234567890ABCDEF;
    
    println!("Full 64-bit value: 0x{:x}", value);
    println!("32-bit (EAX): 0x{:x}", value as u32);
    println!("16-bit (AX): 0x{:x}", value as u16);
    println!("8-bit (AL): 0x{:x}", (value & 0xFF) as u8);
    println!();
    println!("✅ Registers can hold 1 byte (via AL, AH, etc.)");
    println!("✅ Registers can hold 2 bytes (via AX)");
    println!("✅ Registers can hold 4 bytes (via EAX)");
    println!("✅ Registers can hold 8 bytes (via RAX)\n");
}

fn demonstrate_cache_line_relationship() {
    println!("=== Register vs Cache Line ===\n");
    
    println!("Register:");
    println!("  - Size: 8 bytes each (64-bit)");
    println!("  - Count: 16 general-purpose = 128 bytes total");
    println!("  - Speed: 1 cycle (fastest)");
    println!("  - Purpose: Active computation\n");
    
    println!("Cache Line:");
    println!("  - Size: 64 bytes each");
    println!("  - Count: Thousands (depends on cache size)");
    println!("  - Speed: 1-3 cycles (very fast)");
    println!("  - Purpose: Data transfer from RAM\n");
    
    println!("Relationship:");
    println!("  - Register size (8 bytes) ≠ Cache line size (64 bytes)");
    println!("  - No direct correlation");
    println!("  - Different purposes:");
    println!("    * Registers: Active computation");
    println!("    * Cache lines: Data from RAM\n");
    
    // Demonstrate cache line efficiency
    let array: Vec<u64> = (0..16).collect();  // 16 × 8 bytes = 128 bytes
    
    println!("Array access example:");
    println!("  Array size: {} elements = {} bytes", array.len(), array.len() * 8);
    println!("  Cache line: 64 bytes = 8 × u64 elements");
    println!("  Accessing array[0] loads array[0..7] into cache");
    println!("  Accessing array[1..7] is now fast (cache hit)\n");
}

fn main() {
    println!("=== CPU Registers: Deep Dive ===\n");
    
    demonstrate_register_usage();
    demonstrate_byte_access();
    demonstrate_cache_line_relationship();
    
    println!("=== Key Takeaways ===");
    println!("1. Only 16 general-purpose registers (very limited!)");
    println!("2. Registers can hold 1, 2, 4, or 8 bytes");
    println!("3. Running out of registers causes spilling (slow!)");
    println!("4. Cache line size (64 bytes) ≠ Register size (8 bytes)");
    println!("5. They serve different purposes (computation vs data transfer)");
}
