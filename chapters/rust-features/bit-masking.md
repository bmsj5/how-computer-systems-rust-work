# Bit Masking and Powers of 2

## Question 1: Why Mask Bits After Shifting?

**Your question:** "Why do we need to mask the bits with AND operation, if we already could have the exact address: `page_number = (address >> 12)` if we just shift the right 12 bits?"

**Answer:** **To ensure we only get the page number bits and ignore any high bits!**

### The Problem Without Masking

**64-bit address:**
```
Address: 0x7FFF...D000

After shift right 12:
  (0x7FFF...D000 >> 12) = 0x7FFF...D

But wait! What if address has high bits set?
```

**Example with high bits:**
```
Address: 0xFFFF...D000 (high bits set)

After shift right 12:
  (0xFFFF...D000 >> 12) = 0xFFFF...D

Problem: High bits (bits 52-63) are still there!
```

### Why This Matters

**Page number should only be 40 bits (bits 12-51):**
```
63        52 51        12 11        0
┌──────────┬──────────────┬──────────┐
│ Unused   │ Page Number  │ Offset   │
│ (12 bits)│ (40 bits)    │ (12 bits)│
└──────────┴──────────────┴──────────┘
```

**Without masking:**
```
After shift: 0xFFFF...D (52 bits, includes unused high bits)
```

**With masking:**
```
After shift: 0xFFFF...D
After mask:  0x7FFF...D (40 bits, only page number)
```

### The Mask Operation

**Mask: `0xFFFFFFFFFF` (40 bits of 1s)**

```
0xFFFFFFFFFF in binary:
  1111 1111 1111 1111 1111 1111 1111 1111 1111 1111
  └──────────────────────────────────────────────┘
           40 bits of 1s

AND operation:
  (address >> 12) & 0xFFFFFFFFFF
  
Keeps only low 40 bits, clears high 12 bits
```

### Visual Example

**Address: 0xFFFF...D000**

**Step 1: Shift right 12 bits**
```
Before: 0xFFFF...D000
After:  0xFFFF...D (52 bits, includes high bits)
```

**Step 2: Mask with 0xFFFFFFFFFF**
```
Before mask: 0xFFFF...D (52 bits)
After mask:  0x7FFF...D (40 bits, high bits cleared)
```

**Result: Only page number bits remain!**

### Why Not Just Use 40-bit Type?

**Problem:** CPU registers are 64-bit, not 40-bit!

**When you do:**
```asm
mov rax, address      ; 64-bit value
shr rax, 12          ; Shift right 12 bits
; RAX still has 64 bits, high bits might be set!
```

**Need to mask to ensure only 40 bits:**
```asm
and rax, 0xFFFFFFFFFF  ; Clear high 24 bits, keep low 40 bits
```

### Summary

| Operation | Purpose |
|-----------|---------|
| **Shift right 12** | Remove offset, get page number |
| **Mask 0xFFFFFFFFFF** | Clear high bits, keep only 40-bit page number |

**Key insight:** Masking ensures we only get the page number bits (40 bits), ignoring any high bits that might be set.

## Question 2: Why Everything is Power of 2?

**Your question:** "Why everything is a power of 2? The total possible amount of RAM is 2^64, max usize is 2^64, a page is 2^12."

**Answer:** **Powers of 2 make binary operations fast and efficient!**

### Why Powers of 2?

**1. Binary representation:**
- Computers work in binary (base 2)
- Powers of 2 align with binary naturally
- Makes operations simple and fast

**2. Bit operations:**
- Shifting: `address >> 12` (divide by 4096)
- Masking: `address & 0xFFF` (get low 12 bits)
- These are single CPU instructions (very fast!)

**3. Memory alignment:**
- Aligned addresses are faster to access
- Powers of 2 make alignment easy
- No wasted space

**4. Hardware design:**
- CPU circuits work with powers of 2
- Makes hardware simpler and faster
- Aligns with binary nature of computers

### Examples

**Page size: 2^12 = 4096 bytes**

**Why not 5000 bytes?**
- Would need division: `address / 5000` (slow!)
- Would need modulo: `address % 5000` (slow!)
- Alignment would be complex

**With 4096 (2^12):**
- Shift: `address >> 12` (fast!)
- Mask: `address & 0xFFF` (fast!)
- Single CPU instruction each!

**Address space: 2^64**

**Why 64 bits?**
- Natural size for modern CPUs
- Allows 2^64 addresses
- Powers of 2 make calculations simple

**usize: 2^64**

**Why same as address space?**
- `usize` is pointer-sized
- Matches address space size
- Powers of 2 make it efficient

### Visual: Powers of 2 in Memory

```
2^0  = 1 byte      (smallest unit)
2^1  = 2 bytes
2^2  = 4 bytes     (u32)
2^3  = 8 bytes     (u64, pointer)
2^4  = 16 bytes
2^8  = 256 bytes
2^10 = 1024 bytes  (1 KB)
2^12 = 4096 bytes  (4 KB, page size)
2^20 = 1 MB
2^30 = 1 GB
2^40 = 1 TB
2^64 = 16 EB       (address space)
```

**All powers of 2!**

### Why This Matters

**Operations with powers of 2:**
```rust
// Fast (single CPU instruction)
let page = address >> 12;        // Divide by 4096
let offset = address & 0xFFF;    // Modulo 4096

// Slow (multiple CPU instructions)
let page = address / 5000;       // Division (slow!)
let offset = address % 5000;     // Modulo (slow!)
```

**Performance difference:**
- Powers of 2: 1 CPU cycle
- Non-powers of 2: 10-100 CPU cycles

### Summary

| Value | Power of 2 | Why |
|-------|-----------|-----|
| **Page size** | 2^12 (4096) | Fast bit operations |
| **Address space** | 2^64 | Natural CPU size |
| **usize** | 2^64 | Matches address space |
| **Register size** | 2^3 (8 bytes) | CPU architecture |

**Key insight:** Powers of 2 make binary operations fast and efficient. Everything aligns with binary nature of computers.
