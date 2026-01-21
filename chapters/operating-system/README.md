# Operating System Concepts

How the OS manages processes, memory, and I/O: the layer between your application and hardware.

## 🎯 Learning Objectives

- Process vs thread models and their trade-offs
- Virtual memory and address space isolation
- Scheduling algorithms and CPU time management
- I/O operations and their performance impact
- Memory-mapped files and efficient data access
- System calls and kernel interactions

## 📚 Topics

### 1. Process Management
**Files:** `internals.md`
**Demo:** `cargo run --bin operating-system-concepts`

Process lifecycle, creation, and termination.

### 2. Threading Models
**Files:** `threads-processes.md`
**Demo:** `cargo run --bin operating-system-concepts`

Threads vs processes, scheduling, and synchronization.

### 3. Memory Management
**Files:** `disk-loading.md`
**Demo:** `cargo run --bin memory-management`

Virtual memory, paging, and memory protection.

## 🚀 Quick Start

```bash
# Run operating system demos
make os

# Key demo
cd code && cargo run --bin operating-system-concepts
```

## 🔑 Key Concepts

### Processes
- **Isolation**: Each process has its own memory space
- **Heavyweight**: Significant OS resources for creation
- **Security**: Cannot access other process memory directly

### Threads
- **Shared Memory**: Threads in same process share address space
- **Lightweight**: Lower overhead than processes
- **Concurrency**: Multiple execution streams in one process

### Scheduling
- **Time Slicing**: CPU time divided between threads
- **Priority Levels**: Different importance levels
- **Context Switching**: Saving/restoring thread state

### I/O Operations
- **Blocking**: Thread waits for I/O completion
- **Non-blocking**: Thread continues while I/O happens
- **Async**: Event-driven I/O completion

## 🧪 Experiments

1. **Thread Scaling**: See how many threads your CPU can handle
2. **Memory Isolation**: Observe process memory protection
3. **I/O Performance**: Compare sync vs async operations
4. **Scheduling Priority**: Effects of thread priorities

## 📖 Next Steps

Now that you understand OS fundamentals, explore [Advanced Topics](../advanced/) for practical applications and deeper systems concepts.