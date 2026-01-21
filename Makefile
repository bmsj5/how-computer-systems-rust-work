# Computer Systems Rust - Educational Demo Runner
.PHONY: all run-all hardware memory compilation rust-features os advanced clean help

# Default target
all: help

# Run all educational demos in learning order
run-all: hardware memory compilation rust-features os advanced

# Hardware demonstrations
hardware:
	@echo "🖥️  Running Hardware Fundamentals Demos..."
	cd code && cargo run --bin hardware-fundamentals
	cd code && cargo run --bin cache-line-demo
	cd code && cargo run --bin register-demo

# Memory management demos
memory:
	@echo "🧠 Running Memory Management Demos..."
	cd code && cargo run --bin memory-management
	cd code && cargo run --bin memory-access-demo
	cd code && cargo run --bin array-indexing-demo

# Compilation and optimization demos
compilation:
	@echo "⚙️  Running Compilation & Optimization Demos..."
	cd code && cargo run --bin compilation-optimization
	cd code && cargo run --bin optimization-demo
	cd code && cargo run --bin optimization-levels-demo

# Rust language feature demos
rust-features:
	@echo "🦀 Running Rust Language Feature Demos..."
	cd code && cargo run --bin rust-language-features
	cd code && cargo run --bin iterator-demo
	cd code && cargo run --bin pointer-safety-demo

# Operating system concepts
os:
	@echo "💻 Running Operating System Demos..."
	cd code && cargo run --bin operating-system-concepts

# Advanced topics
advanced:
	@echo "🚀 Running Advanced Topic Demos..."
	cd code && cargo run --bin lru-implementation

# Run with release optimizations
release-%:
	cd code && cargo run --release --bin $*

# Profile a specific demo
profile-%:
	cd code && cargo build --release --bin $* && \
	valgrind --tool=callgrind --callgrind-out-file=callgrind.out ./target/release/$*

# Benchmark compilation
bench-compile:
	cd code && cargo build --release && \
	time cargo build --release

# Clean build artifacts
clean:
	cd code && cargo clean
	rm -f *.out *.prof callgrind.out.*

# Show available demos
list-demos:
	@echo "Available demos:"
	@cd code && cargo run --bin | grep -E "^\s*[a-zA-Z-]+$$" | sed 's/^/  /'

# Help target
help:
	@echo "Computer Systems Through Rust - Demo Runner"
	@echo ""
	@echo "Available targets:"
	@echo "  run-all          - Run all demos in learning order"
	@echo "  hardware         - CPU, registers, cache demos"
	@echo "  memory           - Memory management demos"
	@echo "  compilation      - Compiler optimization demos"
	@echo "  rust-features    - Rust language feature demos"
	@echo "  os              - Operating system concepts"
	@echo "  advanced        - Advanced topic demos"
	@echo "  release-<demo>  - Run specific demo with optimizations"
	@echo "  profile-<demo>  - Profile specific demo"
	@echo "  bench-compile   - Benchmark compilation time"
	@echo "  clean           - Clean build artifacts"
	@echo "  list-demos      - Show all available demos"
	@echo ""
	@echo "Examples:"
	@echo "  make hardware              # Run hardware demos"
	@echo "  make release-iterator-demo # Run iterator demo optimized"
	@echo "  make profile-cache-line-demo # Profile cache demo"