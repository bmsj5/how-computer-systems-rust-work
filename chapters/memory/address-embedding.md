# Address Embedding: Compile Time vs Runtime

## Question 4: Is Address Embedded During Compilation?

**Your question:** "But the address is already embedded during the compilation stage, right?"

**Answer:** **Partially! Static addresses are known at compile time, but dynamic addresses (stack, heap) are calculated at runtime.**

### What's Known at Compile Time

**1. Code addresses (mostly):**
```rust
fn my_function() {
    // Function address is known at compile time
    // (relative to code section start)
}

// Compiler knows:
// my_function is at offset 0x1234 from code start
// Final address = code_base + 0x1234 (calculated at load time)
```

**2. Global variables:**
```rust
static GLOBAL: i32 = 42;

// Compiler knows:
// GLOBAL is at offset 0x5678 from data section start
// Final address = data_base + 0x5678 (calculated at load time)
```

**3. String literals:**
```rust
let s = "Hello";

// Compiler knows:
// String is at offset 0x9ABC from data section
// Final address = data_base + 0x9ABC (calculated at load time)
```

### What's NOT Known at Compile Time

**1. Stack addresses:**
```rust
fn example() {
    let x: u32 = 100;  // Address = RSP - offset
    // RSP is only known at runtime!
    // Offset is known at compile time
    // Final address = RSP - offset (calculated at runtime)
}
```

**2. Heap addresses:**
```rust
let vec = Vec::new();
vec.push(1);  // Heap address allocated at runtime
// Address only known after allocation
```

**3. Dynamic allocations:**
```rust
let ptr = Box::new(5);  // Address allocated at runtime
// Compiler doesn't know address
```

### How It Actually Works

**Compile time:**
```
Compiler generates:
  - Offsets (relative addresses)
  - Instructions with offsets
  - Symbol table (variable → offset mapping)
```

**Runtime:**
```
OS loads program:
  - Sets code_base (where code section starts)
  - Sets data_base (where data section starts)
  - Sets stack_base (where stack starts)

CPU calculates addresses:
  - Code address = code_base + offset
  - Data address = data_base + offset
  - Stack address = stack_base - offset (or RSP - offset)
```

### Example: Function Call

**Compile time:**
```asm
; Compiler generates:
call my_function
; Offset: 0x1234 (relative to current instruction)
```

**Runtime:**
```
OS loads code at: 0x400000
Current instruction at: 0x400500
my_function address = 0x400500 + 0x1234 = 0x401734
```

### Example: Stack Variable

**Compile time:**
```asm
; Compiler generates:
mov eax, [rbp - 4]
; Offset: -4 (known at compile time)
```

**Runtime:**
```
RBP = 0x7FFF...F000 (set at runtime)
Variable address = 0x7FFF...F000 - 4 = 0x7FFF...EFFC
```

### Position Independent Code (PIC)

**Modern code is often position-independent:**

**Traditional (absolute addresses):**
```asm
call 0x401734  ; Hard-coded address
```

**Position-independent (relative addresses):**
```asm
call $+0x1234  ; Relative to current instruction
; Works regardless of where code is loaded
```

**Benefits:**
- Code can be loaded at any address
- Enables ASLR (Address Space Layout Randomization)
- More secure (harder to exploit)

### Summary

| Address Type | Known at Compile Time? | How It Works |
|--------------|------------------------|--------------|
| **Code addresses** | Offset yes, absolute no | code_base + offset (at load time) |
| **Global variables** | Offset yes, absolute no | data_base + offset (at load time) |
| **Stack variables** | Offset yes, address no | RSP - offset (at runtime) |
| **Heap allocations** | No | Allocated at runtime |
| **Function calls** | Offset yes, absolute no | Relative or code_base + offset |

**Key insight:** Compiler knows offsets (relative addresses), but absolute addresses are calculated at runtime using base addresses set by OS. Stack and heap addresses are completely runtime.
