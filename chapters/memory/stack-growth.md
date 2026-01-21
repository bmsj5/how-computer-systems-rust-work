# Stack Growth, Page Tables, and OS Tracking

## Your Questions

1. "When stack grows and it's necessary to allocate another page, what happens to RSP? Does it reset?"
2. "If CPU manages page table translation, why does OS keep the initial RSP?"
3. "Does OS know where every program starts/ends and track it?"

## Question 1: Stack Growth and RSP

### How Stack Grows

**Initial state:**
```
Virtual Address Space:
┌─────────────────────┐
│ 0x7FFF...F000 (RSP) │ ← Stack top (page 1)
│                     │
│ 0x7FFF...E000       │ ← End of page 1
├─────────────────────┤
│ 0x7FFF...D000       │ ← Start of page 2 (not allocated yet)
│                     │
└─────────────────────┘
```

**RSP = 0x7FFF...F000** (points to top of stack, in page 1)

### When Stack Grows Beyond Page

**What happens:**

1. **Function allocates more stack space:**
```asm
sub rsp, 0x2000  ; Allocate 8KB (2 pages)
```

2. **RSP moves down:**
```
Before: RSP = 0x7FFF...F000 (page 1)
After:  RSP = 0x7FFF...D000 (page 2)
```

3. **CPU tries to access new address:**
   - CPU: "Access 0x7FFF...D000"
   - CPU: "Page 0x7FFF...D not in page table!"
   - CPU: **Page fault!** (interrupt to OS)

4. **OS handles page fault:**
   - OS: "Stack is growing, need to allocate new page"
   - OS: Allocates physical page
   - OS: Updates page table: `0x7FFF...D → 0x6000` (physical)
   - OS: Returns to program

5. **Program continues:**
   - RSP = 0x7FFF...D000 (still points to new location)
   - Stack now spans 2 pages
   - No reset! RSP just moved to new page

### RSP Does NOT Reset!

**RSP is just a register pointing to current stack top:**
- It moves down as stack grows
- It moves up as stack shrinks
- It's not tied to a specific page
- It can span multiple pages

**Example:**
```
Page 1: 0x7FFF...F000 to 0x7FFF...E000 (4KB)
Page 2: 0x7FFF...E000 to 0x7FFF...D000 (4KB)
Page 3: 0x7FFF...D000 to 0x7FFF...C000 (4KB)

RSP can be anywhere: 0x7FFF...F000, 0x7FFF...E500, 0x7FFF...D200, etc.
```

## Question 2: Why OS Keeps Initial RSP?

### What OS Tracks

**For each process, OS maintains:**

1. **Initial RSP:**
   - Where stack starts (e.g., 0x7FFF...F000)
   - Used to set RSP when process starts
   - Used to detect stack overflow (if RSP goes too far down)

2. **Stack limit:**
   - Maximum stack size (usually 2-8 MB)
   - If RSP goes below this → stack overflow → kill process

3. **Page table:**
   - Maps virtual pages → physical pages
   - Updated when stack grows (new pages allocated)

### Why OS Needs Initial RSP

**Reasons:**

1. **Process initialization:**
   - When OS starts a process, it sets RSP to initial value
   - Without this, process wouldn't know where stack is

2. **Stack overflow detection:**
   - OS checks: "Is RSP < (initial_RSP - stack_limit)?"
   - If yes → stack overflow → kill process

3. **Context switching:**
   - When OS switches between processes, it saves/restores RSP
   - Needs to know where each process's stack is

4. **Memory management:**
   - OS needs to know stack region to manage pages
   - When process exits, OS frees stack pages

**Example:**
```
Process starts:
  OS: "Set RSP = 0x7FFF...F000 (initial stack)"
  OS: "Stack limit = 0x7FFF...0000 (8 MB below)"
  
Function call:
  RSP moves down: 0x7FFF...F000 → 0x7FFF...EF00
  
Stack overflow check:
  OS: "Is 0x7FFF...EF00 < 0x7FFF...0000? No, OK"
  
If RSP goes below limit:
  OS: "Stack overflow! Kill process"
```

## Question 3: Does OS Track Where Programs Start/End?

### Yes! OS Tracks Program Memory Regions

**For each process, OS maintains:**

1. **Code segment (text):**
   - Virtual addresses: 0x400000 to 0x500000 (example)
   - Contains: Program instructions
   - Mapped from: Executable file on disk

2. **Data segment:**
   - Virtual addresses: 0x600000 to 0x700000 (example)
   - Contains: Global variables, constants
   - Mapped from: Executable file on disk

3. **Stack:**
   - Virtual addresses: 0x7FFF...F000 to 0x7FFF...0000 (example)
   - Contains: Local variables, function calls
   - Grows: Downward (toward lower addresses)

4. **Heap:**
   - Virtual addresses: 0x100000 to 0x200000 (example)
   - Contains: Dynamically allocated memory
   - Grows: Upward (toward higher addresses)

### How OS Tracks This

**Process Control Block (PCB) - OS data structure:**

```
Process A:
  Code:    0x400000 - 0x500000
  Data:    0x600000 - 0x700000
  Stack:   0x7FFF...F000 - 0x7FFF...0000
  Heap:    0x100000 - 0x200000 (grows)
  Page table: [virtual → physical mappings]
  Initial RSP: 0x7FFF...F000
  Stack limit: 0x7FFF...0000
```

**OS uses this to:**
- Allocate/deallocate pages
- Detect invalid memory access
- Manage memory efficiently
- Isolate processes

### What Happens on Memory Access

**When program accesses address 0x7FFF...E500:**

1. **CPU checks page table:**
   - Virtual address: 0x7FFF...E500
   - Page: 0x7FFF...E
   - Is page in page table? Yes → translate to physical
   - No → **Page fault** → OS handles

2. **OS checks if valid:**
   - Is 0x7FFF...E500 in stack region? Yes
   - Is it within stack limit? Yes
   - Allocate page if needed
   - Update page table
   - Return to program

3. **If invalid:**
   - Address not in any valid region → **Segmentation fault**
   - OS kills process

## Complete Example: Stack Growth

### Initial State

```
Process starts:
  OS sets RSP = 0x7FFF...F000
  OS reserves stack region: 0x7FFF...F000 to 0x7FFF...0000 (8 MB)
  OS allocates first page: 0x7FFF...F000 to 0x7FFF...E000
  Page table: 0x7FFF...F → 0x5000 (physical)
```

### Function Call Allocates Large Array

```rust
fn large_function() {
    let array = [0u8; 8192];  // 8KB array
    // ...
}
```

**What happens:**

1. **Compiler generates:**
```asm
sub rsp, 8192  ; Allocate 8KB on stack
```

2. **RSP moves:**
   - Before: 0x7FFF...F000
   - After: 0x7FFF...D000 (moved down 8KB, into page 2)

3. **CPU accesses new address:**
   - Tries to access 0x7FFF...D000
   - Page 0x7FFF...D not in page table
   - **Page fault!**

4. **OS handles:**
   - Checks: Is 0x7FFF...D000 in stack region? Yes
   - Checks: Is it within stack limit? Yes
   - Allocates physical page: 0x6000
   - Updates page table: 0x7FFF...D → 0x6000
   - Returns to program

5. **Program continues:**
   - RSP = 0x7FFF...D000 (still points to stack top)
   - Stack now spans 2 pages
   - No reset!

### Stack Now Looks Like:

```
Virtual Address Space:
┌─────────────────────┐
│ 0x7FFF...F000       │ ← Original RSP (page 1, allocated)
│ ...                 │
│ 0x7FFF...E000       │ ← End of page 1
├─────────────────────┤
│ 0x7FFF...D000 (RSP) │ ← Current RSP (page 2, just allocated)
│ ...                 │
│ 0x7FFF...C000       │ ← End of page 2
└─────────────────────┘

Page table:
  0x7FFF...F → 0x5000 (physical, page 1)
  0x7FFF...E → (not allocated yet)
  0x7FFF...D → 0x6000 (physical, page 2)
```

## Summary

| Question | Answer |
|----------|--------|
| **RSP reset on new page?** | No! RSP just moves to new page, no reset |
| **Why OS keeps initial RSP?** | Process init, stack overflow detection, context switching |
| **Does OS track program regions?** | Yes! Code, data, stack, heap - all tracked in PCB |
| **How stack grows?** | RSP moves down, OS allocates new pages on page fault |
| **Who manages what?** | OS: Pages, regions. CPU: Translation. Compiler: Offsets |

**Key insights:**
1. RSP is just a register - it moves, doesn't reset
2. OS tracks all memory regions (code, stack, heap, data)
3. OS manages page tables, CPU translates addresses
4. Stack grows via page faults - OS allocates pages on demand
5. OS needs initial RSP for initialization and overflow detection
