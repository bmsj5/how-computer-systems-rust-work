//! Rust Language Features Demo
//!
//! Demonstrates Rust's unique features: ownership, borrowing, iterators, error handling.
//! Run with: cargo run --bin rust-language-features

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

fn demonstrate_ownership() {
    println!("👑 Ownership & Borrowing");
    println!("=======================");

    // Ownership transfer
    let owned_string = String::from("Hello");
    println!("1. Created owned string: '{}'", owned_string);

    let moved_string = owned_string; // Ownership moves here
    println!("2. Moved ownership: '{}'", moved_string);

    // println!("3. Original is gone: '{}'", owned_string); // ❌ Compile error!

    // Borrowing
    let borrowed_string = &moved_string; // Immutable borrow
    println!("4. Immutable borrow: '{}'", borrowed_string);

    // Mutable borrow
    let mut mutable_string = String::from("Mutable");
    let mutable_borrow = &mut mutable_string;
    mutable_borrow.push_str(" string");
    println!("5. Mutable borrow result: '{}'", mutable_string);
    println!();
}

fn demonstrate_iterators() {
    println!("🔄 Iterator Performance");
    println!("======================");

    let numbers: Vec<i32> = (1..=1000).collect();

    // Traditional loop
    let start = std::time::Instant::now();
    let mut sum_loop = 0;
    for &num in &numbers {
        if num % 2 == 0 {
            sum_loop += num * 2;
        }
    }
    let loop_time = start.elapsed();

    // Iterator chain
    let start = std::time::Instant::now();
    let sum_iter: i32 = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 2)
        .sum();
    let iter_time = start.elapsed();

    println!("Traditional loop result: {}", sum_loop);
    println!("Iterator chain result: {}", sum_iter);
    println!("Loop time: {:?}", loop_time);
    println!("Iterator time: {:?}", iter_time);
    println!("Iterators are often faster due to LLVM optimizations!");
    println!();
}

fn demonstrate_smart_pointers() {
    println!("🧠 Smart Pointers");
    println!("=================");

    // Box - heap allocation
    let boxed = Box::new(42);
    println!("Boxed value: {}", boxed);

    // Rc - reference counting
    let rc_value = Rc::new(String::from("Shared"));
    let rc_clone1 = Rc::clone(&rc_value);
    let rc_clone2 = Rc::clone(&rc_value);
    println!("RC value: '{}', references: {}", rc_value, Rc::strong_count(&rc_value));

    // RefCell - interior mutability
    let refcell = RefCell::new(vec![1, 2, 3]);
    {
        let mut borrow = refcell.borrow_mut();
        borrow.push(4);
    }
    println!("RefCell after mutation: {:?}", refcell.borrow());
    println!();
}

fn demonstrate_error_handling() {
    println!("⚠️  Error Handling");
    println!("=================");

    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        Overflow,
    }

    fn divide(a: i32, b: i32) -> Result<i32, MathError> {
        if b == 0 {
            return Err(MathError::DivisionByZero);
        }
        if a == i32::MAX && b == 1 {
            return Err(MathError::Overflow);
        }
        Ok(a / b)
    }

    // Using Result
    let result1 = divide(10, 2);
    let result2 = divide(10, 0);

    match result1 {
        Ok(value) => println!("10 ÷ 2 = {}", value),
        Err(e) => println!("Error: {:?}", e),
    }

    match result2 {
        Ok(value) => println!("10 ÷ 0 = {}", value),
        Err(e) => println!("Error: {:?}", e),
    }

    // Using ? operator (would need to be in a function that returns Result)
    println!("The ? operator propagates errors automatically");
    println!();
}

fn demonstrate_pattern_matching() {
    println!("🎯 Pattern Matching");
    println!("==================");

    let data = vec![Some(1), None, Some(3), None, Some(5)];

    println!("Filtering and transforming with pattern matching:");
    let result: Vec<i32> = data.into_iter()
        .filter_map(|x| x) // Remove None values
        .map(|x| x * 2)    // Double the values
        .collect();

    println!("Input: [Some(1), None, Some(3), None, Some(5)]");
    println!("Output: {:?}", result);
    println!();
}

fn demonstrate_traits_and_generics() {
    println!("🔧 Traits & Generics");
    println!("====================");

    trait Printable {
        fn print(&self);
    }

    impl Printable for i32 {
        fn print(&self) { println!("Integer: {}", self); }
    }

    impl Printable for String {
        fn print(&self) { println!("String: {}", self); }
    }

    fn print_twice<T: Printable>(value: T) {
        value.print();
        value.print();
    }

    print_twice(42);
    print_twice(String::from("Hello"));

    // Generic function
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest in {:?} is {}", numbers, largest(&numbers));
    println!();
}

fn demonstrate_lifetimes() {
    println!("⏰ Lifetimes");
    println!("===========");

    // Explicit lifetime annotation
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
    println!("Lifetime ensures the returned reference doesn't outlive its source");
    println!();
}

fn main() {
    println!("🦀 Rust Language Features Demo");
    println!("================================");
    println!("What makes Rust unique and powerful.\n");

    demonstrate_ownership();
    demonstrate_iterators();
    demonstrate_smart_pointers();
    demonstrate_error_handling();
    demonstrate_pattern_matching();
    demonstrate_traits_and_generics();
    demonstrate_lifetimes();

    println!("🎯 Key Takeaways:");
    println!("• Ownership prevents memory bugs at compile time");
    println!("• Borrowing allows efficient zero-cost sharing");
    println!("• Iterators provide functional programming with performance");
    println!("• Smart pointers give you choice over memory management");
    println!("• Result/Option types make error handling explicit");
    println!("• Pattern matching is powerful and ergonomic");
    println!("• Traits enable polymorphism without inheritance");
    println!("• Lifetimes ensure memory safety without garbage collection");
}