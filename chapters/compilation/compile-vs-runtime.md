# Compile-Time vs Runtime: What Happens Where?

## Compile-Time (When You Run `cargo build`)

### What the Compiler Does:
```
1. Parse Rust code
2. Build symbol table (in compiler's RAM):
   ┌─────────────────────┐
   │ Symbol Table:        │ ← In compiler's memory
   │   x: offset -1      │ ← Temporary, used for code generation
   │   y: offset -3      │
   │   z: offset -7      │
   └─────────────────────┘
3. Calculate all offsets
4. Generate machine code with embedded offsets
5. Write binary file to disk
```

### Resources Used:
- **CPU**: Heavy computation (parsing, type checking, optimization)
- **RAM**: Compiler's memory (symbol tables, AST, intermediate representations)
- **Time**: Can take seconds to minutes
- **Disk**: Final binary file

### What Gets Written to Binary:
```
Binary file (on disk):
┌─────────────────────────────┐
│ Machine code instructions:   │
│   mov byte [rsp-1], 5        │ ← Offset embedded here
│   mov word [rsp-3], 10       │ ← Offset embedded here
│   ...                        │
├─────────────────────────────┤
│ Data (constants, strings):   │
│   "Hello"                   │
│   [1, 2, 3, 4]              │
└─────────────────────────────┘

❌ NO symbol table in binary!
❌ NO offset tables in binary!
```

## Runtime (When You Execute the Binary)

### What's in Memory:
```
CPU Registers:
┌─────────────┐
│ RSP: 8 bytes│ ← Stack pointer (in CPU, not RAM)
└─────────────┘

RAM (loaded from binary):
┌─────────────────────────────┐
│ Instructions (from binary): │
│   [0xC6][0x44][0x24][0xFF]  │ ← Offset embedded in instruction
│   [0x05]                    │
│   ...                       │
├─────────────────────────────┤
│ Stack (grows downward):     │
│   [variable data...]        │
└─────────────────────────────┘

❌ NO symbol table!
❌ NO compiler!
```

### Resources Used:
- **CPU**: Execute instructions
- **RAM**: 
  - Instructions (loaded from binary)
  - Stack (RSP points here)
  - Heap (if you allocate)
- **Time**: Milliseconds (execution)

## The Key Difference

### Compile-Time:
- Compiler builds symbol table → **Uses RAM/CPU**
- Generates code with embedded offsets
- **Symbol table is DISCARDED** (not in binary)

### Runtime:
- **No compiler**
- **No symbol table**
- Only: Instructions (with embedded offsets) + RSP register

## Example: Compilation Process

```rust
// Source code (Rust)
fn example() {
    let x: u8 = 5;
    let y: u16 = 10;
}

// Compiler's internal symbol table (compile-time only):
Symbol Table (in compiler's RAM):
  x: type=u8, offset=-1
  y: type=u16, offset=-3

// Generated machine code (written to binary):
Assembly:
  mov byte [rsp-1], 5    ; Offset -1 embedded
  mov word [rsp-3], 10   ; Offset -3 embedded

// Binary file (on disk):
Machine code bytes:
  [0xC6][0x44][0x24][0xFF][0x05]  ; mov byte [rsp-1], 5
  [0xC7][0x44][0x24][0xFD][0x0A][0x00]  ; mov word [rsp-3], 10

// Runtime (when executed):
CPU:
  RSP register = 0x7FFF...FF00
  Execute instruction: Calculate 0x7FFF...FF00 + (-1) = 0x7FFF...FEFF
  Store byte 5 at address 0x7FFF...FEFF
```

## Summary

### Compile-Time:
- ✅ Compiler uses RAM/CPU to build symbol tables
- ✅ Calculates offsets
- ✅ Generates machine code
- ✅ Writes binary to disk
- ❌ Symbol table NOT in binary (discarded)

### Runtime:
- ✅ RSP register (8 bytes, in CPU)
- ✅ Instructions with embedded offsets (1-4 bytes each)
- ❌ No symbol table
- ❌ No compiler

## Memory Efficiency

**Compile-time** (one-time cost):
- Compiler's RAM: Can be hundreds of MB
- Symbol tables: Temporary, discarded after compilation

**Runtime** (per execution):
- RSP: 8 bytes (CPU register, not RAM)
- Instructions: ~1-4 bytes per variable access
- **No per-variable storage for addresses!**
