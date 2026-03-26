# Contributing to TypeSearch

Thank you for your interest in contributing to TypeSearch!

## Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/typesearch.git
   cd typesearch
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test
   ```

4. Run the example:
   ```bash
   cargo run --example basic
   ```

## Project Structure

```
typesearch/
├── typesearch/           # Main crate
│   └── src/
│       ├── lib.rs       # Public API
│       ├── field.rs     # Field types
│       ├── schema.rs    # Schema generation
│       └── query.rs     # Query builder (future)
└── typesearch-derive/   # Proc macro crate
    └── src/
        ├── lib.rs       # Macro entry point
        ├── attrs.rs     # Attribute parsing
        └── expand.rs    # Code generation
```

## Testing

- Unit tests: `cargo test`
- Doc tests: `cargo test --doc`
- Example: `cargo run --example basic`

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix any warnings
- Add documentation for public APIs
- Add tests for new features

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and linters
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Ideas for Contributions

- [ ] Add nested object support
- [ ] Implement type-safe query builder
- [ ] Add Tantivy backend support
- [ ] Add more comprehensive tests
- [ ] Improve documentation
- [ ] Add more examples
- [ ] Schema migration tools

## Questions?

Feel free to open an issue for discussion!
