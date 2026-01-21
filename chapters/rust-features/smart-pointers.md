# Smart Pointers Guide: Box, Rc, Arc, RefCell, Cell

## Quick Answer

**Yes, they're all smart pointers that manage heap allocation/deallocation, but each solves different problems:**

## 1. Box<T> - Single Ownership, Heap Allocation

**What:** Moves data from stack to heap.

**When to use:**
- **Large data** (doesn't fit on stack)
- **Recursive types** (compile-time size unknown)
- **Trait objects** (dynamic dispatch)

**Example:**
```rust
// Stack (might overflow):
let huge_array = [0u8; 1_000_000];  // ❌ Stack overflow risk

// Heap (safe):
let huge_array = Box::new([0u8; 1_000_000]);  // ✅ On heap

// Recursive type (compile-time size unknown):
enum List {
    Cons(i32, Box<List>),  // ✅ Box needed (size unknown)
    Nil,
}
```

**Memory:**
- **Stack:** Stores pointer (8 bytes) + metadata
- **Heap:** Stores actual data
- **Deallocation:** Automatic when `Box` goes out of scope

---

## 2. Rc<T> - Reference Counting (Single Thread)

**What:** Multiple owners, same data (read-only).

**When to use:**
- **Shared ownership** (multiple variables point to same data)
- **Single-threaded** only
- **Read-only** access (use `RefCell` for mutability)

**Example:**
```rust
use std::rc::Rc;

let data = Rc::new(vec![1, 2, 3]);
let owner1 = Rc::clone(&data);  // Reference count: 2
let owner2 = Rc::clone(&data);  // Reference count: 3

// All point to same Vec on heap
// When all Rc's drop → Vec is freed
```

**Memory:**
- **Stack:** Stores pointer + reference counter
- **Heap:** Stores data + reference count metadata
- **Deallocation:** When reference count reaches 0

---

## 3. Arc<T> - Atomic Reference Counting (Multi-Thread)

**What:** Like `Rc`, but **thread-safe**.

**When to use:**
- **Shared ownership** across **multiple threads**
- **Read-only** access (use `Mutex`/`RwLock` for mutability)

**Example:**
```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);
let data1 = Arc::clone(&data);
let data2 = Arc::clone(&data);

thread::spawn(move || {
    println!("{:?}", data1);  // ✅ Thread-safe
});

thread::spawn(move || {
    println!("{:?}", data2);  // ✅ Thread-safe
});
```

**Memory:**
- Same as `Rc`, but uses **atomic operations** (slower, but thread-safe)

---

## 4. RefCell<T> - Interior Mutability (Single Thread)

**What:** Allows **mutable** access to **immutable** reference (runtime borrow checking).

**When to use:**
- **Interior mutability** (need to mutate through `&T`)
- **Single-threaded** only
- **Runtime borrow checking** (panics if violated)

**Example:**
```rust
use std::cell::RefCell;

let data = RefCell::new(5);
let borrowed = data.borrow();      // ✅ Immutable borrow
let mut_borrowed = data.borrow_mut();  // ❌ Panic! (already borrowed)

// Correct:
{
    let borrowed = data.borrow();
    println!("{}", *borrowed);
}  // Borrow ends here

let mut_borrowed = data.borrow_mut();  // ✅ Now works
*mut_borrowed = 10;
```

**Memory:**
- **Stack:** Stores pointer + borrow state
- **Heap:** Stores data
- **Runtime checks:** Panics if borrow rules violated

---

## 5. Cell<T> - Interior Mutability (Copy Types)

**What:** Like `RefCell`, but for **copy types** (no borrowing, just swap).

**When to use:**
- **Copy types** only (`i32`, `bool`, etc.)
- **No borrowing** (just get/set)
- **Single-threaded** only

**Example:**
```rust
use std::cell::Cell;

let data = Cell::new(5);
let value = data.get();        // ✅ Get copy
data.set(10);                  // ✅ Set new value
let new_value = data.get();   // ✅ Get new value (10)
```

**Memory:**
- **Stack:** Stores data directly (copy types)
- **No heap allocation** (unlike `RefCell`)

---

## Common Combinations

### Rc<RefCell<T>> - Shared Mutable (Single Thread)
```rust
use std::rc::Rc;
use std::cell::RefCell;

let data = Rc::new(RefCell::new(vec![1, 2, 3]));
let owner1 = Rc::clone(&data);
let owner2 = Rc::clone(&data);

owner1.borrow_mut().push(4);  // ✅ Mutate through Rc
```

### Arc<Mutex<T>> - Shared Mutable (Multi-Thread)
```rust
use std::sync::{Arc, Mutex};
use std::thread;

let data = Arc::new(Mutex::new(vec![1, 2, 3]));
let data1 = Arc::clone(&data);

thread::spawn(move || {
    let mut vec = data1.lock().unwrap();
    vec.push(4);  // ✅ Thread-safe mutation
});
```

---

## Stack vs Heap

**Stack:**
- Fast (just move RSP)
- Fixed size (compile-time known)
- Automatic cleanup (RSP moves back)

**Heap:**
- Slower (allocation/deallocation)
- Dynamic size (runtime known)
- Manual cleanup (smart pointers handle it)

**Why use heap?**
1. **Size unknown at compile time** (recursive types, dynamic data)
2. **Too large for stack** (huge arrays)
3. **Shared ownership** (multiple owners)
4. **Lifetime beyond function** (return from function)

---

## Summary Table

| Type | Ownership | Thread-Safe | Mutability | Use Case |
|------|-----------|-------------|------------|----------|
| `Box<T>` | Single | N/A | Yes | Large data, recursive types |
| `Rc<T>` | Multiple | ❌ No | Read-only | Shared ownership (single thread) |
| `Arc<T>` | Multiple | ✅ Yes | Read-only | Shared ownership (multi-thread) |
| `RefCell<T>` | Single | ❌ No | Interior mutability | Mutate through `&T` |
| `Cell<T>` | Single | ❌ No | Interior mutability | Copy types only |

---

## Key Takeaways

1. **All manage heap allocation/deallocation** ✅
2. **Box:** Single owner, heap allocation
3. **Rc/Arc:** Multiple owners, reference counting
4. **RefCell/Cell:** Interior mutability (runtime checks)
5. **Stack vs Heap:** Stack = fast/fixed, Heap = slow/dynamic
6. **Combinations:** `Rc<RefCell<T>>`, `Arc<Mutex<T>>` for shared mutable data
