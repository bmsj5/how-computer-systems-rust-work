# LRU Implementation: How It Works

## Question 1: How is LRU Implemented?

**Your question:** "Can this be built as some vector, where you add pages at the top every time, or OS counts how many times some pages were accessed, stores it in map and then goes through the map, finding the least counter and deletes the key? Is there a more reliable, faster and preferable way?"

**Answer:** **OS uses a more efficient approach: doubly-linked list + hash map (O(1) operations).**

### Your Proposed Approaches

**Approach 1: Vector (add at top)**
```rust
let mut pages: Vec<Page> = Vec::new();

// On access: Move to top
pages.remove(page_index);
pages.insert(0, page);

// Evict: Remove from bottom
pages.pop();  // Least recently used
```

**Problem:**
- `remove()` and `insert()` are O(n) operations
- Slow for large number of pages
- Not efficient

**Approach 2: Map with counters**
```rust
let mut access_count: HashMap<Page, usize> = HashMap::new();

// On access: Increment counter
*access_count.entry(page).or_insert(0) += 1;

// Evict: Find minimum counter
let least_used = access_count.iter()
    .min_by_key(|(_, &count)| count)
    .unwrap();
```

**Problem:**
- Finding minimum is O(n)
- Doesn't track "recently" - only counts total accesses
- Not true LRU (Least Recently Used, not Least Frequently Used)

### How OS Actually Implements LRU

**OS uses: Doubly-linked list + Hash map (O(1) operations)**

**Data structure:**
```rust
struct LRUCache {
    // Doubly-linked list: Most recent → Least recent
    list: LinkedList<Page>,
    
    // Hash map: Page → Node in list
    map: HashMap<Page, *mut ListNode>,
}

struct ListNode {
    page: Page,
    prev: *mut ListNode,
    next: *mut ListNode,
}
```

**Operations:**

**1. On page access (O(1)):**
```rust
fn access_page(&mut self, page: Page) {
    if let Some(node_ptr) = self.map.get(&page) {
        // Page already in cache
        // Move to front of list (most recent)
        self.move_to_front(*node_ptr);
    } else {
        // Page not in cache
        // Add to front
        let node = self.add_to_front(page);
        self.map.insert(page, node);
    }
}
```

**2. Evict least recently used (O(1)):**
```rust
fn evict_lru(&mut self) -> Page {
    // Remove from back of list (least recent)
    let lru_page = self.remove_from_back();
    self.map.remove(&lru_page);
    lru_page
}
```

**Why this is fast:**
- Access: O(1) (hash map lookup + list move)
- Evict: O(1) (remove from back of list)
- No searching, no counting

### Visual: LRU Cache

```
Doubly-linked list (most recent → least recent):
┌─────┐    ┌─────┐    ┌─────┐    ┌─────┐
│PageA│←──│PageB│←──│PageC│←──│PageD│
│(MRU)│   │     │   │     │   │(LRU)│
└─────┘    └─────┘    └─────┘    └─────┘
  ↑                                    ↑
Most recent                    Least recent

Hash map:
PageA → pointer to PageA node
PageB → pointer to PageB node
PageC → pointer to PageC node
PageD → pointer to PageD node

On access to PageC:
  1. Find PageC in hash map (O(1))
  2. Move PageC to front (O(1))
  3. Update pointers (O(1))

On evict:
  1. Remove PageD from back (O(1))
  2. Remove from hash map (O(1))
```

### General Purpose LRU Implementation

**Rust example (simplified):**
```rust
use std::collections::HashMap;
use std::ptr;

struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, *mut Node<K, V>>,
    head: *mut Node<K, V>,
    tail: *mut Node<K, V>,
}

struct Node<K, V> {
    key: K,
    value: V,
    prev: *mut Node<K, V>,
    next: *mut Node<K, V>,
}

impl<K, V> LRUCache<K, V> 
where
    K: std::hash::Hash + Eq + Clone,
{
    fn new(capacity: usize) -> Self {
        // Initialize with dummy head/tail nodes
        // ...
    }
    
    fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(node_ptr) = self.map.get(key) {
            // Move to front
            self.move_to_front(*node_ptr);
            unsafe { Some(&(*node_ptr).value) }
        } else {
            None
        }
    }
    
    fn put(&mut self, key: K, value: V) {
        if let Some(node_ptr) = self.map.get(&key) {
            // Update existing
            unsafe { (*node_ptr).value = value; }
            self.move_to_front(*node_ptr);
        } else {
            // Add new
            if self.map.len() >= self.capacity {
                // Evict LRU
                self.evict_lru();
            }
            let node = self.add_to_front(key.clone(), value);
            self.map.insert(key, node);
        }
    }
    
    fn move_to_front(&mut self, node: *mut Node<K, V>) {
        // Remove from current position
        // Add to front
        // Update pointers
    }
    
    fn evict_lru(&mut self) {
        // Remove from tail
        // Remove from map
    }
}
```

**Why this is preferred:**
- O(1) for all operations
- No searching
- No counting
- Efficient for large caches

### Alternative: Clock Algorithm (Approximate LRU)

**Some OSes use Clock algorithm (faster, approximate):**

**How it works:**
- Circular buffer of pages
- Each page has "reference bit"
- Clock hand sweeps around
- If reference bit = 0 → evict
- If reference bit = 1 → set to 0, continue

**Advantages:**
- Simpler implementation
- O(1) per operation
- Good enough approximation

**Disadvantages:**
- Not true LRU (approximate)
- May evict recently used pages

### Summary

| Approach | Time Complexity | Notes |
|----------|----------------|-------|
| **Vector (add at top)** | O(n) | Slow, not used |
| **Map with counters** | O(n) for evict | Not true LRU |
| **Doubly-linked list + Hash map** | O(1) | **OS uses this!** |
| **Clock algorithm** | O(1) | Approximate, simpler |

**Key insight:** OS uses doubly-linked list + hash map for O(1) operations. This is the standard, efficient LRU implementation.
