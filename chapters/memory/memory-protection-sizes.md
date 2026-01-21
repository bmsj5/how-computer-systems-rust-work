# Memory Protection and Data Sizes

## Question 5: Can Processes Access Each Other's Data?

**Your question:** "Can I order CPU to somehow access the neighbor's data? I heard that even program's threads have their own stack, but I also heard that somehow, processes can share stack, access each other's data. Is that true? Is it possible to 'steal' or 'take a peek' on someone else's data?"

**Answer:** **No! Processes are isolated by default. But processes CAN share memory if explicitly allowed (shared memory).**

### Process Isolation

**Default behavior:**
- Each process has its own virtual address space
- Processes cannot access each other's memory
- OS enforces this via page tables

**What happens if you try:**

```rust
// Process A tries to access Process B's memory
let ptr: *const u8 = 0x7FFF...F000 as *const u8;  // Process B's stack
unsafe {
    let value = *ptr;  // Try to read
}
```

**Result:**
1. CPU: "Access 0x7FFF...F000"
2. CPU: "Check page table for Process A"
3. CPU: "Page not in Process A's page table!"
4. CPU: **Page fault!**
5. OS: "Invalid address for Process A"
6. OS: **Segmentation fault!** (kills process)

**You cannot access another process's memory!**

### Threads vs Processes

**Threads (same process):**
- Share the same virtual address space
- Share heap, code, data
- Each thread has its own stack (but in same address space)
- Can access each other's data (if you have pointer)

**Processes (different processes):**
- Separate virtual address spaces
- Separate page tables
- Cannot access each other's memory
- Isolated by OS

**Visual:**

```
Process A (2 threads):
┌─────────────────────┐
│ Thread 1 stack      │
│ Thread 2 stack       │
│ Shared heap         │ ← Both threads can access
│ Shared code         │ ← Both threads can access
└─────────────────────┘

Process B:
┌─────────────────────┐
│ Stack               │ ← Process A cannot access
│ Heap                │ ← Process A cannot access
│ Code                │ ← Process A cannot access
└─────────────────────┘
```

### Shared Memory (Explicit Sharing)

**Processes CAN share memory if explicitly allowed:**

**Linux: Shared memory segment**
```c
// Process A creates shared memory
int shm_id = shmget(KEY, SIZE, IPC_CREAT);

// Process B attaches to same shared memory
void *ptr = shmat(shm_id, NULL, 0);

// Both processes can now access same memory!
```

**How it works:**
1. OS creates shared memory segment
2. Both processes map it into their virtual address space
3. Both processes' page tables point to same physical pages
4. Changes by one process visible to other

**Use cases:**
- Inter-process communication (IPC)
- Database shared buffers
- High-performance computing

**Security:**
- Only processes with permission can access
- OS controls access (not automatic)

### Can You "Steal" Data?

**Short answer: No, not through normal means.**

**Why:**
1. **OS isolation:** Page tables prevent access
2. **Hardware protection:** CPU enforces page table checks
3. **Kernel protection:** OS kernel is separate (ring 0)

**Possible attacks (security vulnerabilities):**
1. **Buffer overflow:** Exploit bug to access adjacent memory
2. **Use-after-free:** Access freed memory
3. **Spectre/Meltdown:** CPU side-channel attacks (hardware bugs)
4. **Kernel exploits:** Gain kernel access, then access any memory

**But these are security vulnerabilities, not normal operation!**

### Summary

| Aspect | Threads | Processes |
|--------|---------|-----------|
| **Address space** | Shared | Separate |
| **Page tables** | Same | Different |
| **Can access each other?** | Yes (same address space) | No (isolated) |
| **Stack** | Separate (but same address space) | Separate (different address spaces) |
| **Shared memory** | Automatic | Must be explicitly created |

**Key insight:** Processes are isolated by default. You cannot access another process's memory through normal means. Threads share memory because they're in the same process.

## Question 6: Data Sizes (byte, word, dword, etc.)

**Your question:** "You mentioned 'dword' size, what other sizes are there? I know there are byte (1 byte), word (2 bytes)."

**Answer:** **There are several standard sizes: byte, word, dword, qword.**

### Standard Data Sizes

| Name | Size | Bits | Common Use |
|------|------|------|------------|
| **Byte** | 1 byte | 8 bits | `u8`, `i8` |
| **Word** | 2 bytes | 16 bits | `u16`, `i16` |
| **Dword** | 4 bytes | 32 bits | `u32`, `i32`, `f32` |
| **Qword** | 8 bytes | 64 bits | `u64`, `i64`, `f64` |

### Assembly Instructions

**x86-64 assembly uses these sizes:**

```asm
; Byte (1 byte)
mov al, [address]    ; Load 1 byte into AL (8-bit register)
mov byte [address], 5  ; Store 1 byte

; Word (2 bytes)
mov ax, [address]    ; Load 2 bytes into AX (16-bit register)
mov word [address], 100  ; Store 2 bytes

; Dword (4 bytes)
mov eax, [address]   ; Load 4 bytes into EAX (32-bit register)
mov dword [address], 1000  ; Store 4 bytes

; Qword (8 bytes)
mov rax, [address]   ; Load 8 bytes into RAX (64-bit register)
mov qword [address], 10000  ; Store 8 bytes
```

### Register Sizes

**x86-64 registers:**

```
RAX (64 bits = 8 bytes = qword)
├─ EAX (32 bits = 4 bytes = dword) - lower half
│  ├─ AX (16 bits = 2 bytes = word) - lower quarter
│  │  ├─ AH (8 bits = 1 byte = byte) - upper byte of AX
│  │  └─ AL (8 bits = 1 byte = byte) - lower byte of AX
```

**Usage:**
```asm
mov al, 5      ; Byte (1 byte) → AL
mov ax, 100    ; Word (2 bytes) → AX
mov eax, 1000  ; Dword (4 bytes) → EAX
mov rax, 10000 ; Qword (8 bytes) → RAX
```

### Rust Types

**Rust types map to these sizes:**

```rust
// Byte (1 byte)
let x: u8 = 5;   // Unsigned byte
let y: i8 = -5;  // Signed byte

// Word (2 bytes)
let x: u16 = 100;   // Unsigned word
let y: i16 = -100;  // Signed word

// Dword (4 bytes)
let x: u32 = 1000;    // Unsigned dword
let y: i32 = -1000;   // Signed dword
let z: f32 = 3.14;    // Float (dword)

// Qword (8 bytes)
let x: u64 = 10000;   // Unsigned qword
let y: i64 = -10000;  // Signed qword
let z: f64 = 3.14;    // Double (qword)
let ptr: usize = 0x1000;  // Pointer (qword on 64-bit)
```

### Why These Names?

**Historical reasons:**
- **Byte:** Smallest addressable unit (8 bits)
- **Word:** Natural size for early CPUs (16 bits on 8086)
- **Dword:** Double word (2 × 16 bits = 32 bits)
- **Qword:** Quad word (4 × 16 bits = 64 bits)

**Modern usage:**
- Still used in assembly
- Still used in documentation
- Helps understand data sizes

### Summary

| Size | Bytes | Bits | Assembly | Rust |
|------|-------|------|----------|------|
| **Byte** | 1 | 8 | `mov al, [...]` | `u8`, `i8` |
| **Word** | 2 | 16 | `mov ax, [...]` | `u16`, `i16` |
| **Dword** | 4 | 32 | `mov eax, [...]` | `u32`, `i32`, `f32` |
| **Qword** | 8 | 64 | `mov rax, [...]` | `u64`, `i64`, `f64`, `usize` |

**Key insight:** These are standard data sizes used in assembly and correspond to Rust types. Understanding them helps with low-level programming and understanding how CPUs work.
