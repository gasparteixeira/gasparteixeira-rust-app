use crate::models::User;
use crate::repository::UserRepository;
use rocket::http::Status;
use rocket::response::status::Custom;
use std::sync::Arc;

/// UserService - Single Responsibility Principle
/// This service is only responsible for business logic related to users
/// It depends on UserRepository abstraction (Dependency Inversion Principle)
pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        UserService { repository }
    }

    /// Create a new user with validation
    pub async fn create_user(&self, user: User) -> Result<Vec<User>, Custom<String>> {
        // Validate user before creating
        user.validate().map_err(|e| Custom(Status::BadRequest, e))?;

        self.repository.create(&user).await?;
        self.get_all_users().await
    }

    /// Get all users
    pub async fn get_all_users(&self) -> Result<Vec<User>, Custom<String>> {
        self.repository.find_all().await
    }

    /// Update an existing user with validation
    pub async fn update_user(&self, id: i32, user: User) -> Result<Vec<User>, Custom<String>> {
        // Validate user before updating
        user.validate().map_err(|e| Custom(Status::BadRequest, e))?;

        self.repository.update(id, &user).await?;
        self.get_all_users().await
    }

    /// Delete a user
    pub async fn delete_user(&self, id: i32) -> Result<(), Custom<String>> {
        self.repository.delete(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::tests::MockUserRepository;

    fn create_test_service() -> UserService {
        let repo = Arc::new(MockUserRepository::new());
        UserService::new(repo)
    }

    #[tokio::test]
    async fn test_create_user_valid() {
        let service = create_test_service();
        let user = User::new("John Doe".to_string(), "john@example.com".to_string(), "password123".to_string());

        let result = service.create_user(user).await;
        assert!(result.is_ok());

        let users = result.unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].name, "John Doe");
    }

    #[tokio::test]
    async fn test_create_user_invalid_name() {
        let service = create_test_service();
        let user = User::new("".to_string(), "john@example.com".to_string(), "password123".to_string());

        let result = service.create_user(user).await;
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.0, Status::BadRequest);
        assert_eq!(err.1, "Name cannot be empty");
    }

    #[tokio::test]
    async fn test_create_user_invalid_email() {
        let service = create_test_service();
        let user = User::new("John Doe".to_string(), "invalid_email".to_string(), "password123".to_string());

        let result = service.create_user(user).await;
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.0, Status::BadRequest);
        assert_eq!(err.1, "Invalid email format");
    }

    #[tokio::test]
    async fn test_get_all_users() {
        let service = create_test_service();
        let user1 = User::new("John Doe".to_string(), "john@example.com".to_string(), "password123".to_string());
        let user2 = User::new("Jane Doe".to_string(), "jane@example.com".to_string(), "password456".to_string());

        service.create_user(user1).await.unwrap();
        service.create_user(user2).await.unwrap();

        let users = service.get_all_users().await.unwrap();
        assert_eq!(users.len(), 2);
    }

    #[tokio::test]
    async fn test_update_user_valid() {
        let service = create_test_service();
        let user = User::new("John Doe".to_string(), "john@example.com".to_string(), "password123".to_string());
        service.create_user(user).await.unwrap();

        let updated_user = User::new(
            "John Smith".to_string(),
            "johnsmith@example.com".to_string(),
            "newpassword123".to_string(),
        );
        let result = service.update_user(1, updated_user).await;
        assert!(result.is_ok());

        let users = result.unwrap();
        assert_eq!(users[0].name, "John Smith");
    }

    #[tokio::test]
    async fn test_update_user_invalid() {
        let service = create_test_service();
        let user = User::new("John Doe".to_string(), "john@example.com".to_string(), "password123".to_string());
        service.create_user(user).await.unwrap();

        let invalid_user = User::new("".to_string(), "john@example.com".to_string(), "password123".to_string());
        let result = service.update_user(1, invalid_user).await;
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.0, Status::BadRequest);
    }

    #[tokio::test]
    async fn test_delete_user() {
        let service = create_test_service();
        let user = User::new("John Doe".to_string(), "john@example.com".to_string(), "password123".to_string());
        service.create_user(user).await.unwrap();

        let result = service.delete_user(1).await;
        assert!(result.is_ok());

        let users = service.get_all_users().await.unwrap();
        assert_eq!(users.len(), 0);
    }

    #[tokio::test]
    async fn test_delete_nonexistent_user() {
        let service = create_test_service();

        let result = service.delete_user(999).await;
        assert!(result.is_err());
    }
}
