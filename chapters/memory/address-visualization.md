# Address Access: Visual Explanation

## Question 1: Visual Representation of 12 Bits Addressing 4096 Bytes

**Your question:** "Can you visually show how 12 bits address 4096 bytes?"

### Visual: 12 Bits Addressing 4096 Bytes

```
12 bits in binary: 0000 0000 0000 to 1111 1111 1111
                  └──────────────────────────────┘
                          12 bits

Decimal range: 0 to 4095 (2^12 - 1)

Page layout (4 KB = 4096 bytes):
┌─────────────────────────────────────────┐
│ Byte 0    │ Offset: 0x000 (binary: 0000 0000 0000) │
│ Byte 1    │ Offset: 0x001 (binary: 0000 0000 0001) │
│ Byte 2    │ Offset: 0x002 (binary: 0000 0000 0010) │
│ ...       │ ...                                      │
│ Byte 255  │ Offset: 0x0FF (binary: 0000 1111 1111) │
│ Byte 256  │ Offset: 0x100 (binary: 0001 0000 0000) │
│ ...       │ ...                                      │
│ Byte 4095 │ Offset: 0xFFF (binary: 1111 1111 1111) │
└─────────────────────────────────────────┘
        4096 bytes total (0 to 4095)

12 bits can represent:
  2^12 = 4096 different values
  Range: 0 to 4095
  Perfect for addressing all bytes in a 4KB page!
```

### Bit-by-Bit Breakdown

```
Offset: 0x000 (0 decimal)
Binary: 0000 0000 0000
        └──────────────┘
        12 bits = 0

Offset: 0x001 (1 decimal)
Binary: 0000 0000 0001
        └──────────────┘
        12 bits = 1

Offset: 0x0FF (255 decimal)
Binary: 0000 1111 1111
        └──────────────┘
        12 bits = 255

Offset: 0xFFF (4095 decimal)
Binary: 1111 1111 1111
        └──────────────┘
        12 bits = 4095 (maximum)
```

### Why 12 Bits?

```
Page size: 4 KB = 4096 bytes

To address all bytes:
  Need to count from 0 to 4095
  That's 4096 different values
  2^12 = 4096 ✓

12 bits gives us exactly 4096 values (0 to 4095)
Perfect fit for 4KB page!
```

## Question 2: How CPU Reads u32 at Offset 0x004

**Your question:** "I thought CPU reads like this: splits 0x7FFF...D004 into 0x7FFF...D and 0x004, looks up for the exact start, knows x is a u32 data which is 4 bytes length and just reads 0x004 + 4 bytes?"

**Answer:** **Almost correct! But CPU doesn't "know" it's u32 - the instruction tells it how many bytes to read.**

### How CPU Actually Reads

**Your code:**
```rust
let x: u32 = 100;  // Stored at 0x7FFF...D004
let value = x;     // Access x
```

**Compiler generates:**
```asm
mov eax, [0x7FFF...D004]  ; Load 4 bytes (u32) from address
```

**The `eax` register and `dword` size tell CPU it's 4 bytes!**

### Step-by-Step Process

**Step 1: CPU splits address**
```
Virtual address: 0x7FFF...D004

Split:
  Page number: 0x7FFF...D (high bits)
  Offset: 0x004 (low 12 bits)
```

**Step 2: CPU looks up page table**
```
Page table[0x7FFF...D] → Physical page: 0x5000
```

**Step 3: CPU calculates physical address**
```
Physical address = (0x5000 << 12) | 0x004
                 = 0x5000000 + 0x004
                 = 0x5000004
```

**Step 4: CPU reads bytes (instruction tells it how many!)**
```
Instruction: mov eax, [0x7FFF...D004]
             └───┘
             eax = 32-bit register = 4 bytes

CPU reads: 4 bytes starting at 0x5000004
  Byte 0x5000004: 0x64 (100 in decimal, low byte)
  Byte 0x5000005: 0x00
  Byte 0x5000006: 0x00
  Byte 0x5000007: 0x00 (high byte)

CPU combines: 0x00000064 = 100 (u32)
```

### Visual: Reading u32 at Offset 0x004

```
Physical page: 0x5000000 (start of page)
Offset: 0x004

Physical RAM:
┌─────────────────────────────────────────┐
│ 0x5000000 │ (some data)                 │
│ 0x5000001 │ (some data)                 │
│ 0x5000002 │ (some data)                 │
│ 0x5000003 │ (some data)                 │
│ 0x5000004 │ 0x64 ← u32 starts here      │
│ 0x5000005 │ 0x00 │                      │
│ 0x5000006 │ 0x00 │ 4 bytes total        │
│ 0x5000007 │ 0x00 ← u32 ends here        │
│ 0x5000008 │ (some data)                 │
│ ...       │ ...                         │
└─────────────────────────────────────────┘

CPU reads: 4 bytes from 0x5000004 to 0x5000007
Result: 0x00000064 = 100 (u32)
```

### How CPU Knows How Many Bytes

**The instruction tells CPU!**

```asm
; Different instructions for different sizes:

mov al, [address]   ; Load 1 byte (u8)  → AL register (8 bits)
mov ax, [address]   ; Load 2 bytes (u16) → AX register (16 bits)
mov eax, [address]  ; Load 4 bytes (u32) → EAX register (32 bits)
mov rax, [address]  ; Load 8 bytes (u64) → RAX register (64 bits)
```

**CPU doesn't "know" it's u32 - the instruction size tells it!**

### Complete Example: Different Data Types

**u8 at 0x7FFF...D000:**
```asm
mov al, [0x7FFF...D000]  ; Load 1 byte
```
- CPU reads: 1 byte at offset 0x000
- Result: 8-bit value in AL

**u32 at 0x7FFF...D004:**
```asm
mov eax, [0x7FFF...D004]  ; Load 4 bytes
```
- CPU reads: 4 bytes starting at offset 0x004
- Result: 32-bit value in EAX

**u64 at 0x7FFF...D008:**
```asm
mov rax, [0x7FFF...D008]  ; Load 8 bytes
```
- CPU reads: 8 bytes starting at offset 0x008
- Result: 64-bit value in RAX

### Your Understanding - Corrected

**What you said:**
> "CPU splits 0x7FFF...D004 into 0x7FFF...D and 0x004, looks up for the exact start, knows x is a u32 data which is 4 bytes length and just reads 0x004 + 4 bytes?"

**Corrected:**
> "CPU splits 0x7FFF...D004 into 0x7FFF...D and 0x004, looks up page table to get physical page, calculates physical address (physical_page + 0x004), and the **instruction** (mov eax) tells CPU to read 4 bytes starting at that address."

**Key difference:**
- CPU doesn't "know" it's u32
- The **instruction** (register size) tells CPU how many bytes to read
- CPU reads that many bytes starting at the calculated address

### Summary

| Step | What Happens |
|------|--------------|
| **1. Split address** | Page number (high) + Offset (low) |
| **2. Look up page table** | Page number → Physical page |
| **3. Calculate physical address** | Physical page + Offset |
| **4. Read bytes** | Instruction tells CPU how many bytes to read |
| **5. Return value** | Bytes combined into register |

**Key insight:** CPU doesn't know data types - the instruction (register size) tells it how many bytes to read!
