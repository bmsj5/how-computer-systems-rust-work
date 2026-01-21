# How Compilers Work: The "Chicken-Egg" Problem Solved

## The Key Insight: Compiler is Just a Program

The compiler is **not magic** — it's a regular program that:
- Runs on your CPU
- Uses normal RAM and stack
- Stores symbol tables as data structures (HashMaps, Vecs, etc.)
- Was written by humans (in Rust, C++, or assembly)

## What is the Compiler?

```
Compiler = Regular Program (like your client.rs)

Your Program:
┌─────────────────────┐
│ client.rs           │ ← Your code
│   - Uses RAM        │
│   - Uses stack      │
│   - Has variables   │
└─────────────────────┘

Compiler Program:
┌─────────────────────┐
│ rustc (compiler)    │ ← Also a program!
│   - Uses RAM        │ ← Same RAM as your program
│   - Uses stack      │ ← Same stack mechanism
│   - Has variables   │ ← Symbol tables are just variables!
└─────────────────────┘
```

## Symbol Table: Just a Data Structure

The compiler's symbol table is **not special** — it's just a HashMap or similar data structure:

```rust
// Simplified: What the compiler does internally
struct Symbol {
    name: String,
    offset: i32,
    type_info: Type,
}

// Compiler's symbol table (in compiler's RAM):
let mut symbol_table: HashMap<String, Symbol> = HashMap::new();

symbol_table.insert("x".to_string(), Symbol {
    name: "x".to_string(),
    offset: -1,
    type_info: Type::U8,
});

symbol_table.insert("y".to_string(), Symbol {
    name: "y".to_string(),
    offset: -3,
    type_info: Type::U16,
});
```

**This is stored in normal RAM, using normal stack/heap!**

## How the Compiler Works (Simplified)

```
1. Compiler starts (like any program):
   - OS loads rustc binary into memory
   - rustc gets its own stack (like your program)
   - rustc allocates heap memory for symbol tables

2. Compiler reads your source code:
   - Reads client.rs from disk
   - Parses it into Abstract Syntax Tree (AST)

3. Compiler builds symbol table:
   - Creates HashMap in heap memory
   - Calculates offsets
   - Stores in symbol_table HashMap

4. Compiler generates code:
   - Uses symbol_table to generate machine code
   - Writes binary to disk

5. Compiler exits:
   - Symbol table is freed (garbage collected)
   - Only the binary remains
```

## The "Chicken-Egg" Problem: Bootstrapping

### Question: Who wrote the compiler?

**Answer: Bootstrapping** — building compilers in stages:

### Stage 1: First Compiler (Assembly)
```
Assembly Language (lowest level):
┌─────────────────────┐
│ mov eax, 5          │ ← Written by hand
│ add eax, 10         │ ← Direct CPU instructions
└─────────────────────┘
```
- Written by humans in **assembly language**
- Assembler converts assembly → machine code
- Assembler was written in... assembly (or machine code directly)

### Stage 2: C Compiler (in Assembly)
```
C Compiler (written in Assembly):
┌─────────────────────┐
│ // C compiler code  │ ← Written in assembly
│ // Can compile C    │
└─────────────────────┘
```
- First C compiler written in assembly
- Once it works, you can write C compiler in C!

### Stage 3: Rust Compiler (in Rust)
```
Rust Compiler (written in Rust):
┌─────────────────────┐
│ // rustc code       │ ← Written in Rust
│ // Can compile Rust │
└─────────────────────┘
```
- First Rust compiler (rustc 0.1) written in **OCaml**
- Once it worked, rustc was rewritten in Rust (self-hosting)
- Now rustc compiles itself!

## Rust Compiler History

```
2010: rustc 0.1 (written in OCaml)
  ↓
2011: rustc 0.2 (can compile basic Rust)
  ↓
2012: rustc 0.3 (rewritten in Rust!)
  ↓
2015: rustc 1.0 (fully self-hosting)
  ↓
2024: rustc 1.80+ (still written in Rust, compiles itself)
```

## How rustc Compiles Itself

```
Step 1: You have rustc 1.79 (old version)
┌─────────────────────┐
│ rustc 1.79          │ ← Already compiled binary
│ (can compile Rust)  │
└─────────────────────┘

Step 2: Compile rustc 1.80 source code
┌─────────────────────┐
│ rustc 1.80 source   │ ← New version source code
│ (Rust code)         │
└─────────────────────┘
         ↓
    rustc 1.79 compiles it
         ↓
┌─────────────────────┐
│ rustc 1.80 binary  │ ← New compiler!
└─────────────────────┘

Step 3: rustc 1.80 can now compile itself
(and any Rust code)
```

## Compiler's Memory Usage

### When rustc runs:
```
Your System RAM:
┌─────────────────────────────────┐
│ rustc process:                  │
│   Stack:                        │
│     - Function call frames      │
│     - Local variables           │
│                                 │
│   Heap:                         │
│     - Symbol tables (HashMap)  │ ← Just data structures!
│     - AST (tree structures)     │
│     - Type information          │
│     - Generated code            │
│                                 │
│   Total: 500MB - 2GB           │
└─────────────────────────────────┘
```

**It's just a program using normal memory!**

## Example: Compiler's Internal Code

```rust
// Simplified: What rustc does internally
fn compile_function(func: &Function) {
    // Create symbol table (just a HashMap!)
    let mut symbols: HashMap<String, i32> = HashMap::new();
    
    // Calculate offsets
    let mut offset = 0;
    for var in &func.variables {
        offset -= var.size;
        symbols.insert(var.name.clone(), offset);
    }
    
    // Generate code using symbol table
    for instruction in &func.instructions {
        match instruction {
            StoreVar(name, value) => {
                let offset = symbols[name];  // Look up offset
                emit_code(format!("mov [rsp{}], {}", offset, value));
            }
            // ...
        }
    }
}
```

**This code runs on your CPU, using normal RAM!**

## The Answer to Your Question

### "Does compiler have its own stack?"

**No!** The compiler uses the **same stack mechanism** as your program:
- OS allocates stack for rustc process (like any program)
- rustc's functions use stack frames (like your functions)
- Symbol tables are stored in **heap** (as data structures)

### "Who wrote the compiler?"

**Humans, in stages:**
1. First compiler: Written in assembly (by hand)
2. Later compilers: Written in earlier languages
3. Modern compilers: Self-hosting (written in themselves)

### "Chicken-egg situation?"

**Solved by bootstrapping:**
- Start with assembly/another language
- Build compiler that can compile language X
- Rewrite compiler in language X
- Now compiler compiles itself!

## Summary

1. **Compiler is a regular program** (uses normal RAM/stack)
2. **Symbol tables are data structures** (HashMaps, Vecs, etc.)
3. **Bootstrapping**: First compiler in assembly, then self-hosting
4. **No magic**: Just programs compiling programs!

The compiler doesn't have "special" memory — it's just a program that:
- Runs on CPU
- Uses RAM/stack
- Stores symbol tables as variables
- Generates machine code
- Exits (symbol tables freed)
