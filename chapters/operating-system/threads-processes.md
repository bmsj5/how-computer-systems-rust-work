# Threads vs Processes: Memory Model and Differences

## Quick Answer

**Process:** Isolated virtual address space (separate memory)
**Thread:** Shares virtual address space with other threads in same process (shared memory)

---

## 1. What is a Process?

**Process = Program + Memory + Resources**

Each process has:
- **Own virtual address space** (isolated from other processes)
- **Own stack** (grows downward from high address)
- **Own heap** (grows upward from low address)
- **Own code section** (program instructions)
- **Own data section** (global variables)
- **Own file descriptors, network sockets, etc.**

**Memory isolation:**
```
Process A Virtual Space:        Process B Virtual Space:
┌─────────────────────┐         ┌─────────────────────┐
│ Stack (0x7FFF...)   │         │ Stack (0x7FFF...)   │
│ Heap (0x1000...)    │         │ Heap (0x1000...)    │
│ Code (0x400000)     │         │ Code (0x400000)     │
└─────────────────────┘         └─────────────────────┘
        ↓                               ↓
   Physical RAM                   Physical RAM
   (Different pages)              (Different pages)
```

**Key point:** Process A **cannot** access Process B's memory (OS enforces this via page tables).

---

## 2. What is a Thread?

**Thread = Execution path within a process**

Each thread has:
- **Own stack** (separate stack pointer RSP)
- **Own registers** (RAX, RBX, RCX, etc.)
- **Own instruction pointer** (RIP - where it's executing)
- **Shared heap** (all threads in process share same heap)
- **Shared code section** (same program code)
- **Shared data section** (same global variables)

**Memory sharing:**
```
Single Process Virtual Space:
┌─────────────────────────────────────┐
│ Stack Thread 1 (0x7FFF...F000)      │ ← Thread 1's stack
│ Stack Thread 2 (0x7FFF...E000)      │ ← Thread 2's stack
│ Stack Thread 3 (0x7FFF...D000)      │ ← Thread 3's stack
│                                     │
│ ─────────────────────────────────── │
│                                     │
│ Shared Heap (0x1000...)             │ ← All threads share
│ Shared Code (0x400000)               │ ← All threads share
│ Shared Data (globals)                │ ← All threads share
└─────────────────────────────────────┘
```

**Key point:** Threads in same process **can** access each other's heap/data (shared memory).

---

## 3. Process vs Thread: Detailed Comparison

### Memory Model

| Aspect | Process | Thread |
|--------|---------|--------|
| **Virtual Address Space** | Own (isolated) | Shared (same process) |
| **Stack** | Own | Own (separate stack per thread) |
| **Heap** | Own | Shared (all threads share) |
| **Code Section** | Own | Shared (same code) |
| **Global Variables** | Own | Shared (same globals) |
| **Memory Isolation** | ✅ Yes (OS enforced) | ❌ No (can access shared memory) |

### Creation Cost

| Aspect | Process | Thread |
|--------|---------|--------|
| **Creation Time** | Slow (~1-10ms) | Fast (~0.1ms) |
| **Memory Overhead** | High (separate address space) | Low (just new stack) |
| **Context Switch** | Slow (switch page tables) | Fast (just switch registers) |

### Communication

| Aspect | Process | Thread |
|--------|---------|--------|
| **Shared Memory** | ❌ No (isolated) | ✅ Yes (same heap) |
| **Communication** | IPC (pipes, sockets, shared memory) | Direct memory access |
| **Synchronization** | OS-level (semaphores, mutexes) | Language-level (Mutex, RwLock) |

---

## 4. How Threading Works: Memory Layout

### Example: 3 Threads in One Process

```
Process Virtual Address Space (0x0000...0000 to 0xFFFF...FFFF):

High Address (0x7FFF...FFFF)
┌─────────────────────────────────────┐
│ Guard Page                           │
│ ─────────────────────────────────── │
│ Stack Thread 1 (RSP₁)                │ ← Thread 1 executing
│   - Local variables                 │
│   - Function calls                  │
│ ─────────────────────────────────── │
│ Stack Thread 2 (RSP₂)                │ ← Thread 2 executing
│   - Local variables                 │
│   - Function calls                  │
│ ─────────────────────────────────── │
│ Stack Thread 3 (RSP₃)                │ ← Thread 3 executing
│   - Local variables                 │
│   - Function calls                  │
│ ─────────────────────────────────── │
│                                     │
│ Shared Heap                          │ ← All threads access
│   - Vec, String, Box, etc.          │
│   - Arc<Mutex<T>>                   │
│ ─────────────────────────────────── │
│ Code Section                         │ ← All threads execute
│   - Function definitions            │
│ ─────────────────────────────────── │
│ Data Section (globals)               │ ← All threads access
│   - static GLOBAL: i32 = 42         │
Low Address (0x0000...0000)
└─────────────────────────────────────┘
```

### Thread-Specific Details

**Each thread has:**
1. **Own RSP (Stack Pointer):** Points to thread's stack
2. **Own RBP (Base Pointer):** Points to current stack frame
3. **Own RIP (Instruction Pointer):** Where thread is executing
4. **Own Registers:** RAX, RBX, RCX, RDX, etc. (saved/restored on context switch)

**Shared across threads:**
1. **Heap:** All threads allocate/free from same heap
2. **Code:** All threads execute same program code
3. **Globals:** All threads see same global variables

---

## 5. Rust Example: Threads Sharing Memory

```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    // Shared data on heap (all threads can access)
    let shared_data = Arc::new(Mutex::new(vec![1, 2, 3]));
    
    // Thread 1: Own stack, shares heap
    let data1 = Arc::clone(&shared_data);
    let thread1 = thread::spawn(move || {
        // This closure runs on Thread 1's stack
        let mut vec = data1.lock().unwrap();
        vec.push(4);  // Modifies shared heap data
    });
    
    // Thread 2: Own stack, shares heap
    let data2 = Arc::clone(&shared_data);
    let thread2 = thread::spawn(move || {
        // This closure runs on Thread 2's stack
        let mut vec = data2.lock().unwrap();
        vec.push(5);  // Modifies shared heap data
    });
    
    thread1.join().unwrap();
    thread2.join().unwrap();
    
    // Main thread: Can see changes from other threads
    println!("{:?}", shared_data.lock().unwrap());  // [1, 2, 3, 4, 5]
}
```

**Memory layout:**
```
Main Thread Stack:              Thread 1 Stack:              Thread 2 Stack:
┌─────────────────┐            ┌─────────────────┐      ┌─────────────────┐
│ shared_data     │            │ data1          │      │ data2            │
│ (Arc pointer)   │            │ (Arc pointer) │      │ (Arc pointer)    │
│                 │            │                │      │                  │
│ RSP_main        │            │ RSP₁           │      │ RSP₂             │
└─────────────────┘            └─────────────────┘      └─────────────────┘
        │                              │                        │
        └──────────────┬───────────────┴────────────────────────┘
                       │
                       ↓
              Shared Heap:
              ┌─────────────────┐
              │ Arc metadata    │
              │ Mutex metadata  │
              │ Vec [1,2,3,4,5] │ ← All threads point here
              └─────────────────┘
```

---

## 6. Thread Safety: Why Mutex/Arc?

**Problem:** Multiple threads accessing same memory → **data races**

**Example (unsafe):**
```rust
let mut counter = 0;  // Shared on heap

thread::spawn(|| {
    counter += 1;  // ❌ Data race! (unsafe)
});

thread::spawn(|| {
    counter += 1;  // ❌ Data race! (unsafe)
});
```

**Solution: Synchronization primitives**

### Arc<Mutex<T>> - Shared Mutable (Thread-Safe)
```rust
let counter = Arc::new(Mutex::new(0));

let c1 = Arc::clone(&counter);
thread::spawn(move || {
    let mut num = c1.lock().unwrap();
    *num += 1;  // ✅ Safe (mutex protects)
});

let c2 = Arc::clone(&counter);
thread::spawn(move || {
    let mut num = c2.lock().unwrap();
    *num += 1;  // ✅ Safe (mutex protects)
});
```

**How it works:**
1. **Arc:** Allows multiple threads to own same data (reference counting)
2. **Mutex:** Ensures only one thread can mutate at a time (lock/unlock)

---

## 7. Context Switching: How OS Switches Between Threads

**Context switch = Save current thread state, load another thread state**

**Important:** On a **single-core CPU**, only **one thread executes at a time** (same as processes). The OS scheduler switches between threads rapidly, creating the illusion of parallel execution.

**What gets saved:**
1. **Registers:** RAX, RBX, RCX, RDX, RSP, RBP, RIP, etc.
2. **Stack pointer:** RSP (where thread's stack is)
3. **Instruction pointer:** RIP (where thread was executing)

### Single-Core CPU: Time-Slicing

**On single-core, the OS scheduler switches between threads:**
```
Time slice 1: Process A, Thread 1 executing
  ↓ (context switch)
Time slice 2: Process B, Thread 1 executing
  ↓ (context switch)
Time slice 3: Process B, Thread 2 executing (same process!)
  ↓ (context switch)
Time slice 4: Process A, Thread 1 executing
  ↓ (context switch)
...
```

**Key point:** The OS scheduler can switch between **any thread from any process**. It's not limited to threads within the same process.

### Process Context Switch (Slow)

**Switching between threads from different processes:**
```
Process A, Thread 1 → Process B, Thread 1:
1. Save Process A Thread 1's registers
2. Switch page table (virtual → physical mapping) ← Expensive!
3. Load Process B Thread 1's registers
4. Resume Process B Thread 1
Time: ~1-10 microseconds
```

### Thread Context Switch (Fast)

**Switching between threads in the same process:**
```
Process B, Thread 1 → Process B, Thread 2:
1. Save Thread 1's registers
2. Load Thread 2's registers
3. Resume Thread 2
Time: ~0.1-1 microseconds
(No page table switch - same process!)
```

**Why faster?** Same process = same page table = no need to switch virtual memory mapping.

### Multi-Core CPU: True Parallelism

**On multi-core, multiple threads can execute simultaneously:**
```
Core 1: Process A, Thread 1 executing
Core 2: Process B, Thread 1 executing
Core 3: Process B, Thread 2 executing
Core 4: Process C, Thread 1 executing
```

**All executing at the same time** (true parallelism, not time-slicing).

---

## 8. When to Use Processes vs Threads

### Use Processes When:
- ✅ **Isolation needed** (crash in one doesn't kill others)
- ✅ **Security** (separate address spaces)
- ✅ **Different programs** (separate executables)
- ✅ **OS-level parallelism** (multiple CPUs, separate memory)

**Example:** Web browser (each tab = separate process)

### Use Threads When:
- ✅ **Shared memory** (need to share data easily)
- ✅ **Fast communication** (direct memory access)
- ✅ **Same program** (parallel execution of same code)
- ✅ **Lightweight** (low overhead)

**Example:** Web server (each request = thread, shares connection pool)

---

## 9. Rust Threading Model

**Rust's approach:**
1. **Ownership rules** prevent data races at compile time
2. **Send + Sync traits** ensure thread safety
3. **Arc + Mutex** for shared mutable data
4. **Channels** for message passing (alternative to shared memory)

**Example with channels:**
```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send(42).unwrap();  // Send data to main thread
});

let received = rx.recv().unwrap();  // Receive data
println!("{}", received);
```

---

## 10. Key Takeaways

1. **Process:** Isolated virtual address space (separate memory)
2. **Thread:** Shares virtual address space (shared memory)
3. **Stack:** Each thread has own stack (separate RSP)
4. **Heap:** Threads share heap (same process)
5. **Context switch:** Threads faster (no page table switch)
6. **Thread safety:** Use `Arc<Mutex<T>>` for shared mutable data
7. **Rust:** Ownership prevents data races, `Send`/`Sync` ensure thread safety

---

## 11. Memory Safety: Why Rust's Ownership Matters

**Without ownership (C/C++):**
```c
int* data = malloc(sizeof(int));
*data = 42;

thread1: free(data);      // Thread 1 frees
thread2: *data = 100;      // ❌ Use after free! (undefined behavior)
```

**With ownership (Rust):**
```rust
let data = Arc::new(42);

let d1 = Arc::clone(&data);
thread::spawn(move || {
    // d1 owns reference, data stays alive
    println!("{}", d1);
});

// ✅ Safe: Arc ensures data lives until last reference drops
```

**Rust's guarantees:**
- ✅ **No data races** (compile-time checked)
- ✅ **No use-after-free** (ownership rules)
- ✅ **No double-free** (single owner or Arc reference counting)

---

## 12. Are Threads Just an Abstraction/Hack?

**Yes! Threads are essentially an OS-level abstraction that gives you the best of both worlds:**

### What Threads Provide (Like Separate Processes):
1. ✅ **Separate execution contexts** (own stack, registers, instruction pointer)
2. ✅ **Parallel execution** on multi-core CPUs (can run simultaneously)
3. ✅ **Independent control flow** (each thread executes different code paths)

### What Threads Share (Unlike Separate Processes):
1. ✅ **Same virtual address space** (no page table switch needed)
2. ✅ **Shared heap** (easy data sharing, no IPC overhead)
3. ✅ **Shared code section** (same program code)
4. ✅ **Shared globals** (same global variables)

### The "Hack" / Optimization:

**Threads are essentially:**
- **Lightweight processes** that share memory space
- **Execution units** within a process (not separate processes)
- **OS abstraction** to avoid expensive page table switches

**Why it's efficient:**
```
Process context switch:
  - Save registers (fast)
  - Switch page table (slow - invalidate TLB, reload mappings)
  - Load registers (fast)
  Total: ~1-10 microseconds

Thread context switch (same process):
  - Save registers (fast)
  - Load registers (fast)
  Total: ~0.1-1 microseconds
  (No page table switch - same virtual address space!)
```

### The Abstraction Layer:

**From the OS perspective:**
- **Process:** Unit of resource isolation (memory, files, network)
- **Thread:** Unit of execution within a process (CPU scheduling)

**From the CPU perspective:**
- CPU doesn't know about "processes" or "threads"
- CPU just executes instructions from different memory addresses
- OS scheduler decides which thread gets CPU time

**From your program's perspective:**
- Threads look like separate execution paths
- They can access shared memory directly (no IPC)
- They can run in parallel on multi-core

### Is It Really Just an Abstraction?

**Yes, but a very useful one:**

1. **Hardware reality:** CPU cores execute instructions. They don't care about "processes" or "threads"
2. **OS abstraction:** OS creates the illusion of processes/threads by:
   - Managing virtual memory (page tables)
   - Scheduling CPU time (context switching)
   - Tracking resources (file descriptors, etc.)
3. **Your program's view:** Threads appear as separate execution units that can share memory

**The "hack" is:**
- Instead of creating a whole new process (expensive: new page table, isolated memory)
- Create a lightweight execution context (cheap: just new stack + registers)
- Share the same virtual address space (no page table switch needed)
- Get parallel execution on multi-core (CPU doesn't care - it just executes instructions)

### Why Not Just Use Processes?

**You could, but threads are more efficient for:**
- **Shared data:** Threads can directly access shared heap (no IPC overhead)
- **Fast communication:** Direct memory access vs pipes/sockets
- **Lower overhead:** No page table switch, less memory per thread
- **Easier programming:** Shared memory is simpler than IPC

**Use processes when:**
- **Isolation needed:** Crash in one shouldn't kill others
- **Security:** Separate address spaces prevent memory access bugs
- **Different programs:** Separate executables

### Summary: The Abstraction

**Threads = OS abstraction that provides:**
- ✅ Separate execution contexts (like processes)
- ✅ Shared memory space (unlike processes)
- ✅ Fast context switching (no page table switch)
- ✅ Parallel execution on multi-core (like processes)
- ✅ Direct memory sharing (unlike processes)

**It's not "hacky" - it's a well-designed abstraction that gives you efficiency benefits while maintaining the illusion of separate execution units.**
