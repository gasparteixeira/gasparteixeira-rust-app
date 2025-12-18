use crate::models::User;
use crate::service::UserService;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use std::sync::Arc;

/// Handlers/Controllers - Single Responsibility Principle
/// These handlers are only responsible for HTTP request/response handling
/// They delegate business logic to the service layer

#[post("/api/users", data = "<user>")]
pub async fn add_user(
    service: &State<Arc<UserService>>,
    user: Json<User>,
) -> Result<Json<Vec<User>>, Custom<String>> {
    service.create_user(user.into_inner()).await.map(Json)
}

#[get("/api/users")]
pub async fn get_users(
    service: &State<Arc<UserService>>,
) -> Result<Json<Vec<User>>, Custom<String>> {
    service.get_all_users().await.map(Json)
}

#[put("/api/users/<id>", data = "<user>")]
pub async fn update_user(
    service: &State<Arc<UserService>>,
    id: i32,
    user: Json<User>,
) -> Result<Json<Vec<User>>, Custom<String>> {
    service.update_user(id, user.into_inner()).await.map(Json)
}

#[delete("/api/users/<id>")]
pub async fn delete_user(
    service: &State<Arc<UserService>>,
    id: i32,
) -> Result<Status, Custom<String>> {
    service.delete_user(id).await?;
    Ok(Status::NoContent)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::tests::MockUserRepository;
    use rocket::local::blocking::Client;
    use rocket::{Build, Rocket};

    fn rocket_with_mock_service() -> Rocket<Build> {
        let repo = Arc::new(MockUserRepository::new());
        let service = Arc::new(UserService::new(repo));

        rocket::build()
            .manage(service)
            .mount("/", routes![add_user, get_users, update_user, delete_user])
    }

    #[test]
    fn test_get_users_empty() {
        let client = Client::tracked(rocket_with_mock_service()).expect("valid rocket instance");
        let response = client.get("/api/users").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let users: Vec<User> = response.into_json().unwrap();
        assert_eq!(users.len(), 0);
    }

    #[test]
    fn test_add_user_valid() {
        let client = Client::tracked(rocket_with_mock_service()).expect("valid rocket instance");
        let user = User::new("John Doe".to_string(), "john@example.com".to_string(), "password123".to_string());

        let response = client
            .post("/api/users")
            .json(&user)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        let users: Vec<User> = response.into_json().unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].name, "John Doe");
    }

    #[test]
    fn test_add_user_invalid() {
        let client = Client::tracked(rocket_with_mock_service()).expect("valid rocket instance");
        let user = User::new("".to_string(), "john@example.com".to_string(), "password123".to_string());

        let response = client
            .post("/api/users")
            .json(&user)
            .dispatch();

        assert_eq!(response.status(), Status::BadRequest);
    }

    #[test]
    fn test_update_user() {
        let client = Client::tracked(rocket_with_mock_service()).expect("valid rocket instance");
        
        // First create a user
        let user = User::new("John Doe".to_string(), "john@example.com".to_string(), "password123".to_string());
        client.post("/api/users").json(&user).dispatch();

        // Then update it
        let updated_user = User::new("John Smith".to_string(), "johnsmith@example.com".to_string(), "newpassword123".to_string());
        let response = client
            .put("/api/users/1")
            .json(&updated_user)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        let users: Vec<User> = response.into_json().unwrap();
        assert_eq!(users[0].name, "John Smith");
    }

    #[test]
    fn test_delete_user() {
        let client = Client::tracked(rocket_with_mock_service()).expect("valid rocket instance");
        
        // First create a user
        let user = User::new("John Doe".to_string(), "john@example.com".to_string(), "password123".to_string());
        client.post("/api/users").json(&user).dispatch();

        // Then delete it
        let response = client.delete("/api/users/1").dispatch();
        assert_eq!(response.status(), Status::NoContent);

        // Verify it's deleted
        let response = client.get("/api/users").dispatch();
        let users: Vec<User> = response.into_json().unwrap();
        assert_eq!(users.len(), 0);
    }
}
