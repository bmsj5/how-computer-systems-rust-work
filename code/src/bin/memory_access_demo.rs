// Demonstration of how memory access works

fn demonstrate_variable_access() {
    println!("=== How Variables are Accessed ===\n");
    
    // Example: Local variables on stack
    let x: u8 = 42;      // 1 byte
    let y: u32 = 100;    // 4 bytes
    let z: u64 = 1000;   // 8 bytes
    
    println!("Variables on stack:");
    println!("  x: u8 = {} (1 byte at some address)", x);
    println!("  y: u32 = {} (4 bytes at some address)", y);
    println!("  z: u64 = {} (8 bytes at some address)", z);
    println!();
    
    // Get addresses
    let x_ptr = &x as *const u8 as usize;
    let y_ptr = &y as *const u32 as usize;
    let z_ptr = &z as *const u64 as usize;
    
    println!("Addresses (virtual):");
    println!("  x: 0x{:x}", x_ptr);
    println!("  y: 0x{:x}", y_ptr);
    println!("  z: 0x{:x}", z_ptr);
    println!();
    
    println!("What happens when you access x:");
    println!("  1. CPU has address: 0x{:x}", x_ptr);
    println!("  2. CPU uses page table to find physical page");
    println!("  3. CPU accesses byte at offset within that page");
    println!("  4. Returns 1 byte (u8)");
    println!();
    
    println!("What happens when you access y:");
    println!("  1. CPU has address: 0x{:x}", y_ptr);
    println!("  2. CPU uses page table to find physical page");
    println!("  3. CPU accesses 4 bytes starting at that address");
    println!("  4. Returns 4 bytes (u32)");
    println!();
}

fn demonstrate_stack_layout() {
    println!("=== Stack Layout ===\n");
    
    fn inner_function() {
        let a: u8 = 1;   // At RSP - 1
        let b: u32 = 2;  // At RSP - 5 (1 byte for a + 4 bytes for b)
        let c: u64 = 3;  // At RSP - 13 (previous + 8 bytes for c)
        
        println!("Function stack frame:");
        println!("  RSP (stack pointer) points to top of stack");
        println!("  a (u8):  at RSP - 1  (1 byte)");
        println!("  b (u32): at RSP - 5  (4 bytes)");
        println!("  c (u64): at RSP - 13 (8 bytes)");
        println!();
        println!("Compiler calculates these offsets at compile time!");
        println!("OS doesn't know about individual variables - only pages!");
    }
    
    inner_function();
}

fn demonstrate_page_table() {
    println!("=== Page Table Translation ===\n");
    
    println!("Virtual Address: 0x7FFF12345678");
    println!("  └─ Split into:");
    println!("      Page number: 0x7FFF12345 (high bits)");
    println!("      Offset:      0x678 (low 12 bits, within 4KB page)");
    println!();
    
    println!("Translation process:");
    println!("  1. CPU extracts page number: 0x7FFF12345");
    println!("  2. CPU looks up in page table:");
    println!("     Page table[0x7FFF12345] → Physical page: 0x5000");
    println!("  3. CPU combines:");
    println!("     Physical address = 0x5000 + 0x678 = 0x5000678");
    println!("  4. CPU accesses physical RAM at 0x5000678");
    println!();
    
    println!("Key insight:");
    println!("  - OS manages pages (4KB chunks)");
    println!("  - Compiler manages offsets within pages");
    println!("  - CPU translates virtual → physical automatically");
    println!();
}

fn demonstrate_program_isolation() {
    println!("=== Program Isolation ===\n");
    
    println!("Each process has its own virtual address space:");
    println!("  Process A: Virtual addresses 0x0000...0000 to 0xFFFF...FFFF");
    println!("  Process B: Virtual addresses 0x0000...0000 to 0xFFFF...FFFF");
    println!("  (Same virtual addresses, different physical pages!)");
    println!();
    
    println!("OS tracks:");
    println!("  - Page table for each process");
    println!("  - RSP initial value (stack start)");
    println!("  - Reserved stack space (usually 2-8 MB)");
    println!("  - Heap space (grows dynamically)");
    println!();
    
    println!("OS does NOT track:");
    println!("  - Individual variables (x, y, z)");
    println!("  - Variable offsets (RSP - 1, RSP - 5)");
    println!("  - Variable types (u8, u32, u64)");
    println!("  (Compiler handles this!)");
    println!();
}

fn main() {
    println!("=== Memory Access: Deep Dive ===\n");
    
    demonstrate_variable_access();
    demonstrate_stack_layout();
    demonstrate_page_table();
    demonstrate_program_isolation();
    
    println!("=== Key Takeaways ===");
    println!("1. OS manages pages (4KB chunks), not individual variables");
    println!("2. Compiler calculates offsets (RSP - offset) at compile time");
    println!("3. CPU translates virtual addresses to physical using page table");
    println!("4. Each process has isolated virtual address space");
    println!("5. OS tracks page tables, RSP, stack/heap regions - not variables");
}
