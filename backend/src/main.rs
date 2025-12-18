#[macro_use]
extern crate rocket;

mod db;
mod handlers;
mod models;
mod repository;
mod service;

use repository::PostgresUserRepository;
use rocket_cors::{AllowedOrigins, CorsOptions};
use service::UserService;
use std::sync::Arc;

/// Main entry point - follows Dependency Inversion Principle
/// Dependencies are injected from the outside, making the application flexible and testable
///
/// SOLID Principles Applied:
/// - Single Responsibility: Each module has one clear purpose
/// - Open/Closed: Easy to extend with new repositories or services without modifying existing code
/// - Liskov Substitution: MockUserRepository can replace PostgresUserRepository
/// - Interface Segregation: UserRepository interface is focused and minimal
/// - Dependency Inversion: High-level modules depend on abstractions (UserRepository trait)
#[launch]
async fn rocket() -> _ {
    // Initialize database (connection + schema)
    let client = db::init_database()
        .await
        .expect("Failed to initialize database");

    // Dependency injection - building the application from the inside out
    // Repository layer (data access)
    let repository = Arc::new(PostgresUserRepository::new(client));

    // Service layer (business logic)
    let service = Arc::new(UserService::new(repository));

    // CORS configuration
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .to_cors()
        .expect("Error while building CORS");

    // Build Rocket application with injected dependencies
    rocket::build()
        .manage(service)
        .mount(
            "/",
            routes![
                handlers::add_user,
                handlers::get_users,
                handlers::update_user,
                handlers::delete_user
            ],
        )
        .attach(cors)
}
