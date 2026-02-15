# Constants vs Static Variables vs Structs: When to Use What

## Quick Answer

**Use `const` for simple compile-time constants. Use `static` for runtime global state. Use structs for complex data or when you need methods. You can also use `static` with structs for global state.**

---

## 1. Constants - Compile-Time Values

**What:** Values known at compile time, no runtime memory allocation

**When to use:**
- ✅ **Simple values** - Numbers, strings, simple data
- ✅ **Compile-time constants** - Array sizes, const generics, pattern matching
- ✅ **Immutable values** - Values that never change
- ✅ **No methods needed** - Just data

**Example:**
```rust
const GLOBAL_VALUE: i32 = 42;
const API_URL: &str = "https://api.example.com";
const MAX_CONNECTIONS: usize = 100;
```

**Advantages:**
- ✅ Zero runtime overhead
- ✅ Can be used in const contexts (array sizes, etc.)
- ✅ Evaluated at compile time

**Limitations:**
- ❌ Can't have methods
- ❌ Limited to simple data
- ❌ Hard to organize related data
- ❌ No memory address (can't take references)

---

## 1.5. Static Variables - Runtime Global State

**What:** Global variable that lives for entire program with runtime memory allocation

**When to use:**
- ✅ **Mutable global state** - Need to modify at runtime
- ✅ **Need a memory address** - When you need `&'static T`
- ✅ **Global variables** - When const isn't sufficient

**Example:**
```rust
static mut COUNTER: i32 = 0;
static GLOBAL_DATA: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());
```

**Advantages:**
- ✅ Has a memory address
- ✅ Can be mutable
- ✅ Thread-safe options available

**Limitations:**
- ❌ Runtime memory allocation
- ❌ More complex than const
- ❌ Can't be used in const contexts

---

## 2. Structs - Complex Data with Methods

**What:** Custom type with fields and methods

**When to use:**
- ✅ **Complex data** - Multiple related fields
- ✅ **Need methods** - Behavior associated with data
- ✅ **Organization** - Group related data together

**Example:**
```rust
struct Config {
    api_url: String,
    max_connections: usize,
    timeout: u64,
}

impl Config {
    fn new() -> Self {
        Config {
            api_url: String::from("https://api.example.com"),
            max_connections: 100,
            timeout: 30,
        }
    }
    
    fn get_api_url(&self) -> &str {
        &self.api_url
    }
}
```

**Advantages:**
- ✅ Can have methods
- ✅ Organize related data
- ✅ More flexible

**Limitations:**
- ❌ More complex
- ❌ Need to create instances

---

## 3. Static Structs - Best of Both Worlds

**What:** Global struct instance

**When to use:**
- ✅ **Complex global state** - Need struct with methods
- ✅ **Global configuration** - Complex config as struct
- ✅ **Singleton pattern** - One instance for entire program

**Example:**
```rust
use std::sync::OnceLock;

struct Config {
    api_url: String,
    max_connections: usize,
    timeout: u64,
}

impl Config {
    fn new() -> Self {
        Config {
            api_url: String::from("https://api.example.com"),
            max_connections: 100,
            timeout: 30,
        }
    }
    
    fn get_api_url(&self) -> &str {
        &self.api_url
    }
}

// Static struct instance
static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();

fn get_config() -> &'static Config {
    GLOBAL_CONFIG.get_or_init(|| Config::new())
}

fn main() {
    let config = get_config();
    println!("{}", config.get_api_url());
}
```

**Advantages:**
- ✅ Complex global state
- ✅ Can have methods
- ✅ Organized data
- ✅ Singleton pattern

**Limitations:**
- ⚠️ More complex than simple static
- ⚠️ Runtime initialization (if using OnceLock)

---

## 4. Comparison Table

| Approach | Use Case | Complexity | Methods | Runtime Memory | Example |
|----------|----------|------------|---------|----------------|---------|
| **const** | Compile-time constants | Simple | ❌ No | ❌ No | `const VALUE: i32 = 42;` |
| **static** | Runtime global state | Simple | ❌ No | ✅ Yes | `static VALUE: i32 = 42;` |
| **Struct** | Complex data | Medium | ✅ Yes | Instance-based | `struct Config { ... }` |
| **Static struct** | Global complex state | Complex | ✅ Yes | ✅ Yes | `static CONFIG: OnceLock<Config> = ...` |

---

## 5. When to Use What

### ✅ Use Constants When:

**Simple compile-time constants:**
```rust
const MAX_SIZE: usize = 1024;
const API_KEY: &str = "secret-key";
const DEBUG: bool = true;
```

**Configuration values:**
```rust
const PORT: u16 = 8080;
const HOST: &str = "localhost";
```

**Array sizes and const generics:**
```rust
const BUFFER_SIZE: usize = 1024;
let buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE]; // ✅ Works with const
```

**This is the simplest and most efficient approach for simple data!**

---

### ⚠️ Use Static Variables When:

**Mutable global state:**
```rust
static mut COUNTER: usize = 0;

// Unsafe access required
unsafe {
    COUNTER += 1;
}
```

**Global data that needs a memory address:**
```rust
static mut GLOBAL_VEC: Vec<i32> = Vec::new();

fn add_to_global(value: i32) {
    unsafe {
        GLOBAL_VEC.push(value); // Need address for mutation
    }
}
```

**Thread-safe global state:**
```rust
use std::sync::Mutex;
static GLOBAL_DATA: Mutex<Vec<i32>> = Mutex::new(Vec::new());
```

**Use static only when const isn't sufficient!**

---

### ✅ Use Structs When:

**Complex data with methods:**
```rust
struct User {
    name: String,
    age: u32,
    email: String,
}

impl User {
    fn new(name: String, age: u32, email: String) -> Self {
        User { name, age, email }
    }
    
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}
```

**Organizing related data:**
```rust
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}
```

**Use structs when you need methods or complex data!**

---

### ✅ Use Static Structs When:

**Global configuration:**
```rust
use std::sync::OnceLock;

struct AppConfig {
    database_url: String,
    redis_url: String,
    log_level: String,
}

impl AppConfig {
    fn load() -> Self {
        // Load from environment, file, etc.
        AppConfig {
            database_url: std::env::var("DATABASE_URL").unwrap(),
            redis_url: std::env::var("REDIS_URL").unwrap(),
            log_level: std::env::var("LOG_LEVEL").unwrap_or("info".to_string()),
        }
    }
}

static CONFIG: OnceLock<AppConfig> = OnceLock::new();

fn get_config() -> &'static AppConfig {
    CONFIG.get_or_init(|| AppConfig::load())
}
```

**Singleton pattern:**
```rust
use std::sync::OnceLock;

struct Logger {
    level: String,
}

impl Logger {
    fn new() -> Self {
        Logger {
            level: "info".to_string(),
        }
    }
    
    fn log(&self, message: &str) {
        println!("[{}] {}", self.level, message);
    }
}

static LOGGER: OnceLock<Logger> = OnceLock::new();

fn get_logger() -> &'static Logger {
    LOGGER.get_or_init(|| Logger::new())
}
```

**Use static structs for global complex state!**

---

## 6. Real-World Examples

### Example 1: Simple Configuration (Constants)

```rust
// Simple - use constants
const API_URL: &str = "https://api.example.com";
const TIMEOUT: u64 = 30;
const MAX_RETRIES: u32 = 3;

fn make_request() {
    // Use const values
    println!("Calling {}", API_URL);
}
```

**✅ Use const for simple constants!**

---

### Example 2: Complex Configuration (Static Struct)

```rust
use std::sync::OnceLock;

struct Config {
    api_url: String,
    timeout: u64,
    max_retries: u32,
    database_url: String,
    cache_size: usize,
}

impl Config {
    fn load() -> Self {
        Config {
            api_url: std::env::var("API_URL").unwrap(),
            timeout: 30,
            max_retries: 3,
            database_url: std::env::var("DATABASE_URL").unwrap(),
            cache_size: 1024,
        }
    }
    
    fn get_api_url(&self) -> &str {
        &self.api_url
    }
}

static CONFIG: OnceLock<Config> = OnceLock::new();

fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| Config::load())
}
```

**✅ Use this for complex global state!**

---

### Example 3: Regular Struct (Not Global)

```rust
struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: String, age: u32) -> Self {
        User { name, age }
    }
    
    fn greet(&self) {
        println!("Hello, I'm {}", self.name);
    }
}

fn main() {
    let user = User::new("Alice".to_string(), 30);
    user.greet();  // Create instances as needed
}
```

**✅ Use this for data that's not global!**

---

## 7. Summary

**Rule of thumb:**
1. **Simple constants** → Use `const VALUE: Type = value;`
2. **Mutable global state** → Use `static mut VALUE: Type = value;`
3. **Complex data with methods** → Use `struct` (create instances)
4. **Global complex state** → Use `static STRUCT: OnceLock<Struct> = ...`

**Your question:**
- `const GLOBAL_VALUE: i32 = 42;` → ✅ Use for simple constants
- `static mut COUNTER: i32 = 0;` → ✅ Use for mutable global state
- `struct Config { ... }` → ✅ Use for complex data with methods
- `static CONFIG: OnceLock<Config> = ...` → ✅ Use for global complex state

**Choose based on needs:**
- Compile-time constant? → const
- Mutable global variable? → static
- Complex data with methods? → Struct
- Global complex state? → Static struct

---

## 8. Common Patterns

### Pattern 1: Compile-Time Constant
```rust
const MAX_SIZE: usize = 1024;  // ✅ Zero overhead, compile-time
```

### Pattern 2: Mutable Global State
```rust
static mut COUNTER: usize = 0;  // ✅ Runtime global state
```

### Pattern 3: Global Struct Instance
```rust
use std::sync::OnceLock;

struct Config { ... }
static CONFIG: OnceLock<Config> = OnceLock::new();  // ✅ Complex global state
```

### Pattern 4: Regular Struct (Not Global)
```rust
struct User { ... }
let user = User::new(...);  // ✅ Create instances as needed
```

**Prefer const over static, static over global structs!**
