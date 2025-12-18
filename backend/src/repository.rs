use crate::models::User;
use async_trait::async_trait;
use rocket::http::Status;
use rocket::response::status::Custom;
use std::sync::Arc;
use tokio_postgres::Client;

/// Repository trait - Dependency Inversion Principle
/// High-level modules (service layer) depend on this abstraction, not on concrete implementations
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<(), Custom<String>>;
    async fn find_all(&self) -> Result<Vec<User>, Custom<String>>;
    async fn update(&self, id: i32, user: &User) -> Result<(), Custom<String>>;
    async fn delete(&self, id: i32) -> Result<(), Custom<String>>;
}

/// PostgreSQL implementation of UserRepository
/// This follows the Single Responsibility Principle - only handles database operations
pub struct PostgresUserRepository {
    client: Arc<Client>,
}

impl PostgresUserRepository {
    pub fn new(client: Arc<Client>) -> Self {
        PostgresUserRepository { client }
    }

    async fn execute_query(
        &self,
        query: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Custom<String>> {
        self.client
            .execute(query, params)
            .await
            .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &User) -> Result<(), Custom<String>> {
        self.execute_query(
            "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)",
            &[&user.name, &user.email, &user.password],
        )
        .await?;
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<User>, Custom<String>> {
        let users = self
            .client
            .query("SELECT id, name, email, password FROM users", &[])
            .await
            .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?
            .iter()
            .map(|row| User {
                id: Some(row.get(0)),
                name: row.get(1),
                email: row.get(2),
                password: row.get(3),
            })
            .collect::<Vec<User>>();

        Ok(users)
    }

    async fn update(&self, id: i32, user: &User) -> Result<(), Custom<String>> {
        self.execute_query(
            "UPDATE users SET name = $1, email = $2, password = $3 WHERE id = $4",
            &[&user.name, &user.email, &user.password, &id],
        )
        .await?;
        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), Custom<String>> {
        self.execute_query("DELETE FROM users WHERE id = $1", &[&id])
            .await?;
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::models::User;

    // Mock repository for testing - demonstrates Interface Segregation Principle
    pub struct MockUserRepository {
        pub users: std::sync::Mutex<Vec<User>>,
    }

    impl MockUserRepository {
        pub fn new() -> Self {
            MockUserRepository {
                users: std::sync::Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn create(&self, user: &User) -> Result<(), Custom<String>> {
            let mut users = self.users.lock().unwrap();
            let id = users.len() as i32 + 1;
            let mut new_user = user.clone();
            new_user.id = Some(id);
            users.push(new_user);
            Ok(())
        }

        async fn find_all(&self) -> Result<Vec<User>, Custom<String>> {
            let users = self.users.lock().unwrap();
            Ok(users.clone())
        }

        async fn update(&self, id: i32, user: &User) -> Result<(), Custom<String>> {
            let mut users = self.users.lock().unwrap();
            if let Some(existing_user) = users.iter_mut().find(|u| u.id == Some(id)) {
                existing_user.name = user.name.clone();
                existing_user.email = user.email.clone();
                existing_user.password = user.password.clone();
                Ok(())
            } else {
                Err(Custom(
                    Status::NotFound,
                    format!("User with id {} not found", id),
                ))
            }
        }

        async fn delete(&self, id: i32) -> Result<(), Custom<String>> {
            let mut users = self.users.lock().unwrap();
            if let Some(pos) = users.iter().position(|u| u.id == Some(id)) {
                users.remove(pos);
                Ok(())
            } else {
                Err(Custom(
                    Status::NotFound,
                    format!("User with id {} not found", id),
                ))
            }
        }
    }

    #[tokio::test]
    async fn test_mock_repository_create() {
        let repo = MockUserRepository::new();
        let user = User::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "password123".to_string(),
        );

        let result = repo.create(&user).await;
        assert!(result.is_ok());

        let users = repo.find_all().await.unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].name, "John Doe");
    }

    #[tokio::test]
    async fn test_mock_repository_find_all() {
        let repo = MockUserRepository::new();
        let user1 = User::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "password123".to_string(),
        );
        let user2 = User::new(
            "Jane Doe".to_string(),
            "jane@example.com".to_string(),
            "password456".to_string(),
        );

        repo.create(&user1).await.unwrap();
        repo.create(&user2).await.unwrap();

        let users = repo.find_all().await.unwrap();
        assert_eq!(users.len(), 2);
    }

    #[tokio::test]
    async fn test_mock_repository_update() {
        let repo = MockUserRepository::new();
        let user = User::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "password123".to_string(),
        );
        repo.create(&user).await.unwrap();

        let updated_user = User::new(
            "John Smith".to_string(),
            "johnsmith@example.com".to_string(),
            "newpassword123".to_string(),
        );
        let result = repo.update(1, &updated_user).await;
        assert!(result.is_ok());

        let users = repo.find_all().await.unwrap();
        assert_eq!(users[0].name, "John Smith");
        assert_eq!(users[0].email, "johnsmith@example.com");
        assert_eq!(users[0].password, "newpassword123");
    }

    #[tokio::test]
    async fn test_mock_repository_delete() {
        let repo = MockUserRepository::new();
        let user = User::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "password123".to_string(),
        );
        repo.create(&user).await.unwrap();

        let result = repo.delete(1).await;
        assert!(result.is_ok());

        let users = repo.find_all().await.unwrap();
        assert_eq!(users.len(), 0);
    }

    #[tokio::test]
    async fn test_mock_repository_update_nonexistent() {
        let repo = MockUserRepository::new();
        let user = User::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "password123".to_string(),
        );

        let result = repo.update(999, &user).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mock_repository_delete_nonexistent() {
        let repo = MockUserRepository::new();

        let result = repo.delete(999).await;
        assert!(result.is_err());
    }
}
