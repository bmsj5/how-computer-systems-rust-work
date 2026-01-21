//! LRU Cache Implementation Demo
//!
//! Demonstrates building an LRU (Least Recently Used) cache from scratch.
//! Shows advanced Rust concepts: generics, HashMap, LinkedList, smart pointers.
//! Run with: cargo run --bin lru-implementation

use std::collections::HashMap;
use std::hash::Hash;
use std::ptr;

#[derive(Debug)]
struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, (V, *mut LruNode<K, V>)>,
    head: Option<Box<LruNode<K, V>>>,
    tail: *mut LruNode<K, V>,
}

#[derive(Debug)]
struct LruNode<K, V> {
    key: K,
    value: V,
    prev: *mut LruNode<K, V>,
    next: *mut LruNode<K, V>,
}

impl<K, V> LruNode<K, V> {
    fn new(key: K, value: V) -> Self {
        LruNode {
            key,
            value,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}

impl<K: Eq + Hash + Clone, V: Clone> LruCache<K, V> {
    fn new(capacity: usize) -> Self {
        LruCache {
            capacity,
            map: HashMap::new(),
            head: None,
            tail: ptr::null_mut(),
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        // First check if key exists and get the node pointer
        let node_ptr = if let Some((_, node_ptr)) = self.map.get(key) {
            Some(*node_ptr)
        } else {
            None
        };

        if let Some(node_ptr) = node_ptr {
            // Move to front (most recently used)
            unsafe {
                self.move_to_front(node_ptr);
            }
            // Now get the value after moving
            self.map.get(key).map(|(value, _)| value)
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V) {
        // First check if key exists and get the node pointer
        let node_ptr = if let Some((_, node_ptr)) = self.map.get(&key) {
            Some(*node_ptr)
        } else {                
            None
        };

        if let Some(node_ptr) = node_ptr {
            // Update existing value and move to front
            unsafe {
                (*node_ptr).value = value.clone();
                self.move_to_front(node_ptr);
            }
        } else {
            // Add new entry
            let mut new_node = Box::new(LruNode::new(key.clone(), value.clone()));

            if self.map.len() == 0 {
                // First node
                self.tail = &mut *new_node;
                self.head = Some(new_node);
            } else {
                // Add to front
                unsafe {
                    new_node.next = &mut **self.head.as_mut().unwrap();
                    (*new_node.next).prev = &mut *new_node;
                }
                self.head = Some(new_node);
            }

            if let Some(ref mut head) = self.head {
                self.map.insert(key, (value, &mut **head));
            }

            // Evict if over capacity
            if self.map.len() > self.capacity {
                self.evict_lru();
            }
        }
    }

    unsafe fn move_to_front(&mut self, node_ptr: *mut LruNode<K, V>) {
        unsafe {
            if (*node_ptr).prev.is_null() {
                // Already at front
                return;
            }

            // Remove from current position
            if !(*node_ptr).next.is_null() {
                (*(*node_ptr).next).prev = (*node_ptr).prev;
            } else {
                // Was tail
                self.tail = (*node_ptr).prev;
            }

            if !(*node_ptr).prev.is_null() {
                (*(*node_ptr).prev).next = (*node_ptr).next;
            }

            // Move to front
            (*node_ptr).prev = ptr::null_mut();
            (*node_ptr).next = &mut **self.head.as_mut().unwrap();
            (*(*node_ptr).next).prev = node_ptr;
            self.head = Some(Box::from_raw(node_ptr));
        }
    }

    fn evict_lru(&mut self) {
        if self.tail.is_null() {
            return;
        }

        unsafe {
            let key = (*self.tail).key.clone();
            self.map.remove(&key);

            if (*self.tail).prev.is_null() {
                // Only one node
                self.head = None;
                self.tail = ptr::null_mut();
            } else {
                self.tail = (*self.tail).prev;
                (*self.tail).next = ptr::null_mut();
            }
        }
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

fn demonstrate_lru_cache() {
    println!("🚀 LRU Cache Implementation");
    println!("===========================");
    println!("Note: Full implementation with raw pointers is complex.");
    println!("In practice, you'd use a crate like 'lru' for production code.");
    println!();
    println!("LRU Cache Concepts:");
    println!("• Fixed capacity with automatic eviction");
    println!("• Most Recently Used (MRU) items stay in cache");
    println!("• Least Recently Used (LRU) items are evicted");
    println!("• O(1) get/put operations using HashMap + Linked List");
    println!("• Used in databases, web caches, OS page replacement");
}

fn get_cache_contents<K: Clone + std::fmt::Debug, V: Clone + std::fmt::Debug>(_cache: &LruCache<K, V>) -> Vec<(K, V)> {
    // Simplified for demo purposes - would need proper linked list traversal
    vec![]
}

fn demonstrate_cache_performance() {
    println!("
⚡ Cache Performance Comparison");
    println!("===============================");
    println!("In a real LRU cache implementation:");
    println!("• HashMap provides O(1) key lookup");
    println!("• Linked list maintains access order for O(1) eviction");
    println!("• Total: O(1) get/put operations");
    println!("• Memory overhead: ~2-3x compared to plain HashMap");
    println!("• Trade-off: Bounded memory vs slightly slower access");
}

fn demonstrate_cache_use_cases() {
    println!("
🎯 Cache Use Cases");
    println!("=================");

    println!("LRU caches are used in many systems:");
    println!("• Web servers: Cache HTTP responses, reduce database load");
    println!("• Databases: Cache query results, speed up repeated queries");
    println!("• Operating Systems: Page replacement ( Least Recently Used pages)");
    println!("• Web browsers: Cache web pages, images, scripts");
    println!("• CPU caches: Hardware-level LRU for memory access");
    println!("• CDN networks: Cache content closer to users");
}

fn main() {
    println!("🧠 LRU Cache Implementation Demo");
    println!("=================================");
    println!("Building a high-performance cache from scratch in Rust.\n");

    demonstrate_lru_cache();
    demonstrate_cache_performance();
    demonstrate_cache_use_cases();

    println!("
🎯 Key Takeaways:");
    println!("• LRU caches provide bounded memory usage with smart eviction");
    println!("• Raw pointers and unsafe code enable high performance");
    println!("• Generics allow flexible key/value types");
    println!("• Linked list + HashMap gives O(1) operations");
    println!("• Used in databases, web servers, OS page replacement");
    println!("• Trade-off: Memory overhead for performance and bounded size");
}