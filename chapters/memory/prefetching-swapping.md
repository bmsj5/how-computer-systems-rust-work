# Prefetching and Page Swapping

## Question 1: How Does Prefetching Work?

**Your question:** "If CPU accesses different page, OS just loaded an extra page, but this is okay and it stays in memory, or does this prefetch act like a real cache with some LRU mechanism? How many pages are prefetched in advance?"

**Answer:** **Prefetched pages stay in memory, but OS can evict them if RAM is full (using LRU-like mechanisms).**

### Prefetching Mechanism

**What happens when OS prefetches:**

1. **OS prefetches page:**
   - Loads page from disk to RAM
   - Updates page table: "Page is now in memory"
   - Page stays in RAM (cached)

2. **If CPU uses prefetched page:**
   - CPU accesses page → Already in RAM!
   - No page fault, fast access
   - Prefetch was successful

3. **If CPU doesn't use prefetched page:**
   - Page stays in RAM (wasted space)
   - But if RAM is full, OS can evict it
   - Uses LRU (Least Recently Used) mechanism

### How Many Pages Are Prefetched?

**It's dynamic, but typically:**

- **Sequential access:** 2-4 pages ahead
- **Stride pattern:** 1-2 pages ahead
- **Adjacent access:** 1 page ahead

**OS algorithms:**

**1. Sequential Prefetching:**
```
If CPU accesses pages: N, N+1, N+2
Then prefetch: N+3, N+4 (2 pages ahead)
```

**2. Aggressive Prefetching:**
```
If sequential pattern detected:
  Prefetch: N+3, N+4, N+5 (3 pages ahead)
```

**3. Conservative Prefetching:**
```
If uncertain pattern:
  Prefetch: N+1 (1 page ahead)
```

**Modern OSes use adaptive algorithms:**
- Start conservative (1 page)
- If pattern continues, become aggressive (3-4 pages)
- If pattern breaks, become conservative again

### What Happens to Prefetched Pages?

**If RAM is not full:**
- Prefetched pages stay in RAM
- Available for immediate use
- No eviction needed

**If RAM is full:**
- OS uses LRU (Least Recently Used) to evict pages
- Prefetched pages that aren't used are candidates for eviction
- Recently accessed pages stay in RAM

**LRU mechanism:**
```
Page access order:
  Page A (accessed)
  Page B (accessed)
  Page C (prefetched, not used)
  Page D (accessed)
  
If RAM is full:
  Evict Page C (least recently used)
  Keep Pages A, B, D (recently used)
```

### Summary

| Aspect | How It Works |
|--------|--------------|
| **Pages prefetched** | 1-4 pages (dynamic, adaptive) |
| **Stay in memory?** | Yes, until RAM is full |
| **Eviction?** | Yes, if RAM full (LRU mechanism) |
| **Wasted if unused?** | Temporarily, but can be evicted |

**Key insight:** Prefetching is a prediction - sometimes wrong, but usually right. Wrong predictions waste some RAM temporarily, but OS can evict unused prefetched pages if needed.

## Question 2: Virtual Memory and Page Swapping

**Your question:** "How does virtual memory work and page swapping? When I'm out of RAM, but disk is used instead. I know it should be fixed value, set before any program executes, on OS startup, called paging file or something like this."

**Answer:** **Yes! Virtual memory uses a swap file/page file on disk. OS swaps pages to disk when RAM is full.**

### How Virtual Memory Works

**The illusion:**
- Each process thinks it has huge address space (2^64 bytes)
- But physical RAM is limited (e.g., 16 GB)
- OS uses disk (swap file) as "backup RAM"

**The reality:**
- Some pages in RAM (fast)
- Some pages on disk (slow)
- OS swaps pages between RAM and disk as needed

### Swap File/Page File

**What is it?**
- Fixed-size file on disk (set at OS startup)
- Usually 1-2x RAM size (e.g., 16 GB RAM → 16-32 GB swap)
- Can be on HDD or SSD
- Managed by OS kernel

**Linux example:**
```bash
# Swap file (can be created dynamically)
/dev/sda2  swap  swap  defaults  0  0

# Or swap partition
# Created during OS installation
```

**Windows example:**
```
C:\pagefile.sys (hidden system file)
Size: Set automatically or manually
```

### How Page Swapping Works

**Step 1: RAM is full**
```
Physical RAM (16 GB):
  Process A: 4 GB
  Process B: 4 GB
  Process C: 4 GB
  Process D: 4 GB
  Total: 16 GB (full!)
```

**Step 2: New page needed**
```
Process E needs new page
But RAM is full!
```

**Step 3: OS swaps page to disk**
```
OS selects page to evict (LRU):
  Process A's page (not used recently)
  
OS:
  1. Writes page to swap file on disk
  2. Marks page table: "Page is on disk, not in RAM"
  3. Frees RAM page
  4. Allocates RAM page for Process E
```

**Step 4: Page table updated**
```
Page table entry:
  Virtual page: 0x400000
  Physical page: (on disk, swap file offset 0x1000)
  Present bit: false (not in RAM)
  Swapped bit: true (on disk)
```

**Step 5: When page needed again**
```
Process A accesses swapped page:
  1. CPU: "Page fault! Page not in RAM"
  2. OS: "Page is on disk (swapped)"
  3. OS: Selects another page to swap out
  4. OS: Reads page from swap file
  5. OS: Writes to RAM
  6. OS: Updates page table
  7. CPU: Continues execution
```

### Visual: Page Swapping

```
Physical RAM (16 GB):
┌─────────────────────┐
│ Process A: 4 GB     │
│ Process B: 4 GB     │
│ Process C: 4 GB     │
│ Process D: 4 GB     │ ← Full!
└─────────────────────┘

Swap File on Disk (32 GB):
┌─────────────────────┐
│ Process A pages     │ ← Swapped out
│ Process B pages     │ ← Swapped out
│ (unused space)      │
└─────────────────────┘

When Process A needs swapped page:
  1. OS swaps out Process D page (LRU)
  2. OS loads Process A page from swap file
  3. Process A continues
```

### When Swapping Happens

**OS swaps pages when:**
1. **RAM is full:**
   - New page needed, but no free RAM
   - OS swaps out least recently used page

2. **Low memory pressure:**
   - OS proactively swaps out unused pages
   - Keeps some RAM free for new allocations

3. **Process goes to sleep:**
   - OS may swap out process's pages
   - Frees RAM for active processes

### Performance Impact

**Swapping is slow!**

```
RAM access: 0.1 microseconds
Disk access: 5-10 milliseconds (50,000-100,000x slower!)
```

**If too much swapping (thrashing):**
- System becomes very slow
- Constant disk I/O
- CPU waits for disk
- User experience degrades

**Solution:**
- Add more RAM
- Reduce number of processes
- Close unused programs

### Summary

| Aspect | How It Works |
|--------|--------------|
| **Swap file** | Fixed-size file on disk (set at OS startup) |
| **Size** | Usually 1-2x RAM size |
| **When used** | When RAM is full |
| **Mechanism** | OS swaps pages to/from disk |
| **Performance** | Very slow (50,000-100,000x slower than RAM) |

**Key insight:** Virtual memory uses swap file on disk as "backup RAM". OS swaps pages between RAM and disk as needed. This allows processes to use more memory than physical RAM, but swapping is slow.
