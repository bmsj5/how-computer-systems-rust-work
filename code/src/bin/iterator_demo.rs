// Comprehensive demonstration of Rust iterators

fn main() {
    println!("=== Rust Iterators Explained ===\n");
    
    // Example data
    let vec = vec![1, 2, 3, 4, 5];
    let array = [10, 20, 30, 40, 50];
    
    println!("1. Traditional Loop (Index-based):");
    println!("   Code:");
    println!("   for i in 0..vec.len() {{");
    println!("       println!(\"{{}}\", vec[i]);");
    println!("   }}");
    println!("   Output:");
    for i in 0..vec.len() {
        print!("{} ", vec[i]);
    }
    println!("\n");
    
    println!("2. Iterator (Element-based):");
    println!("   Code:");
    println!("   for element in &vec {{");
    println!("       println!(\"{{}}\", element);");
    println!("   }}");
    println!("   Output:");
    for element in &vec {
        print!("{} ", element);
    }
    println!("\n");
    
    println!("3. Iterator Methods (Functional Style):");
    println!("   Code:");
    println!("   vec.iter().for_each(|x| println!(\"{{}}\", x));");
    println!("   Output:");
    vec.iter().for_each(|x| print!("{} ", x));
    println!("\n");
    
    println!("4. Transforming Data:");
    println!("   Traditional loop:");
    let mut doubled = Vec::new();
    for i in 0..vec.len() {
        doubled.push(vec[i] * 2);
    }
    println!("   Result: {:?}", doubled);
    
    println!("   Iterator (map):");
    let doubled_iter: Vec<i32> = vec.iter().map(|x| x * 2).collect();
    println!("   Result: {:?}", doubled_iter);
    println!();
    
    println!("5. Filtering Data:");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    println!("   Traditional loop (even numbers):");
    let mut evens = Vec::new();
    for i in 0..numbers.len() {
        if numbers[i] % 2 == 0 {
            evens.push(numbers[i]);
        }
    }
    println!("   Result: {:?}", evens);
    
    println!("   Iterator (filter):");
    let evens_iter: Vec<i32> = numbers.iter()
        .filter(|x| *x % 2 == 0)
        .copied()
        .collect();
    println!("   Result: {:?}", evens_iter);
    println!();
    
    println!("6. Chaining Operations:");
    let result: Vec<i32> = numbers.iter()
        .filter(|x| *x % 2 == 0)  // Keep even numbers
        .map(|x| x * 3)            // Multiply by 3
        .collect();
    println!("   Even numbers × 3: {:?}", result);
    println!();
    
    println!("7. Summing:");
    let sum_loop: i32 = {
        let mut s = 0;
        for i in 0..numbers.len() {
            s += numbers[i];
        }
        s
    };
    println!("   Loop sum: {}", sum_loop);
    
    let sum_iter: i32 = numbers.iter().sum();
    println!("   Iterator sum: {}", sum_iter);
    println!();
    
    println!("8. Finding Elements:");
    let found_loop = {
        let mut found = None;
        for i in 0..numbers.len() {
            if numbers[i] > 5 {
                found = Some(numbers[i]);
                break;
            }
        }
        found
    };
    println!("   Loop find (>5): {:?}", found_loop);
    
    let found_iter = numbers.iter().find(|&&x| x > 5);
    println!("   Iterator find (>5): {:?}", found_iter);
    println!();
    
    println!("9. Array Iteration:");
    println!("   Arrays work the same way:");
    for element in &array {
        print!("{} ", element);
    }
    println!("\n");
    
    println!("10. Performance Note:");
    println!("    - Iterators are often optimized BETTER than loops");
    println!("    - LLVM can optimize iterators more aggressively");
    println!("    - No bounds checking overhead (iterator knows bounds)");
    println!("    - More idiomatic Rust code");
    println!();
    
    println!("=== When to Use What ===");
    println!("✅ Use iterators for:");
    println!("   - Transforming data (map)");
    println!("   - Filtering data (filter)");
    println!("   - Chaining operations");
    println!("   - Functional-style code");
    println!();
    println!("✅ Use loops for:");
    println!("   - Complex control flow (break, continue)");
    println!("   - When you need the index");
    println!("   - When iterators become too complex");
    println!();
    println!("💡 Best practice: Prefer iterators, use loops when needed");
}
