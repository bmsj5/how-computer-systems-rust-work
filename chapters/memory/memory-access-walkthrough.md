# Memory Access: Complete Walkthrough

## Your Question

"How does accessing a u8 or u32 work? How does the OS find the value? Why do we need page tables? How does OS track program memory?"

## The Complete Pipeline

### Step 1: Compiler Calculates Offset

**Your code:**
```rust
fn example() {
    let x: u8 = 42;   // 1 byte
    let y: u32 = 100; // 4 bytes
}
```

**What compiler does:**
1. Creates symbol table:
   - `x`: offset = RSP - 1 (1 byte)
   - `y`: offset = RSP - 5 (4 bytes, after x)

2. Generates assembly:
```asm
; Function prologue
push rbp
mov rbp, rsp
sub rsp, 8        ; Allocate 8 bytes on stack

; let x: u8 = 42;
mov byte [rbp - 1], 42    ; Store 1 byte at RBP - 1

; let y: u32 = 100;
mov dword [rbp - 5], 100  ; Store 4 bytes at RBP - 5
```

**Key point:** Compiler knows offsets (RBP - 1, RBP - 5) at compile time!

### Step 2: Runtime - RSP Points to Stack

**When function is called:**
- RSP (Stack Pointer) = 0x7FFF...F000 (example virtual address)
- RBP (Base Pointer) = RSP (after prologue)

**Stack layout:**
```
Virtual Address    Content
─────────────────────────────────
0x7FFF...F000     (RSP/RBP - top of stack)
0x7FFF...EFFF     x (u8, 1 byte) ← RBP - 1
0x7FFF...EFFE     (padding)
0x7FFF...EFFD     (padding)
0x7FFF...EFFC     (padding)
0x7FFF...EFFB     y (u32, 4 bytes) ← RBP - 5
```

### Step 3: Accessing Variable `x` (u8)

**When you do `let value = x;`:**

#### 3.1: CPU Calculates Virtual Address

**Assembly:**
```asm
mov al, [rbp - 1]  ; Load 1 byte from RBP - 1
```

**CPU calculates:**
- RBP = 0x7FFF...F000 (from register)
- Offset = -1
- Virtual address = 0x7FFF...F000 - 1 = 0x7FFF...EFFF

#### 3.2: CPU Splits Address into Page Number + Offset

**Virtual address:** 0x7FFF...EFFF

**Split:**
- **Page number:** 0x7FFF...E (high bits, identifies which 4KB page)
- **Offset:** 0xFFF (low 12 bits, position within the 4KB page)

**Why 12 bits?**
- Page size = 4 KB = 4096 bytes = 2^12 bytes
- Need 12 bits to address all bytes within a page (0 to 4095)

#### 3.3: CPU Looks Up Page Table

**Page table (managed by OS):**
```
Virtual Page    Physical Page    Flags
─────────────────────────────────────────
0x7FFF...E      → 0x5000        RW (read-write)
0x7FFF...F      → 0x6000        RW
...
```

**CPU:**
1. Extracts page number: 0x7FFF...E
2. Looks up in page table: 0x7FFF...E → 0x5000 (physical page)
3. Combines: Physical address = 0x5000 + 0xFFF = 0x5FFF

#### 3.4: CPU Accesses Physical RAM

**Physical RAM:**
```
Physical Address    Content
─────────────────────────────
0x5FFF            42 (1 byte, u8)
```

**CPU:**
1. Reads 1 byte from physical address 0x5FFF
2. Returns value: 42
3. Stores in register AL (1 byte)

**Done!** You have the value.

### Step 4: Accessing Variable `y` (u32)

**When you do `let value = y;`:**

**Assembly:**
```asm
mov eax, [rbp - 5]  ; Load 4 bytes from RBP - 5
```

**Process:**
1. Virtual address = 0x7FFF...F000 - 5 = 0x7FFF...EFFB
2. Page number = 0x7FFF...E, Offset = 0xFFB
3. Page table lookup: 0x7FFF...E → 0x5000
4. Physical address = 0x5000 + 0xFFB = 0x5FFB
5. CPU reads 4 bytes starting at 0x5FFB
6. Returns value: 100 (u32)

**Key difference:** CPU reads 4 bytes instead of 1 byte (because it's u32).

## Why Page Tables?

### Without Page Tables (Direct Physical Addressing)

**Problem:**
- Every program would use physical addresses directly
- Programs could access each other's memory
- No security/isolation
- Hard to manage memory

### With Page Tables (Virtual Memory)

**Benefits:**
1. **Isolation:** Each process has its own virtual address space
2. **Security:** Processes can't access each other's memory
3. **Flexibility:** OS can move pages around (swap to disk)
4. **Simplicity:** Programs use simple virtual addresses

## How OS Tracks Program Memory

### What OS Tracks

**For each process, OS maintains:**

1. **Page table:**
   - Maps virtual pages → physical pages
   - Stored in kernel memory
   - CPU uses this for translation

2. **Process memory regions:**
   - Stack: Virtual addresses 0x7FFF...F000 to 0x7FFF...0000 (example)
   - Heap: Virtual addresses 0x1000...0000 to 0x2000...0000 (example)
   - Code: Virtual addresses 0x4000...0000 to 0x5000...0000 (example)

3. **Initial RSP:**
   - OS sets RSP to stack start when process starts
   - Example: RSP = 0x7FFF...F000 (top of stack)

4. **Reserved stack space:**
   - OS reserves 2-8 MB for stack
   - Not all allocated immediately (lazy allocation)

### What OS Does NOT Track

**OS does NOT track:**
- Individual variables (x, y, z)
- Variable offsets (RBP - 1, RBP - 5)
- Variable types (u8, u32, u64)
- Variable names

**Why?**
- This is the compiler's job!
- OS only cares about pages (4KB chunks)
- Compiler calculates offsets at compile time

## Your Understanding - Corrected

### Your Question

"Does OS know where that exact program starts and finishes and it knows this for every running program?"

### Answer

**Partially correct, but with important clarifications:**

**OS knows:**
- ✅ Virtual address ranges for each process (stack, heap, code)
- ✅ Page tables (virtual → physical mapping)
- ✅ Initial RSP value
- ✅ Reserved stack/heap space

**OS does NOT know:**
- ❌ Individual variables (x, y, z)
- ❌ Variable offsets (RBP - 1, RBP - 5)
- ❌ Variable types (u8, u32, u64)

**How it works:**
1. **OS:** "Process A's stack is at virtual addresses 0x7FFF...F000 to 0x7FFF...0000"
2. **OS:** "RSP starts at 0x7FFF...F000"
3. **Compiler:** "Variable x is at RBP - 1 (calculated at compile time)"
4. **CPU:** "When accessing RBP - 1, translate virtual address using page table"

**OS manages pages, compiler manages offsets within pages!**

## Complete Example: Accessing `x`

### Your Code
```rust
fn example() {
    let x: u8 = 42;
    let value = x;  // Access x
}
```

### What Happens (Step by Step)

1. **Compile time:**
   - Compiler: "x is at RBP - 1"
   - Generates: `mov al, [rbp - 1]`

2. **Runtime - Function call:**
   - OS sets RBP = 0x7FFF...F000 (stack start)
   - Compiler's code stores 42 at RBP - 1

3. **Runtime - Access `x`:**
   - CPU executes: `mov al, [rbp - 1]`
   - CPU calculates: Virtual address = 0x7FFF...F000 - 1 = 0x7FFF...EFFF
   - CPU splits: Page = 0x7FFF...E, Offset = 0xFFF
   - CPU looks up page table: 0x7FFF...E → 0x5000 (physical page)
   - CPU calculates: Physical address = 0x5000 + 0xFFF = 0x5FFF
   - CPU reads: 1 byte from physical RAM at 0x5FFF
   - CPU returns: 42 (stored in register AL)

4. **Done!** Value is in register.

## Key Insights

1. **OS manages pages (4KB chunks), not individual variables**
2. **Compiler calculates offsets (RBP - 1, RBP - 5) at compile time**
3. **CPU translates virtual addresses to physical using page table**
4. **Each process has isolated virtual address space**
5. **OS tracks page tables, RSP, stack/heap regions - not variables**

## Summary

| Component | Responsibility |
|-----------|----------------|
| **Compiler** | Calculates variable offsets (RBP - 1, etc.) |
| **OS** | Manages page tables, virtual address spaces |
| **CPU** | Translates virtual → physical, accesses RAM |
| **RAM** | Stores actual data (bytes) |

**The magic:** OS doesn't need to know about individual variables - compiler handles that, OS just manages pages!
