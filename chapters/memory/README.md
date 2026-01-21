# Memory Management

Understanding how programs use memory: from virtual addresses to physical RAM, stack vs heap, and memory protection.

## 🎯 Learning Objectives

- Virtual memory translation and address spaces
- Stack vs heap allocation strategies
- Memory protection and segmentation
- Page tables and address resolution
- Memory access patterns and performance

## 📚 Topics

### 1. Virtual Memory Fundamentals
**Files:** `address-visualization.md`, `address-embedding.md`
**Demo:** `cargo run --bin memory-access-demo`

How virtual addresses map to physical memory.

### 2. Stack vs Heap
**Files:** `stack-growth.md`, `stack-size.md`
**Demo:** `cargo run --bin memory-management`

Where variables live and why it matters.

### 3. Memory Protection
**Files:** `memory-protection-sizes.md`, `segmentation.md`
**Demo:** `cargo run --bin pointer-safety-demo`

How the OS protects your program's memory.

### 4. Advanced Memory Concepts
**Files:** `prefetching-swapping.md`, `swap-file.md`, `array-indexing.md`
**Demo:** `cargo run --bin array-indexing-demo`

Memory optimization and system-level memory management.

## 🚀 Quick Start

```bash
# Run memory management demos
make memory

# Key demos
cd code && cargo run --bin memory-access-demo
cd code && cargo run --bin array-indexing-demo
```

## 🔑 Key Concepts

### Virtual Memory
- **Address Space**: Each process has its own 64-bit address space
- **Translation**: Virtual → Physical address mapping via page tables
- **Protection**: Prevents processes from accessing each other's memory

### Stack vs Heap
- **Stack**: Fast, automatic, limited size, LIFO allocation
- **Heap**: Slower, manual, large, complex allocation patterns

### Memory Access Costs
- **Registers**: ~1 cycle
- **Cache**: ~10 cycles
- **RAM**: ~100 cycles
- **Disk**: ~10,000,000 cycles

## 🧪 Experiments

1. **Memory Access Patterns**: Compare sequential vs random access
2. **Stack Overflow**: See what happens when stack limits are exceeded
3. **Heap Allocation**: Profile different allocation strategies

## 📖 Next Steps

After memory management, explore [Compilation & Optimization](../compilation/) to see how your code becomes machine instructions.