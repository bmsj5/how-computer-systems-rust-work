# Bit Operations and Hexadecimal

## Question 3: Bit Operations Explained

**Your question:** "Does shift mean just shift these 1s to the right and they become 0s? What are bit operations? What do &, | do?"

**Answer:** **Yes! Shift moves bits left/right. & is AND, | is OR.**

### Bit Operations

**Basic bit operations:**

1. **Shift left (`<<`):**
   - Moves bits to the left
   - Fills right with 0s
   - Equivalent to multiply by 2^n

2. **Shift right (`>>`):**
   - Moves bits to the right
   - Fills left with 0s (or sign bit for signed)
   - Equivalent to divide by 2^n

3. **AND (`&`):**
   - Bitwise AND operation
   - Result is 1 only if both bits are 1

4. **OR (`|`):**
   - Bitwise OR operation
   - Result is 1 if either bit is 1

### Visual Examples

**Shift right (`>>`):**
```
Before: 0xFFFF...D000
Binary: 1111 1111 1111 ... 1101 0000 0000 0000

After >> 12:
Binary: 0000 0000 0000 ... 1111 1111 1111 ... 1101
        └──────────────┘
        New zeros (shifted in from left)

Result: 0xFFFF...D (high bits still there!)
```

**Why mask is needed:**
```
After shift: 0xFFFF...D (52 bits, includes high bits)

Mask with 0xFFFFFFFFFF (40 bits of 1s):
  0xFFFF...D & 0xFFFFFFFFFF

Binary:
  1111 1111 ... 1101  (52 bits)
& 0000 1111 ... 1111  (40 bits of 1s, 12 bits of 0s)
─────────────────────
  0000 1111 ... 1101  (40 bits, high bits cleared!)

Result: 0x7FFF...D (only page number bits)
```

### AND Operation (`&`)

**Truth table:**
```
A  B  A & B
0  0   0
0  1   0
1  0   0
1  1   1
```

**Example:**
```
0xFF & 0xF0

Binary:
  1111 1111  (0xFF)
& 1111 0000  (0xF0)
───────────
  1111 0000  (0xF0)

Result: Keeps high 4 bits, clears low 4 bits
```

**Use case: Masking**
```
address & 0xFFF  // Get low 12 bits (offset)
```

### OR Operation (`|`)

**Truth table:**
```
A  B  A | B
0  0   0
0  1   1
1  0   1
1  1   1
```

**Example:**
```
0x50 | 0x0F

Binary:
  0101 0000  (0x50)
| 0000 1111  (0x0F)
───────────
  0101 1111  (0x5F)

Result: Sets low 4 bits to 1
```

**Use case: Combining**
```
(physical_page << 12) | offset  // Combine page and offset
```

### Complete Example: Address Splitting

**Address: 0x7FFF...D000**

**Step 1: Shift right 12 bits**
```
0x7FFF...D000 >> 12

Binary:
  0111 1111 1111 ... 1101 0000 0000 0000
  >> 12 (shift right)
  0000 0000 0000 ... 0111 1111 1111 ... 1101

Result: 0x7FFF...D (but might have high bits if address was larger)
```

**Step 2: Mask with 0xFFFFFFFFFF**
```
0x7FFF...D & 0xFFFFFFFFFF

Binary:
  0111 1111 1111 ... 1101  (might be 52 bits)
& 0000 1111 1111 ... 1111  (40 bits of 1s)
─────────────────────────
  0111 1111 1111 ... 1101  (40 bits, high bits cleared)

Result: 0x7FFF...D (only page number)
```

**Step 3: Get offset**
```
0x7FFF...D000 & 0xFFF

Binary:
  0111 1111 1111 ... 1101 0000 0000 0000
& 0000 0000 0000 ... 0000 1111 1111 1111
───────────────────────────────────────
  0000 0000 0000 ... 0000 0000 0000 0000

Result: 0x000 (offset)
```

### Summary

| Operation | Symbol | What It Does |
|-----------|--------|--------------|
| **Shift left** | `<<` | Move bits left, fill right with 0s |
| **Shift right** | `>>` | Move bits right, fill left with 0s |
| **AND** | `&` | Bitwise AND (both bits must be 1) |
| **OR** | `\|` | Bitwise OR (either bit can be 1) |

**Key insight:** Shift moves bits, AND masks bits, OR combines bits. These are fundamental CPU operations (very fast, single cycle).

## Question 4: Hexadecimal vs Decimal

**Your question:** "What should I know about hexadecimal vs decimal to understand addresses and bit operations?"

**Answer:** **Hexadecimal is base 16, perfect for representing binary. Each hex digit = 4 bits.**

### Why Hexadecimal?

**Hexadecimal (base 16):**
- Uses digits: 0-9, A-F
- Each digit represents 4 bits
- Easy to convert to/from binary
- Compact representation

**Decimal (base 10):**
- Uses digits: 0-9
- Each digit represents 10 values
- Hard to convert to/from binary
- Verbose for addresses

### Hexadecimal Basics

**Digits:**
```
0 = 0 (binary: 0000)
1 = 1 (binary: 0001)
2 = 2 (binary: 0010)
...
9 = 9 (binary: 1001)
A = 10 (binary: 1010)
B = 11 (binary: 1011)
C = 12 (binary: 1100)
D = 13 (binary: 1101)
E = 14 (binary: 1110)
F = 15 (binary: 1111)
```

**Each hex digit = 4 bits:**
```
0x0 = 0000 (4 bits)
0xF = 1111 (4 bits)
0xFF = 1111 1111 (8 bits = 1 byte)
0xFFF = 1111 1111 1111 (12 bits)
```

### Common Patterns

**Powers of 2 in hex:**
```
2^0  = 1    = 0x1
2^4  = 16   = 0x10
2^8  = 256  = 0x100
2^12 = 4096 = 0x1000
2^16 = 65536 = 0x10000
```

**Page size:**
```
4 KB = 4096 bytes = 0x1000 (hex)
```

**Address alignment:**
```
0x1000 = 4096 (page-aligned)
0x1001 = 4097 (not aligned)
0x1004 = 4100 (aligned to 4 bytes)
0x1008 = 4104 (aligned to 8 bytes)
```

### Converting Between Bases

**Hex to Binary:**
```
0xFF = 1111 1111
  F     F
  1111  1111
```

**Binary to Hex:**
```
1111 1111 = 0xFF
1111 = F
1111 = F
```

**Hex to Decimal:**
```
0xFF = 15*16 + 15 = 240 + 15 = 255
0x1000 = 1*4096 = 4096
```

**Decimal to Hex:**
```
255 = 0xFF (15*16 + 15)
4096 = 0x1000 (1*4096)
```

### Why Hex for Addresses?

**Address: 0x7FFF...D000**

**In decimal:**
```
140737488355328 (hard to read!)
```

**In hex:**
```
0x7FFF...D000 (easy to see page number and offset!)
```

**Split easily:**
```
0x7FFF...D000
│           │
│           └─ 0x000 (offset, low 12 bits)
└───────────── 0x7FFF...D (page, high bits)
```

### Interesting Hex Patterns

**All 1s:**
```
0xFF = 255 (1 byte, all bits set)
0xFFF = 4095 (12 bits, all bits set)
0xFFFF = 65535 (2 bytes, all bits set)
0xFFFFFFFF = 4294967295 (4 bytes, all bits set)
```

**Powers of 2:**
```
0x1 = 1
0x2 = 2
0x4 = 4
0x8 = 8
0x10 = 16
0x20 = 32
0x40 = 64
0x80 = 128
0x100 = 256
```

**Masks:**
```
0xFF = mask low 8 bits
0xFFF = mask low 12 bits (page offset)
0xFFFF = mask low 16 bits
0xFFFFFFFF = mask low 32 bits
```

### Summary

| Aspect | Hexadecimal | Decimal |
|--------|-------------|---------|
| **Base** | 16 | 10 |
| **Digits** | 0-9, A-F | 0-9 |
| **Bits per digit** | 4 | ~3.3 |
| **Address representation** | Compact (0x7FFF...D000) | Verbose (140737488355328) |
| **Binary conversion** | Easy (1 digit = 4 bits) | Hard |

**Key insight:** Hexadecimal is perfect for addresses and bit operations because each digit represents exactly 4 bits, making it easy to see binary patterns and split addresses.
