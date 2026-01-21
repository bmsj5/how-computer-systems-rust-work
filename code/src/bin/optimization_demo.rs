// Demonstration of LLVM optimizations

fn constant_folding() -> i32 {
    // LLVM will calculate this at compile time
    let x = 5 + 10;
    let y = x * 2;
    y
}

fn dead_code_elimination() -> i32 {
    let x = 5;      // Used
    let y = 10;     // Never used - LLVM removes this
    let z = 20;     // Never used - LLVM removes this
    x
}

fn type_example() {
    // Rust's type system: explicit types
    let x: u32 = 5;        // Always i32 (Rust won't change this)
    let y: u8 = 5;         // u8 (you specified)
    
    // LLVM can optimize operations, but won't change Rust's types
    let _result = x + y as u32;
    println!("result: {}", _result);
}

fn main() {
    println!("=== LLVM Optimization Demo ===\n");
    
    println!("1. Constant Folding:");
    println!("   Code: let x = 5 + 10; let y = x * 2;");
    println!("   LLVM optimizes to: let y = 30; (calculated at compile time)");
    println!("   Result: {}\n", constant_folding());
    
    println!("2. Dead Code Elimination:");
    println!("   Code: let x = 5; let y = 10; let z = 20; return x;");
    println!("   LLVM removes: y and z (never used)");
    println!("   Result: {}\n", dead_code_elimination());
    
    println!("3. Type Optimization:");
    println!("   Rust type system prevents automatic type narrowing");
    println!("   let x: i32 = 5;  // Always i32, not u8");
    println!("   But LLVM optimizes operations on these types");
    type_example();
    
    println!("\n=== Key Points ===");
    println!("- LLVM does extensive optimizations");
    println!("- But respects Rust's type system");
    println!("- Type narrowing (i32 → u8) doesn't happen automatically");
    println!("- Constant folding, dead code elimination, inlining all happen");
}
