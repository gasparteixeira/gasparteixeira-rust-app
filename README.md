# gasparteixeira-rust-app

This project is an study of Rust as a web application. It will be a blog to post information

## Project Structure

```text
backend/
├── migrations/
├── src/
|   ├── db.rs           # Database config and schema setup
|   ├── main.rs         # Application entry point and dependency injection
|   ├── models.rs       # Domain models and business entities
|   ├── repository.rs   # Data access layer with trait abstraction
|   ├── service.rs      # Business logic layer
|   └── handlers.rs     # HTTP handlers/controllers
└── Cargo.toml          - Dependencies
frontend/
|── src/
│   ├── lib.rs          # Library exports
│   ├── main.rs         # Application entry point
│   ├── api.rs          # API client layer
│   ├── service.rs      # Business logic layer
│   ├── state.rs        # State management
│   └── components.rs   # UI components
└── tests/
|   └── integration_tests.rs  # Integration tests
├── index.html          # frontend bootstrap
└── Cargo.toml          - Dependencies
|── compose.yml         # Docker database config
└── README.md
```

## Running Tests

We need to be inside of backend or frontend folder before running those tests

```bash
# Run all tests
cargo test

# Run only library tests
cargo test --lib

# Run only integration tests
cargo test --test integration_tests

# Run with verbose output
cargo test -- --nocapture
```