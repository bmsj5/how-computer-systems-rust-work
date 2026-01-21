# Compilation & Optimization

How your Rust code becomes machine instructions: LLVM, optimization passes, and performance tuning.

## 🎯 Learning Objectives

- The complete compilation pipeline from source to binary
- LLVM optimization passes and their impact
- Target CPU architecture effects on performance
- Debug vs release build differences
- Profiling and optimization techniques

## 📚 Topics

### 1. Compilation Pipeline
**Files:** `pipeline.md`, `internals.md`
**Demo:** `cargo run --bin compilation-optimization`

From `fn main()` to machine code.

### 2. Optimization Deep Dive
**Files:** `optimization-deep-dive.md`, `optimization-detailed.md`
**Demo:** `cargo run --bin optimization-demo`

How LLVM makes your code faster.

### 3. Build Configuration
**Files:** `compile-vs-runtime.md`, `cross-compilation.md`
**Demo:** `cargo run --bin optimization-levels-demo`

Different compilation modes and their effects.

## 🚀 Quick Start

```bash
# Run compilation demos
make compilation

# Compare optimization levels
cd code && cargo run --bin optimization-levels-demo

# See LLVM IR generation
cd code && cargo rustc --release -- --emit=llvm-ir
```

## 🔑 Key Concepts

### Compilation Phases
1. **Parsing**: Source code → AST
2. **HIR/MIR**: Rust-specific intermediate representations
3. **LLVM IR**: Target-independent optimization
4. **Machine Code**: CPU-specific instructions

### Optimization Levels
- **Debug (O0)**: No optimizations, fast compilation
- **Release (O3)**: Aggressive optimizations, slower compilation
- **Size (Oz)**: Optimize for binary size

### Target Architecture
- **Generic**: Works everywhere, conservative optimizations
- **Specific CPU**: Uses advanced instructions (AVX, SIMD)
- **Performance Gain**: 2-3x faster for numerical code

## 🧪 Experiments

1. **Optimization Comparison**: Time the same code at different optimization levels
2. **Assembly Inspection**: See how Rust code compiles to machine instructions
3. **Cross-Compilation**: Build for different architectures

## 📖 Next Steps

Now that you understand compilation, explore [Operating System Concepts](../operating-system/) to see how your programs interact with the OS.