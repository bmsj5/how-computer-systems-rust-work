# Disk Loading and Page Address Splitting

## Question 1: Why Not Load Whole Executable into RAM?

**Your question:** "Why not load the whole source code into RAM at least to prevent it? Because it will slow the program down as hell."

**Answer:** **OS does load code into RAM, but uses lazy loading (on demand) for efficiency!**

### The Problem with Loading Everything

**If OS loaded entire executable at once:**

1. **Waste of RAM:**
   - Executable might be 100 MB
   - Program might only use 10% of code (most functions never called)
   - 90 MB wasted in RAM!

2. **Slow startup:**
   - Loading 100 MB from disk takes time (even with SSD: ~100ms)
   - Program can't start until everything is loaded
   - User waits longer

3. **Memory pressure:**
   - If 10 programs each load 100 MB = 1 GB RAM used
   - But only 10% actually needed = 100 MB would suffice
   - Other programs get less RAM

### How Lazy Loading Works (Current Approach)

**What actually happens:**

1. **OS maps file to virtual address space:**
   - Sets up page table entries
   - Marks pages as "file-backed, not in memory"
   - **No disk read yet!** (fast)

2. **Program starts immediately:**
   - Only first page (4KB) loaded from disk
   - Program can start executing
   - User sees program start quickly

3. **Pages loaded on demand:**
   - When CPU tries to execute code at new page
   - Page fault → OS loads that page from disk
   - Only pages actually used are loaded

**Example:**

```
Executable: 10 MB (2560 pages of 4KB each)

Traditional (load all):
  - Load 10 MB from disk: 100ms
  - Use 1 MB (10%): 10ms
  - Wasted: 9 MB
  - Total time: 100ms

Lazy loading (on demand):
  - Map file: 1ms (no disk read)
  - Load first page: 0.1ms
  - Program starts: 1.1ms total
  - Load pages as needed: 0.1ms per page
  - Only load 256 pages (1 MB): 25.6ms
  - Total time: 26.7ms (much faster!)
  - Wasted: 0 MB
```

### Why It's Not Slow in Practice

**Reasons lazy loading is fast:**

1. **Pages stay in RAM:**
   - Once loaded, page stays in RAM (cached)
   - Only first access is slow
   - Subsequent accesses are fast (from RAM)

2. **Spatial locality:**
   - Programs access nearby code (sequential execution)
   - Loading one page loads nearby code too
   - Next few pages likely to be accessed soon

3. **Prefetching:**
   - OS can predict which pages will be needed
   - Loads them in advance (while CPU executes)
   - Hides disk latency

4. **Modern storage is fast:**
   - SSD: ~500 MB/s read speed
   - 4KB page: ~0.008ms to load
   - CPU can execute thousands of instructions in that time

**Timeline:**

```
Time 0ms:   Program starts, first page loaded (0.1ms)
Time 0.1ms: CPU executes code from first page
Time 0.5ms: CPU needs next page → Page fault
Time 0.5ms: OS loads page from disk (0.008ms)
Time 0.508ms: CPU continues (page now in RAM)
Time 1ms:   CPU executes more code
Time 2ms:   CPU needs another page → Page fault
Time 2ms:   OS loads page (0.008ms)
Time 2.008ms: CPU continues
...
```

**Most time is spent executing code, not loading pages!**

### When OS Does Load Everything

**OS does load some things eagerly:**

1. **Critical startup code:**
   - Entry point and initialization code
   - Loaded immediately (can't start without it)

2. **Small executables:**
   - If executable < 1 MB, might load all
   - Faster than lazy loading overhead

3. **Memory-mapped files:**
   - Large data files can be memory-mapped
   - Loaded on demand, but OS can prefetch

### Summary: Lazy Loading Benefits

| Aspect | Load All | Lazy Loading |
|--------|----------|--------------|
| **Startup time** | Slow (load 100 MB) | Fast (load 4 KB) |
| **RAM usage** | High (all code) | Low (only used code) |
| **Disk I/O** | All at once | On demand |
| **Wasted memory** | High (unused code) | Low (only used code) |
| **User experience** | Slow start | Fast start |

**Key insight:** Lazy loading is faster for startup and uses less RAM. Pages stay in RAM once loaded, so subsequent accesses are fast.

## Question 2: How Does Page Address Splitting Work?

**Your question:** "What is this process of splitting the page? Does it split the page into high and lower levels to distinguish between the page itself (4KB) and individual bytes?"

**Answer:** **Yes! Exactly!** The address is split into page number (high bits) and offset (low bits).

### Address Format

**64-bit virtual address:**
```
63        52 51        12 11        0
┌──────────┬──────────────┬──────────┐
│ Unused   │ Page Number  │ Offset   │
│ (12 bits)│ (40 bits)    │ (12 bits)│
└──────────┴──────────────┴──────────┘
```

**Why 12 bits for offset?**
- Page size = 4 KB = 4096 bytes = 2^12 bytes
- Need 12 bits to address all bytes in a page (0 to 4095)

**Why 40 bits for page number?**
- 64-bit address space
- 12 bits for offset
- 12 bits unused (for future expansion)
- 40 bits for page number = 2^40 pages possible

### Example: Splitting Address

**Virtual address: 0x7FFF...D000**

**Step 1: Convert to binary**
```
0x7FFF...D000 = 
0111 1111 1111 1111 ... 1101 0000 0000 0000
```

**Step 2: Split into components**

**Full address:**
```
0x7FFF...D000
│           │
│           └─ Offset: 0x000 (12 bits, low)
└───────────── Page: 0x7FFF...D (52 bits, high)
```

**In binary:**
```
0111 1111 1111 1111 ... 1101 | 0000 0000 0000
└────────────────────────────┘ └──────────────┘
     Page Number (40 bits)      Offset (12 bits)
```

**Step 3: Extract components**

**Page number:**
- High 52 bits: `0x7FFF...D`
- This identifies which 4KB page

**Offset:**
- Low 12 bits: `0x000`
- This identifies which byte within the 4KB page (0 to 4095)

### How CPU Uses This

**When accessing `mov rax, [0x7FFF...D000]`:**

#### Step 1: CPU Splits Address

```asm
Virtual address: 0x7FFF...D000

CPU internally:
  page_number = (address >> 12) & 0xFFFFFFFFFF  ; Shift right 12 bits, mask 40 bits
  offset      = address & 0xFFF                  ; Mask low 12 bits

Result:
  page_number = 0x7FFF...D
  offset      = 0x000
```

**Why shift right 12 bits?**
- Offset is in low 12 bits
- Shift right 12 bits removes offset, leaves page number
- Mask keeps only 40 bits (page number size)

#### Step 2: CPU Looks Up Page Table

**Page table structure:**
```
Page Table (in RAM):
  Entry[0x7FFF...D] = {
    physical_page: 0x5000
    present: true
    permissions: read/write
  }
```

**CPU:**
1. Loads page table pointer from CR3 register
2. Uses page number as index: `page_table[0x7FFF...D]`
3. Gets physical page: `0x5000`

#### Step 3: CPU Combines Physical Page + Offset

**Physical address calculation:**
```
Physical address = (physical_page << 12) | offset
                 = (0x5000 << 12) | 0x000
                 = 0x5000000 | 0x000
                 = 0x5000000
```

**Why shift left 12 bits?**
- Physical page number is page-aligned (starts at 4KB boundary)
- Need to multiply by 4096 (2^12) to get base address
- Shift left 12 bits = multiply by 4096

#### Step 4: CPU Accesses Physical RAM

**Physical RAM:**
```
Physical Address    Content
─────────────────────────────
0x5000000          (start of page)
0x5000001          ...
0x5000002          ...
...
0x5000FFF          (end of page, 4095 bytes later)
```

**CPU:**
1. Accesses physical address: 0x5000000
2. Reads 8 bytes (u64) starting at that address
3. Returns value to register RAX

### Complete Example: Accessing u64 at 0x7FFF...D000

**Your code:**
```rust
let x: u64 = 100;  // Stored at 0x7FFF...D000
let value = x;     // Access x
```

**What happens:**

1. **Compiler generates:**
```asm
mov rax, [0x7FFF...D000]  ; Load 8 bytes from address
```

2. **CPU splits address:**
```
Virtual address: 0x7FFF...D000
  Page number: 0x7FFF...D (high 52 bits)
  Offset: 0x000 (low 12 bits)
```

3. **CPU looks up page table:**
```
Page table[0x7FFF...D] → Physical page: 0x5000
```

4. **CPU calculates physical address:**
```
Physical address = (0x5000 << 12) | 0x000
                 = 0x5000000
```

5. **CPU accesses physical RAM:**
```
Read 8 bytes from 0x5000000
Returns: 100 (u64 value)
```

6. **CPU stores in register:**
```
RAX = 100
```

### Why This Design?

**Benefits of page-based addressing:**

1. **Efficient page table:**
   - Only need one entry per 4KB page
   - 1 GB memory = 262,144 pages (manageable)
   - Without pages: 1 GB = 1 billion entries (too many!)

2. **Easy memory management:**
   - Allocate/deallocate in 4KB chunks
   - Simple to track which pages are used

3. **Cache efficiency:**
   - CPU cache works with pages
   - Loading one page loads nearby data

4. **Virtual memory:**
   - Can swap pages to disk
   - Can share pages between processes
   - Can protect pages (read-only, no-execute)

### Address Alignment

**Important:** Addresses are aligned to data size!

**u8 (1 byte):**
```
Address: 0x7FFF...D000
Offset: 0x000
Access: 1 byte at offset 0x000
```

**u32 (4 bytes):**
```
Address: 0x7FFF...D004
Offset: 0x004
Access: 4 bytes starting at offset 0x004
```

**u64 (8 bytes):**
```
Address: 0x7FFF...D008
Offset: 0x008
Access: 8 bytes starting at offset 0x008
```

**All within the same page (0x7FFF...D), just different offsets!**

### Summary: Page Splitting

| Component | Bits | Purpose |
|-----------|------|---------|
| **Page Number** | 40 bits (high) | Identifies which 4KB page |
| **Offset** | 12 bits (low) | Identifies which byte within page (0-4095) |
| **Total** | 52 bits used | 64-bit address space |

**Process:**
1. Split address: Page number (high) + Offset (low)
2. Look up page table: Page number → Physical page
3. Combine: Physical page + Offset = Physical address
4. Access: Read/write at physical address

**Key insight:** The offset (low 12 bits) maps perfectly to both virtual and physical pages because both use 4KB pages. The page number (high bits) is what gets translated from virtual to physical.
