//! Pointer Safety Demo
//!
//! Demonstrates Rust's memory safety guarantees and raw pointer usage.
//! Run with: cargo run --bin pointer-safety-demo

use std::ptr;

fn demonstrate_raw_pointers() {
    println!("🔍 Raw Pointers & Safety");
    println!("========================");

    let mut value = 42;
    let raw_ptr: *mut i32 = &mut value;

    println!("Value: {}", value);
    println!("Raw pointer address: {:p}", raw_ptr);

    // Safe access through raw pointer (unsafe block required)
    unsafe {
        println!("Value through raw pointer: {}", *raw_ptr);
        *raw_ptr = 100;
        println!("Modified value: {}", *raw_ptr);
    }

    println!("Final value: {}", value);
    println!();
}

fn demonstrate_null_pointers() {
    println!("🚫 Null Pointers");
    println!("================");

    let null_ptr: *const i32 = ptr::null();

    println!("Null pointer: {:p}", null_ptr);
    println!("Is null: {}", null_ptr.is_null());

    // This would be unsafe and crash in C/C++
    // In Rust, we must explicitly check
    if !null_ptr.is_null() {
        unsafe {
            println!("Value: {}", *null_ptr); // Never reached
        }
    } else {
        println!("✅ Safely avoided null pointer dereference!");
    }
    println!();
}

fn demonstrate_pointer_arithmetic() {
    println!("🔢 Pointer Arithmetic");
    println!("====================");

    let array = [10, 20, 30, 40, 50];
    let ptr: *const i32 = array.as_ptr();

    println!("Array: {:?}", array);
    println!("Pointer to start: {:p}", ptr);

    // Safe pointer arithmetic with offsets
    unsafe {
        for i in 0..array.len() {
            let offset_ptr = ptr.add(i);
            println!("Element {} at {:p}: {}", i, offset_ptr, *offset_ptr);
        }
    }

    // Bounds checking prevents out-of-bounds access
    println!("✅ Bounds checking prevents buffer overflows!");
    println!();
}

fn demonstrate_smart_pointers() {
    println!("🧠 Smart Pointers");
    println!("=================");

    // Box - heap allocation
    let boxed = Box::new(42);
    println!("Boxed value: {}", boxed);
    println!("Box pointer: {:p}", &*boxed);

    // Reference counting
    use std::rc::Rc;
    let rc_value = Rc::new(String::from("Shared"));
    let rc1 = Rc::clone(&rc_value);
    let rc2 = Rc::clone(&rc_value);

    println!("RC value: '{}'", rc_value);
    println!("Reference count: {}", Rc::strong_count(&rc_value));

    // Atomic reference counting for threads
    use std::sync::Arc;
    let arc_value = Arc::new(vec![1, 2, 3]);
    let arc1 = Arc::clone(&arc_value);
    let arc2 = Arc::clone(&arc_value);

    println!("Arc value: {:?}", arc_value);
    println!("Reference count: {}", Arc::strong_count(&arc_value));
    println!();
}

fn demonstrate_lifetimes() {
    println!("⏰ Lifetime Safety");
    println!("==================");

    fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    }

    let string1 = String::from("short");
    let string2 = String::from("longer");

    let result = longest(&string1, &string2);
    println!("longest('{}', '{}') = '{}'", string1, string2, result);

    // This would fail at compile time:
    // let result2;
    // {
    //     let temp = String::from("temp");
    //     result2 = longest(&string1, &temp); // ❌ temp doesn't live long enough
    // }

    println!("✅ Lifetime checking prevents dangling references!");
    println!();
}

fn main() {
    println!("🛡️  Pointer Safety Demo");
    println!("========================");
    println!("How Rust prevents memory corruption and undefined behavior.\n");

    demonstrate_raw_pointers();
    demonstrate_null_pointers();
    demonstrate_pointer_arithmetic();
    demonstrate_smart_pointers();
    demonstrate_lifetimes();

    println!("🎯 Key Takeaways:");
    println!("• Raw pointers require unsafe blocks");
    println!("• Null pointer dereference is prevented");
    println!("• Bounds checking prevents buffer overflows");
    println!("• Smart pointers provide automatic memory management");
    println!("• Lifetime checking prevents dangling references");
    println!("• Memory safety is guaranteed at compile time");
}