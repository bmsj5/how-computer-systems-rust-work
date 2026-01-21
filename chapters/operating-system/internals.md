# OS Internals: Detailed Explanations

## Question 1: Why OS Sets RSP? Guard Pages? Context Switching?

### 1.1: Why OS Sets RSP (Can't Program Figure It Out?)

**Your question:** "If virtual address space is the same for every program, why can't the program figure out RSP itself?"

**Answer:** The program **could** theoretically figure it out, but:

1. **OS needs to initialize the process:**
   - When OS creates a new process, it needs to set up the initial state
   - RSP is part of that initial state
   - Program doesn't exist yet to set its own RSP!

2. **OS controls process creation:**
   - OS allocates the virtual address space
   - OS decides where stack starts (not always the same!)
   - OS sets up page tables
   - Program just runs - it doesn't create itself

3. **Different architectures/platforms:**
   - Stack start address can vary (0x7FFF...F000 on Linux, different on others)
   - OS abstracts this away
   - Program doesn't need to know platform-specific details

**What actually happens:**

```
Process creation:
1. OS: "Create new process"
2. OS: "Allocate virtual address space"
3. OS: "Set stack region: 0x7FFF...F000 to 0x7FFF...0000"
4. OS: "Set RSP = 0x7FFF...F000" (in CPU register)
5. OS: "Set RIP = 0x400000" (entry point, from executable)
6. OS: "Start executing"
7. Program: "I'm running, RSP is already set!"
```

**The program doesn't set RSP - OS does during process creation!**

### 1.2: Guard Pages vs OS Stack Overflow Detection

**Your question:** "I thought guard page handles stack overflow. Does OS really check, or is guard page just an abstraction?"

**Answer:** **Both work together!** Here's how:

#### Guard Page Mechanism

**What is a guard page?**
- A special page at the bottom of stack (lowest address)
- Marked as "no access" in page table
- If accessed → **Page fault** → OS detects it

**How it works:**

```
Stack layout:
┌─────────────────────┐
│ 0x7FFF...F000 (RSP) │ ← Stack top (grows down)
│ ...                 │
│ 0x7FFF...1000       │ ← End of usable stack
├─────────────────────┤
│ 0x7FFF...0000       │ ← GUARD PAGE (no access!)
└─────────────────────┘

Page table:
  0x7FFF...F → 0x5000 (physical, readable)
  0x7FFF...E → 0x6000 (physical, readable)
  ...
  0x7FFF...0 → (marked as GUARD, no physical page)
```

**When stack overflow happens:**

1. **Program tries to access guard page:**
   - RSP moves to 0x7FFF...0000 (guard page)
   - Program tries to access: `mov [rsp], rax`

2. **CPU detects page fault:**
   - CPU: "Access 0x7FFF...0000"
   - CPU: "Page table says: GUARD PAGE (no access)"
   - CPU: **Page fault!** (interrupt to OS)

3. **OS handles page fault:**
   - OS: "Page fault on guard page → stack overflow!"
   - OS: "Kill process" (sends SIGSEGV)

**So guard page is the mechanism, OS is the handler!**

#### OS Stack Limit Check

**OS also maintains stack limit for additional safety:**

```
OS tracks:
  initial_RSP = 0x7FFF...F000
  stack_limit = 0x7FFF...0000 (8 MB below)
  
On context switch or memory check:
  if (current_RSP < stack_limit) {
    // Stack overflow detected
    kill_process();
  }
```

**Why both?**
- **Guard page:** Hardware-level protection (automatic)
- **OS check:** Software-level safety (explicit check)
- **Redundancy:** Multiple layers of protection

**Summary:**
- Guard page triggers page fault on access
- OS handles the page fault and kills process
- OS also checks stack limit explicitly
- Both work together for safety

### 1.3: Context Switching and RSP

**Your question:** "Does OS save/restore RSP during context switching?"

**Answer:** **Yes!** Here's how:

#### Context Switching Process

**What is context switching?**
- OS switches from one process to another
- Each process has its own state (registers, memory, etc.)
- OS must save current process's state, restore next process's state

**What OS saves/restores:**

```
Process Control Block (PCB) - OS data structure:

Process A:
  RSP = 0x7FFF...F000
  RIP = 0x400123 (current instruction)
  RAX = 42
  RBX = 100
  ... (all registers)
  Page table pointer
  Stack region
  Heap region
  ...

Process B:
  RSP = 0x7FFF...E000
  RIP = 0x500456
  RAX = 99
  ...
```

**Context switch steps:**

1. **Save current process (A):**
```asm
; OS code (kernel mode)
push all registers to Process A's kernel stack
save RSP_A = current RSP value
save RIP_A = current instruction pointer
save all other registers
```

2. **Switch page table:**
```asm
; OS switches to Process B's page table
mov cr3, Process_B_page_table_pointer
; Now virtual addresses map to Process B's physical pages
```

3. **Restore next process (B):**
```asm
; OS restores Process B's state
load RSP_B from PCB
load RIP_B from PCB
load all other registers
mov rsp, RSP_B  ; Restore stack pointer
```

4. **Return to Process B:**
```asm
; OS returns control to Process B
; Process B continues from where it left off
; RSP now points to Process B's stack
```

**Key point:** RSP is saved/restored like any other register. Each process has its own RSP value.

## Question 2: How Are Instructions Loaded from Disk?

**Your question:** "Are instructions mapped from disk to RAM during execution? Or in registers? Or OS reads sequentially?"

**Answer:** **Instructions are mapped from disk to RAM, not loaded into registers!**

### How It Works

#### Step 1: Executable File on Disk

**When you compile:**
```
rustc main.rs → main (executable file on disk)

File contains:
  - Code section (instructions)
  - Data section (global variables)
  - Metadata (entry point, etc.)
```

#### Step 2: OS Maps File to Memory (Not Copies!)

**When process starts:**

1. **OS reads executable metadata:**
   - Entry point: 0x400000 (where code starts)
   - Code size: 1 MB
   - Data size: 100 KB

2. **OS sets up virtual address space:**
   - Code region: 0x400000 to 0x500000 (virtual)
   - Data region: 0x600000 to 0x610000 (virtual)

3. **OS maps file pages to virtual pages:**
   - Virtual page 0x400000 → File page 0 (first 4KB of code)
   - Virtual page 0x400010 → File page 1 (next 4KB of code)
   - ...
   - **But doesn't copy to RAM yet!**

4. **Page table entries:**
   - Marked as "not in memory" (page fault on access)
   - Points to file location on disk

#### Step 3: Lazy Loading (On Demand)

**When program executes:**

1. **CPU tries to execute instruction:**
   - RIP = 0x400000 (entry point)
   - CPU: "Access 0x400000"
   - CPU: "Page table says: Not in memory!"
   - CPU: **Page fault!**

2. **OS handles page fault:**
   - OS: "Page fault on code page"
   - OS: "This page is mapped to file"
   - OS: Reads 4KB from disk (file page 0)
   - OS: Allocates physical RAM page
   - OS: Copies file data to RAM
   - OS: Updates page table: Virtual 0x400000 → Physical 0x7000
   - OS: Returns to program

3. **Program continues:**
   - CPU: "Access 0x400000"
   - CPU: "Page table: 0x400000 → 0x7000 (physical)"
   - CPU: Reads instruction from RAM
   - CPU: Executes instruction

#### Step 4: Instructions Stay in RAM

**Instructions are:**
- Loaded from disk to RAM (on demand)
- Stay in RAM (cached)
- Executed from RAM (not from disk!)
- Not in registers (registers hold data, not instructions)

**Registers hold:**
- Data values (RAX = 42)
- Addresses (RSP = 0x7FFF...F000)
- Instruction pointer (RIP = 0x400123)

**RAM holds:**
- Instructions (code)
- Data (variables)
- Stack
- Heap

**Disk holds:**
- Executable file (backup copy)
- Swapped-out pages (if RAM is full)

### Summary

| Location | Contains | When |
|----------|----------|------|
| **Disk** | Executable file | Before execution |
| **RAM** | Instructions (loaded on demand) | During execution |
| **Registers** | Data, addresses, RIP | During execution |
| **CPU** | Executes instructions from RAM | Always |

**Key insight:** Instructions are **mapped** from disk, **loaded** to RAM on demand, **executed** from RAM. Not in registers!

## Question 3: Page Fault Process in Detail

**Your question:** "How does page fault work? Does CPU call OS? How does OS distinguish page fault from segmentation fault?"

**Answer:** **CPU interrupts OS automatically. OS distinguishes by checking page table and memory regions.**

### Page Fault Mechanism

#### Step 1: CPU Detects Page Fault

**When CPU tries to access memory:**

```asm
; Program instruction
mov rax, [0x7FFF...D000]  ; Load from memory

; CPU internally:
1. Calculate virtual address: 0x7FFF...D000
2. Split: Page = 0x7FFF...D, Offset = 0x000
3. Look up page table:
   - Load page table pointer from CR3 register
   - Check page table entry for 0x7FFF...D
   - Result: "Not present" or "No access"
4. CPU: "Page fault detected!"
5. CPU: **Automatically interrupts OS** (no function call!)
```

**CPU hardware automatically:**
- Saves current state (RIP, RSP, registers)
- Switches to kernel mode
- Jumps to OS page fault handler
- **No function call - it's a hardware interrupt!**

#### Step 2: OS Page Fault Handler

**OS page fault handler (in kernel):**

```c
// Simplified OS code
void page_fault_handler() {
    // Get fault information from CPU
    virtual_address = get_fault_address();  // 0x7FFF...D000
    fault_type = get_fault_type();          // Read/Write/Execute
    fault_reason = get_fault_reason();      // Not present / Access denied
    
    // Check page table entry
    page_table_entry = lookup_page_table(virtual_address);
    
    if (page_table_entry.is_guard_page()) {
        // Guard page access → Stack overflow
        kill_process(SIGSEGV);
        return;
    }
    
    if (page_table_entry.is_file_mapped()) {
        // File-backed page → Load from disk
        load_page_from_disk(virtual_address);
        return;
    }
    
    if (page_table_entry.is_stack_region()) {
        // Stack growth → Allocate new page
        allocate_stack_page(virtual_address);
        return;
    }
    
    if (page_table_entry.is_heap_region()) {
        // Heap growth → Allocate new page
        allocate_heap_page(virtual_address);
        return;
    }
    
    // Invalid address → Segmentation fault
    kill_process(SIGSEGV);
}
```

#### Step 3: OS Distinguishes Fault Types

**How OS distinguishes:**

1. **Check page table entry:**
   - **Present bit:** Is page in memory?
   - **Access bits:** Read/write/execute permissions
   - **Special flags:** Guard page, file-mapped, etc.

2. **Check memory region:**
   - Is address in stack region? → Stack growth
   - Is address in heap region? → Heap allocation
   - Is address in code region? → Code loading
   - Is address in valid region? → Segmentation fault

3. **Check fault reason:**
   - **Not present:** Page not in memory → Load from disk/allocate
   - **Access denied:** Permission violation → Segmentation fault
   - **Guard page:** Special marker → Stack overflow

**Example scenarios:**

**Scenario 1: Stack Growth (Valid)**
```
Virtual address: 0x7FFF...D000
Page table: "Not present"
Memory region: Stack region
Action: Allocate new stack page
Result: Continue execution
```

**Scenario 2: Guard Page (Stack Overflow)**
```
Virtual address: 0x7FFF...0000
Page table: "Guard page, no access"
Memory region: Stack region (but at limit)
Action: Kill process (SIGSEGV)
Result: Process terminated
```

**Scenario 3: Invalid Address (Segmentation Fault)**
```
Virtual address: 0x12345678
Page table: "Not present"
Memory region: Not in any valid region
Action: Kill process (SIGSEGV)
Result: Process terminated
```

**Scenario 4: Code Loading (Valid)**
```
Virtual address: 0x400000
Page table: "File-mapped, not in memory"
Memory region: Code region
Action: Load from executable file
Result: Continue execution
```

### Complete Page Fault Flow

```
1. Program: mov rax, [0x7FFF...D000]
2. CPU: "Access 0x7FFF...D000"
3. CPU: "Page table: Not present"
4. CPU: **Hardware interrupt** → OS
5. OS: "Page fault! Address: 0x7FFF...D000"
6. OS: "Check page table entry"
7. OS: "Check memory region"
8. OS: "Stack region, not present → Stack growth"
9. OS: Allocate physical page
10. OS: Update page table
11. OS: Return to program
12. Program: Continues (instruction retries)
13. CPU: "Access 0x7FFF...D000"
14. CPU: "Page table: Present → 0x6000 (physical)"
15. CPU: Reads from RAM
16. CPU: Continues execution
```

## Summary

| Question | Answer |
|----------|--------|
| **1. Why OS sets RSP?** | OS creates process, sets initial state. Program doesn't create itself. |
| **2. Guard page vs OS check?** | Both! Guard page triggers fault, OS handles it. Redundant protection. |
| **3. Context switching?** | Yes, OS saves/restores RSP (and all registers) for each process. |
| **4. Instructions from disk?** | Mapped from disk, loaded to RAM on demand, executed from RAM. |
| **5. Page fault mechanism?** | CPU hardware interrupt → OS handler → Check page table/region → Handle or kill. |
| **6. Distinguish faults?** | OS checks page table entry, memory region, fault reason. |

**Key insights:**
1. OS sets RSP during process creation (program doesn't exist yet)
2. Guard page + OS check = redundant protection
3. Context switching saves/restores all registers (including RSP)
4. Instructions: Disk → RAM (on demand) → CPU executes from RAM
5. Page fault = hardware interrupt, not function call
6. OS distinguishes by checking page table, memory regions, fault reason
