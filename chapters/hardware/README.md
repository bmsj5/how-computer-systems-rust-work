# Hardware Fundamentals

This chapter explores the physical components that make your code run: CPUs, registers, cache systems, and memory hierarchy.

## 🎯 Learning Objectives

By the end of this chapter, you'll understand:
- How CPU registers work and why they're crucial for performance
- Why cache lines are 64 bytes (it's not about word size!)
- The difference between hardware threads and software threads
- How spatial locality affects performance
- Memory access patterns and their impact on speed

## 📚 Topics

### 1. CPU Architecture & Registers
**Files:** `registers-cache.md`, `register-size.md`
**Demo:** `cargo run --bin register-demo`

Understanding the CPU's working memory and how it differs from RAM.

### 2. Memory Hierarchy & Cache Systems
**Files:** `cache-line-size.md`, `machine-word-cache.md`
**Demo:** `cargo run --bin cache-line-demo`

Why memory access isn't uniform and how cache systems optimize performance.

### 3. Hardware Concurrency
**Files:** `threads-cores.md`
**Demo:** `cargo run --bin hardware-fundamentals`

The difference between physical cores and logical processors.

### 4. Memory Access Patterns
**Files:** `spatial-locality.md`
**Demo:** `cargo run --bin memory-access-demo`

How data layout affects performance through prefetching and cache utilization.

## 🚀 Quick Start

```bash
# Run all hardware demos
make hardware

# Or run individually
cd code && cargo run --bin cache-line-demo
cd code && cargo run --bin register-demo
cd code && cargo run --bin hardware-fundamentals
```

## 🔑 Key Concepts

### Registers vs Memory
- **Registers**: CPU's ultra-fast internal storage (nanoseconds)
- **Cache**: Fast but limited memory hierarchy
- **RAM**: Slow but large main memory (hundreds of cycles)

### Cache Line Size
- **64 bytes**: Not determined by word size
- **Purpose**: Balance transfer efficiency vs. cache pollution
- **Impact**: Affects struct layout and array access patterns

### Hardware Threads
- **Physical cores**: Actual processing units
- **Hardware threads**: CPU's ability to run multiple instruction streams
- **Hyperthreading**: Intel's implementation of simultaneous multithreading

## 🧪 Experiments

Try these experiments to see hardware concepts in action:

1. **Cache Line Experiment**: Modify struct fields and measure access time differences
2. **Register Usage**: Compare register-heavy vs memory-heavy algorithms
3. **Thread Scaling**: See how hyperthreading affects parallel workloads

## 📖 Further Reading

- Computer Organization and Design (Patterson & Hennessy)
- What Every Programmer Should Know About Memory (Ulrich Drepper)
- Intel/AMD CPU architecture documentation

## 🎮 Next Steps

Once you understand hardware basics, move to [Memory Management](../memory/) to see how software interacts with this hardware.