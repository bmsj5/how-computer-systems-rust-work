# Rc<T> vs Immutable References (&T): When to Use What

## Quick Answer

**For simple read-only access, use `&T` (immutable references). Use `Rc<T>` only when you need shared ownership (multiple owners that keep the data alive).**

---

## 1. Your Question

**You asked:** "Why use `Rc<T>` if I can just use `&T`?"

**Answer:** You're right! For simple read-only access, `&T` is better. `Rc<T>` is only needed when you need **shared ownership** (multiple owners).

---

## 2. Immutable References (`&T`) - The Normal Way

**What:** Borrow the data temporarily

**When to use:**
- ✅ **Simple read-only access** - Just need to read the data
- ✅ **Temporary access** - Don't need to own the data
- ✅ **Most common case** - This is what you should use 99% of the time

**Example:**
```rust
fn main() {
    let value: u8 = 15;
    read1(&value);  // ✅ Pass immutable reference
    read2(&value);  // ✅ Pass immutable reference
}

fn read1(value: &u8) {
    println!("{}", value);  // ✅ Just read
}

fn read2(value: &u8) {
    println!("{}", value);  // ✅ Just read
}
```

**Advantages:**
- ✅ Simple and straightforward
- ✅ Zero overhead
- ✅ Compile-time checked
- ✅ Most common pattern

**Limitations:**
- ❌ Temporary access only (borrowing)
- ❌ Doesn't own the data
- ❌ Data must outlive the reference

---

## 3. Rc<T> - Shared Ownership

**What:** Multiple owners that keep data alive

**When to use:**
- ✅ **Shared ownership** - Multiple variables need to own the same data
- ✅ **Data must live as long as any owner** - Not just borrowed
- ✅ **When you can't use references** - References have lifetime constraints

**Example:**
```rust
use std::rc::Rc;

fn main() {
    let data = Rc::new(vec![1, 2, 3]);
    let owner1 = Rc::clone(&data);  // Owner 1
    let owner2 = Rc::clone(&data);  // Owner 2
    
    // Both owners can use the data
    use_data(&owner1);
    use_data(&owner2);
    
    // Data lives as long as any owner exists
}  // data, owner1, owner2 all go out of scope here
// Data is freed when last owner is dropped
```

**Advantages:**
- ✅ Multiple owners
- ✅ Data lives as long as any owner
- ✅ No lifetime constraints

**Disadvantages:**
- ❌ Runtime overhead (reference counting)
- ❌ More complex
- ❌ Single-threaded only

---

## 4. Key Difference: Ownership vs Borrowing

### Immutable References (`&T`) - Borrowing

```rust
fn main() {
    let value: u8 = 15;
    read(&value);  // Borrow (temporary access)
}  // value is dropped here

fn read(value: &u8) {
    println!("{}", value);  // Just reading
}
```

**Characteristics:**
- ✅ Borrowing (temporary access)
- ✅ Original owner still owns the data
- ✅ Reference doesn't keep data alive
- ✅ Zero overhead

---

### Rc<T> - Shared Ownership

```rust
use std::rc::Rc;

fn main() {
    let data = Rc::new(15);
    let owner1 = Rc::clone(&data);  // Owner 1
    let owner2 = Rc::clone(&data);  // Owner 2
    
    // Both own the data
}  // All owners go out of scope
// Data is freed when last owner is dropped
```

**Characteristics:**
- ✅ Multiple owners
- ✅ Each owner keeps data alive
- ✅ Data lives as long as any owner
- ⚠️ Runtime overhead (reference counting)

---

## 5. When to Use What

### ✅ Use Immutable References (`&T`) When:

**Simple read-only access (99% of cases):**
```rust
fn read_data(data: &Vec<i32>) {
    for item in data {
        println!("{}", item);  // ✅ Just reading
    }
}
```

**Passing to functions:**
```rust
let data = vec![1, 2, 3];
process(&data);  // ✅ Pass reference
```

**This is the preferred way!**

---

### ✅ Use Rc<T> When:

**Need shared ownership (multiple owners):**
```rust
use std::rc::Rc;

let data = Rc::new(vec![1, 2, 3]);
let owner1 = Rc::clone(&data);  // Owner 1
let owner2 = Rc::clone(&data);  // Owner 2

// Both own the data - it lives as long as any owner
```

**When references have lifetime issues:**
```rust
// Can't use references because of lifetime constraints
// Need ownership instead
let shared = Rc::new(data);
```

**When you need data to live beyond the original scope:**
```rust
use std::rc::Rc;

fn create_shared() -> Rc<Vec<i32>> {
    let data = vec![1, 2, 3];
    Rc::new(data)  // ✅ Can return Rc (shared ownership)
}

// Can't do this with references (lifetime issues):
// fn create_ref() -> &Vec<i32> {  // ❌ Can't return reference to local
//     let data = vec![1, 2, 3];
//     &data
// }
```

---

## 6. Real-World Example

### Scenario: Reading data from multiple functions

**With immutable references (preferred):**
```rust
fn main() {
    let data = vec![1, 2, 3];
    read1(&data);  // ✅ Pass reference
    read2(&data);  // ✅ Pass reference
}

fn read1(data: &Vec<i32>) {
    println!("{:?}", data);  // Just reading
}

fn read2(data: &Vec<i32>) {
    println!("{:?}", data);  // Just reading
}
```

**✅ This is the better approach!**

**With Rc (only if you need shared ownership):**
```rust
use std::rc::Rc;

fn main() {
    let data = Rc::new(vec![1, 2, 3]);
    let owner1 = Rc::clone(&data);
    let owner2 = Rc::clone(&data);
    
    read1(&owner1);
    read2(&owner2);
}

fn read1(data: &Rc<Vec<i32>>) {
    println!("{:?}", data);  // Reading
}

fn read2(data: &Rc<Vec<i32>>) {
    println!("{:?}", data);  // Reading
}
```

**⚠️ More complex, only needed if you need shared ownership!**

---

## 7. Comparison Table

| Feature | Immutable References (`&T`) | Rc<T> |
|---------|----------------------------|-------|
| **Use case** | Simple read-only access | Shared ownership |
| **Ownership** | Borrowing (temporary) | Multiple owners |
| **Overhead** | Zero | Runtime (reference counting) |
| **Complexity** | Simple | More complex |
| **Preferred** | ✅ Yes (99% of cases) | ⚠️ Only when needed |

---

## 8. Summary

**Rule of thumb:**
1. **Use `&T` (immutable references)** - For simple read-only access (99% of cases)
2. **Use `Rc<T>`** - Only when you need shared ownership (multiple owners)

**Your intuition is correct!** For simple read-only access, use `&T`. `Rc<T>` is only needed when you need multiple owners that keep the data alive.

---

## 9. When You Actually Need Rc

**You need `Rc<T>` when:**
- Multiple variables need to own the same data
- Data must live as long as any owner
- You can't use references due to lifetime constraints
- You need to return shared data from functions

**You don't need `Rc<T>` when:**
- Just reading data (use `&T`)
- Simple function parameters (use `&T`)
- Temporary access (use `&T`)

**Bottom line:** For your use case (just reading data), `&T` is the right choice!
