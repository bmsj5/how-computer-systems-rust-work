# Array/Vector Indexing and usize

## Your Question

**"How is array/vec slicing related to usize and maximum capacity? Why is index cannot be more than usize - 1? How does indexing actually work?"**

## Answer: How Array Indexing Works

### The Confusion

**You're mixing up two different things:**
1. **usize** - The type used for indexing (can be very large: 2^64)
2. **Array length** - The actual number of elements (stored as usize, but can be any value up to usize::MAX)

**The index limit is the array's length, not usize itself!**

### How Array Indexing Actually Works

**Array indexing formula:**
```
Element address = base_address + (index * element_size)
```

**Step-by-step:**

**1. Array in memory:**
```rust
let array: [u32; 5] = [10, 20, 30, 40, 50];
// Array starts at address: 0x1000 (example)
```

**2. Memory layout:**
```
Address    Content
─────────────────
0x1000     10 (u32, 4 bytes)
0x1004     20 (u32, 4 bytes)
0x1008     30 (u32, 4 bytes)
0x100C     40 (u32, 4 bytes)
0x1010     50 (u32, 4 bytes)
```

**3. Accessing element at index 2:**
```rust
let value = array[2];  // Get element at index 2
```

**What CPU does:**
```
1. Base address: 0x1000 (array start)
2. Index: 2
3. Element size: 4 bytes (u32)
4. Offset = index * element_size = 2 * 4 = 8 bytes
5. Element address = 0x1000 + 8 = 0x1008
6. Read 4 bytes from 0x1008
7. Result: 30
```

### Why usize for Indexing?

**usize is used because:**

1. **Pointer-sized:**
   - usize = size of pointer (64 bits on 64-bit systems)
   - Can address any location in memory
   - Matches address space size

2. **Array length is stored as usize:**
   ```rust
   struct Vec<T> {
       pointer: *mut T,      // Pointer to data
       length: usize,        // Number of elements
       capacity: usize,      // Allocated capacity
   }
   ```

3. **Index must fit in usize:**
   - Index is compared to length (both usize)
   - Index must be < length
   - Both are usize, so comparison is efficient

### Your Example Clarified

**You said: "if usize is 2, this means there are two elements"**

**Correction:**
- **usize is not 2** - usize is a type (64 bits = can hold values 0 to 2^64-1)
- **Array length is 2** - The array has 2 elements
- **Maximum index is 1** - Because indices are 0-based (0, 1)

**Example:**
```rust
let array: [u32; 2] = [10, 20];
// Array length: 2 (stored as usize = 2)
// Valid indices: 0, 1 (both are usize values)
// Invalid index: 2 (out of bounds)

array[0]  // OK: index 0 < length 2
array[1]  // OK: index 1 < length 2
array[2]  // ERROR: index 2 >= length 2
```

### How Indexing Works (Detailed)

**Your code:**
```rust
let array: [u8; 5] = [1, 2, 3, 4, 5];
let value = array[2];  // Get element at index 2
```

**What compiler generates:**
```asm
; Array base address in RDI
; Index in RCX (2)
; Element size: 1 byte (u8)

mov rax, rdi        ; Base address
add rax, rcx        ; Add index (offset = index * 1 for u8)
mov al, [rax]       ; Load 1 byte
```

**For u32 (4 bytes):**
```asm
; Array base address in RDI
; Index in RCX (2)
; Element size: 4 bytes (u32)

mov rax, rdi        ; Base address
mov rdx, rcx        ; Index
shl rdx, 2          ; Multiply by 4 (shift left 2 = *4)
add rax, rdx        ; Add offset
mov eax, [rax]      ; Load 4 bytes
```

**The formula:**
```
offset = index * element_size
address = base_address + offset
```

**usize has nothing to do with the calculation - it's just the type of the index!**

### Maximum Capacity

**Maximum array/vec capacity:**

**Theoretical maximum:**
- usize::MAX elements (2^64 - 1 on 64-bit systems)
- But practical limits are much smaller

**Practical limits:**
1. **Available memory:**
   - Can't allocate more than available RAM
   - 16 GB RAM = ~4 billion u32 elements max

2. **Address space:**
   - Virtual address space is 2^64 bytes
   - But OS limits are usually smaller

3. **Rust's limits:**
   - Vec<T> can hold up to isize::MAX elements (safety)
   - isize::MAX = 2^63 - 1 (signed, so can be negative for bounds checking)

**Example:**
```rust
// Maximum Vec<u8> capacity
let max_capacity = isize::MAX as usize;  // 2^63 - 1
// But you'll run out of RAM long before this!
```

### Why Index Cannot Exceed Length - 1

**Array bounds checking:**
```rust
let array: [u32; 5] = [10, 20, 30, 40, 50];
// Length: 5
// Valid indices: 0, 1, 2, 3, 4
// Maximum index: 4 = length - 1

array[4]  // OK: 4 < 5
array[5]  // ERROR: 5 >= 5 (out of bounds)
```

**Why 0-based indexing:**
- Index represents offset from start
- Index 0 = offset 0 (first element)
- Index 1 = offset 1 * element_size (second element)
- Index n = offset n * element_size

**If 1-based indexing:**
```
Index 1 = first element (offset 0)
Index 2 = second element (offset element_size)
...
Index n = nth element (offset (n-1) * element_size)
```

**0-based is simpler:**
```
Index = offset / element_size
No subtraction needed!
```

### Visual: Array Indexing

**Array: [u32; 5] = [10, 20, 30, 40, 50]**

```
Base address: 0x1000

Index  Element  Address        Calculation
─────────────────────────────────────────────
0      10       0x1000        0x1000 + (0 * 4) = 0x1000
1      20       0x1004        0x1000 + (1 * 4) = 0x1004
2      30       0x1008        0x1000 + (2 * 4) = 0x1008
3      40       0x100C        0x1000 + (3 * 4) = 0x100C
4      50       0x1010        0x1000 + (4 * 4) = 0x1010

Length: 5
Valid indices: 0, 1, 2, 3, 4 (all < 5)
Maximum index: 4 = length - 1
```

### Vec vs Array

**Array (fixed size):**
```rust
let array: [u32; 5] = [10, 20, 30, 40, 50];
// Size known at compile time
// Stored on stack (if small) or in data section
// Length: 5 (compile-time constant)
```

**Vec (dynamic size):**
```rust
let vec: Vec<u32> = vec![10, 20, 30, 40, 50];
// Size known at runtime
// Stored on heap
// Length: 5 (stored as usize in Vec struct)
```

**Vec structure:**
```rust
struct Vec<T> {
    pointer: *mut T,      // Points to heap-allocated data
    length: usize,        // Current number of elements
    capacity: usize,      // Allocated capacity
}
```

**Indexing works the same for both!**

### Summary

| Aspect | Explanation |
|--------|-------------|
| **usize** | Type for indexing (64 bits, can hold 0 to 2^64-1) |
| **Array length** | Number of elements (stored as usize, but can be any value) |
| **Maximum index** | length - 1 (because 0-based indexing) |
| **Indexing formula** | address = base + (index * element_size) |
| **usize role** | Type of index and length, not involved in calculation |

**Key insights:**
1. **usize is a type, not a value** - It can hold very large numbers (2^64)
2. **Array length is a value** - Stored as usize, but can be any number up to usize::MAX
3. **Indexing formula** - address = base + (index * element_size)
4. **usize is just the type** - The actual calculation uses the index value, not usize itself
5. **Maximum index = length - 1** - Because of 0-based indexing

**Your confusion:** You thought "usize is 2" means "2 elements", but actually:
- usize is a type (64 bits)
- Array length is 2 (a value of type usize)
- Maximum index is 1 (length - 1)
