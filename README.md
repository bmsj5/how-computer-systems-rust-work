# Computer Systems Through Rust 🦀

**Understanding how computers work from the ground up**

This repository provides a comprehensive, hands-on exploration of computer systems concepts using Rust as the teaching language. It bridges the gap between theoretical computer science and practical implementation, showing you exactly how hardware, memory, operating systems, and compilers interact to make your code run.

## 🎯 What You'll Learn

This repository covers the **full stack** of computer systems:

- **Hardware Layer**: CPU architecture, registers, cache systems, memory hierarchy
- **Memory Management**: Virtual memory, segmentation, paging, address translation
- **Compilation Pipeline**: How source code becomes machine code
- **Operating Systems**: Processes, threads, scheduling, I/O
- **Rust Language Features**: Memory safety, ownership, zero-cost abstractions
- **Performance Optimization**: Compiler optimizations, profiling, benchmarking

## 📚 Learning Path

Follow this structured path for the best learning experience:

### Phase 1: Hardware Foundations
1. [CPU Registers & Cache](./chapters/hardware/registers-cache.md)
2. [Machine Word & Cache](./chapters/hardware/machine-word-cache.md)
3. [Cache Line Size](./chapters/hardware/cache-line-size.md)
4. [Register Size & Word Size](./chapters/hardware/register-size.md)
5. [Hardware Threads vs Cores](./chapters/hardware/threads-cores.md)
6. [Spatial Locality & Prefetching](./chapters/hardware/spatial-locality.md)

### Phase 2: Memory Management
1. [Memory Access Walkthrough](./chapters/memory/memory-access-walkthrough.md)
2. [Memory Protection & Data Sizes](./chapters/memory/memory-protection-sizes.md)
3. [Address Visualization](./chapters/memory/address-visualization.md)
4. [Address Embedding](./chapters/memory/address-embedding.md)
5. [Array Indexing & usize](./chapters/memory/array-indexing.md)
6. [Segmentation](./chapters/memory/segmentation.md)
7. [Stack Growth & Page Tables](./chapters/memory/stack-growth.md)
8. [Stack Size Requirements](./chapters/memory/stack-size.md)
9. [Prefetching & Swapping](./chapters/memory/prefetching-swapping.md)
10. [Swap File Why Needed](./chapters/memory/swap-file.md)

### Phase 3: Compilation & Optimization
1. [Compilation Pipeline](./chapters/compilation/pipeline.md)
2. [Compiler Internals](./chapters/compilation/internals.md)
3. [Compile vs Runtime](./chapters/compilation/compile-vs-runtime.md)
4. [Cross Compilation & Optimization](./chapters/compilation/cross-compilation.md)
5. [Optimization Deep Dive](./chapters/compilation/optimization-deep-dive.md)
6. [Optimization Detailed](./chapters/compilation/optimization-detailed.md)

### Phase 4: Operating Systems
1. [OS Internals Detailed](./chapters/operating-system/internals.md)
2. [Threads vs Processes](./chapters/operating-system/threads-processes.md)
3. [Disk Loading & Page Splitting](./chapters/operating-system/disk-loading.md)

### Phase 5: Rust Systems Programming
1. [Iterator Collect Explanation](./chapters/rust-features/iterator-collect.md)
2. [Raw Pointers & Safety](./chapters/rust-features/raw-pointers-safety.md)
3. [Rc vs Immutable References](./chapters/rust-features/rc-immutable-refs.md)
4. [RefCell vs Mutable References](./chapters/rust-features/refcell-mutable-refs.md)
5. [Smart Pointers Guide](./chapters/rust-features/smart-pointers.md)
6. [Static vs Struct](./chapters/rust-features/static-struct.md)
7. [Error Handling Guide](./chapters/rust-features/error-handling.md)
8. [Result Return Guide](./chapters/rust-features/result-guide.md)
9. [Bit Masking & Powers of 2](./chapters/rust-features/bit-masking.md)
10. [Bit Operations & Hexadecimal](./chapters/rust-features/bit-operations.md)

### Phase 6: Advanced Topics
1. [LRU Implementation](./chapters/advanced/lru-implementation.md)
2. [Game Cheats & Memory](./chapters/advanced/game-cheats-memory.md)

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ installed
- Basic programming knowledge
- Curiosity about how computers work

### Quick Start
```bash
# Clone this repository
git clone https://github.com/bmsj5/computer-systems-rust.git
cd computer-systems-rust

# Run the main educational demos
cd code
cargo run --bin hardware-fundamentals
cargo run --bin memory-management
cargo run --bin compilation-optimization

# Or run individual demos
cargo run --bin cache-line-demo
cargo run --bin iterator-demo
cargo run --bin optimization-demo
```

## 📖 Repository Structure

```
computer-systems-rust/
├── chapters/                 # Educational content by topic
│   ├── hardware/            # CPU, registers, cache, memory hierarchy
│   ├── memory/              # Virtual memory, allocation, paging
│   ├── compilation/         # LLVM, optimizations, build pipeline
│   ├── operating-system/    # Processes, threads, I/O
│   ├── rust-features/       # Memory safety, ownership, performance
│   └── advanced/            # LRU cache, game cheats, advanced topics
├── code/                    # Rust implementation and demos
│   ├── src/
│   │   ├── bin/            # Individual demo programs
└── README.md               # This file
```

## 🎮 Interactive Demos

Each chapter includes runnable Rust code that demonstrates concepts:

- **Visualize cache line behavior** with real performance measurements
- **Explore memory allocation** with custom allocators
- **Profile compiler optimizations** with different optimization levels
- **Build your own iterator library** to understand zero-cost abstractions
- **Implement thread scheduling** algorithms

## 🎯 Key Concepts Covered

### Hardware
- Why cache lines are 64 bytes (not because of word size!)
- How CPU registers work vs memory
- Memory access patterns and spatial locality
- Hardware threads vs software threads

### Memory
- Virtual memory translation
- Stack vs heap allocation
- Memory protection and segmentation
- Page tables and TLB

### Compilation
- LLVM optimization passes
- Target CPU architecture impact
- Debug vs release builds
- Link-time optimization

### Systems
- Process scheduling algorithms
- Thread synchronization
- I/O multiplexing
- Memory-mapped files

## 🛠️ Development

### Running All Demos
```bash
# Run all educational demos in sequence
make run-all

# Run with profiling
make profile

# Run benchmarks
make bench
```

### Contributing
Contributions should:
- Follow the existing structure
- Update the learning path

**Happy learning!** The goal is to demystify computer systems and show how Rust's design choices map directly to hardware realities.