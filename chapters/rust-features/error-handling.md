# Rust Error Handling Best Practices

## Your Python Pattern → Rust Equivalent

### Python Pattern:
```python
# Low level - no error handling, just propagate
def low_level():
    # Can raise exceptions
    pass

# Mid level - propagate
def mid_level():
    low_level()  # Exception propagates up

# Top level - catch and handle
def top_level():
    try:
        mid_level()
    except Exception as e:
        # Log to Tempo, show user message
        pass
```

### Rust Pattern:
```rust
// Low level - return Result, use ? to propagate
fn low_level() -> Result<T, Error> {
    // Can return error
    Ok(value)
}

// Mid level - propagate with ?
fn mid_level() -> Result<T, Error> {
    let value = low_level()?;  // Propagate error if it fails
    Ok(value)
}

// Top level - match to handle
fn top_level() {
    match mid_level() {
        Ok(value) => { /* success */ }
        Err(e) => { /* log to Tempo, show user */ }
    }
}
```

## Rules of Thumb

### ✅ DO return `Result` when:
- Function can fail (I/O, network, parsing, etc.)
- Error should be handled by caller
- You want to use `?` for propagation

### ❌ DON'T return `Result` when:
- Function cannot fail (pure computation)
- Error should be handled immediately (use `match` or `unwrap`)
- It's a panic situation (use `panic!` or `unwrap`)

### 🔄 Use `?` for:
- Propagating errors up the call stack
- Clean, readable error propagation
- Functions that return `Result`

### 🎯 Handle errors with `match` at:
- Top-level boundaries (main, handlers)
- User-facing boundaries (API endpoints, UI handlers)
- Where you need to log/instrument (Tempo, tracing)

## Example: Layered Architecture

```rust
// Layer 1: Low-level I/O (propagate errors)
fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

// Layer 2: Business logic (propagate errors)
fn process_data(path: &str) -> Result<ProcessedData, Box<dyn Error>> {
    let content = read_file(path)?;  // Propagate I/O errors
    // Process...
    Ok(processed)
}

// Layer 3: Top-level handler (catch and handle)
fn handle_user_action() {
    match process_data("data.txt") {
        Ok(data) => {
            // Success: update UI, log success
            println!("Success!");
        }
        Err(e) => {
            // Error: show user message, log to Tempo
            eprintln!("Error: {}", e);
            // Mark span as error in tracing
        }
    }
}
```

## With Tracing/OTel (like your decorators)

```rust
use tracing::{info, error, instrument};

#[instrument]  // Like your decorator - auto-instruments function
fn process_data(path: &str) -> Result<ProcessedData, Box<dyn Error>> {
    let content = read_file(path)?;
    // If error occurs, span is automatically marked as error
    Ok(processed)
}
```
