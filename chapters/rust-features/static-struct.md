# Static Variables vs Structs: When to Use What

## Quick Answer

**Use `static` for simple global constants. Use structs for complex data or when you need methods. You can also use `static` with structs for global state.**

---

## 1. Static Variables - Simple Global Constants

**What:** Global variable that lives for entire program

**When to use:**
- ✅ **Simple values** - Numbers, strings, simple data
- ✅ **Global constants** - Configuration, constants
- ✅ **No methods needed** - Just data

**Example:**
```rust
static GLOBAL_VALUE: i32 = 42;
static API_URL: &str = "https://api.example.com";
static MAX_CONNECTIONS: usize = 100;
```

**Advantages:**
- ✅ Simple
- ✅ Zero overhead
- ✅ Compile-time known (if const-like)

**Limitations:**
- ❌ Can't have methods
- ❌ Limited to simple data
- ❌ Hard to organize related data

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

| Approach | Use Case | Complexity | Methods | Example |
|----------|----------|------------|---------|---------|
| **Static variable** | Simple constants | Simple | ❌ No | `static VALUE: i32 = 42;` |
| **Struct** | Complex data | Medium | ✅ Yes | `struct Config { ... }` |
| **Static struct** | Global complex state | Complex | ✅ Yes | `static CONFIG: OnceLock<Config> = ...` |

---

## 5. When to Use What

### ✅ Use Static Variables When:

**Simple global constants:**
```rust
static MAX_SIZE: usize = 1024;
static API_KEY: &str = "secret-key";
static DEBUG: bool = true;
```

**Configuration values:**
```rust
static PORT: u16 = 8080;
static HOST: &str = "localhost";
```

**This is the simplest approach for simple data!**

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

### Example 1: Simple Configuration (Static Variables)

```rust
// Simple - use static variables
static API_URL: &str = "https://api.example.com";
static TIMEOUT: u64 = 30;
static MAX_RETRIES: u32 = 3;

fn make_request() {
    // Use static values
    println!("Calling {}", API_URL);
}
```

**✅ Use this for simple constants!**

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
1. **Simple constants** → Use `static VALUE: Type = value;`
2. **Complex data with methods** → Use `struct` (create instances)
3. **Global complex state** → Use `static STRUCT: OnceLock<Struct> = ...`

**Your question:**
- `static GLOBAL_VALUE: i32 = 42;` → ✅ Use for simple constants
- `struct Config { ... }` → ✅ Use for complex data with methods
- `static CONFIG: OnceLock<Config> = ...` → ✅ Use for global complex state

**Choose based on complexity:**
- Simple value? → Static variable
- Complex data? → Struct
- Global complex state? → Static struct

---

## 8. Common Patterns

### Pattern 1: Simple Global Constant
```rust
static MAX_SIZE: usize = 1024;  // ✅ Simple
```

### Pattern 2: Global Struct Instance
```rust
use std::sync::OnceLock;

struct Config { ... }
static CONFIG: OnceLock<Config> = OnceLock::new();  // ✅ Complex
```

### Pattern 3: Regular Struct (Not Global)
```rust
struct User { ... }
let user = User::new(...);  // ✅ Create instances
```

**Choose the simplest approach that works for your use case!**
