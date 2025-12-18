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
|── compose.yml
└── README.md
```
