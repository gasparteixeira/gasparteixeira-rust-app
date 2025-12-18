// API Client Module - Single Responsibility Principle
// Handles all HTTP communication with the backend

use gloo::net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

const API_BASE_URL: &str = "http://127.0.0.1:8000/api";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateUserRequest {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

// Result type for API operations
pub type ApiResult<T> = Result<T, String>;

// Trait for API client (Dependency Inversion Principle)
pub trait UserApiClient {
    fn fetch_users(&self, callback: Callback<ApiResult<Vec<User>>>);
    fn create_user(&self, request: CreateUserRequest, callback: Callback<ApiResult<()>>);
    fn update_user(&self, request: UpdateUserRequest, callback: Callback<ApiResult<()>>);
    fn delete_user(&self, id: i32, callback: Callback<ApiResult<()>>);
}

// Concrete implementation of API client
#[derive(Clone)]
pub struct HttpUserApiClient {
    base_url: String,
}

impl HttpUserApiClient {
    pub fn new() -> Self {
        Self {
            base_url: API_BASE_URL.to_string(),
        }
    }

    #[cfg(test)]
    pub fn with_base_url(base_url: String) -> Self {
        Self { base_url }
    }
}

impl Default for HttpUserApiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl UserApiClient for HttpUserApiClient {
    fn fetch_users(&self, callback: Callback<ApiResult<Vec<User>>>) {
        let url = format!("{}/users", self.base_url);
        spawn_local(async move {
            match Request::get(&url).send().await {
                Ok(resp) if resp.ok() => {
                    match resp.json::<Vec<User>>().await {
                        Ok(users) => callback.emit(Ok(users)),
                        Err(_) => callback.emit(Err("Failed to parse users".to_string())),
                    }
                }
                Ok(_) => callback.emit(Err("Server returned an error".to_string())),
                Err(_) => callback.emit(Err("Failed to fetch users".to_string())),
            }
        });
    }

    fn create_user(&self, request: CreateUserRequest, callback: Callback<ApiResult<()>>) {
        let url = format!("{}/users", self.base_url);
        spawn_local(async move {
            let user_data = serde_json::json!({
                "name": request.name,
                "email": request.email,
                "password": request.password
            });

            match Request::post(&url)
                .header("Content-Type", "application/json")
                .body(user_data.to_string())
                .send()
                .await
            {
                Ok(resp) if resp.ok() => callback.emit(Ok(())),
                Ok(_) => callback.emit(Err("Failed to create user".to_string())),
                Err(_) => callback.emit(Err("Request failed".to_string())),
            }
        });
    }

    fn update_user(&self, request: UpdateUserRequest, callback: Callback<ApiResult<()>>) {
        let url = format!("{}/users/{}", self.base_url, request.id);
        spawn_local(async move {
            let user_data = serde_json::json!({
                "id": request.id,
                "name": request.name,
                "email": request.email,
                "password": request.password
            });
            
            match Request::put(&url)
                .header("Content-Type", "application/json")
                .body(user_data.to_string())
                .send()
                .await
            {
                Ok(resp) if resp.ok() => callback.emit(Ok(())),
                Ok(_) => callback.emit(Err("Failed to update user".to_string())),
                Err(_) => callback.emit(Err("Request failed".to_string())),
            }
        });
    }

    fn delete_user(&self, id: i32, callback: Callback<ApiResult<()>>) {
        let url = format!("{}/users/{}", self.base_url, id);
        spawn_local(async move {
            match Request::delete(&url).send().await {
                Ok(resp) if resp.ok() => callback.emit(Ok(())),
                Ok(_) => callback.emit(Err("Failed to delete user".to_string())),
                Err(_) => callback.emit(Err("Request failed".to_string())),
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User {
            id: 1,
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
    }

    #[test]
    fn test_user_clone() {
        let user = User {
            id: 1,
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };
        let cloned = user.clone();
        assert_eq!(user, cloned);
    }

    #[test]
    fn test_create_user_request() {
        let request = CreateUserRequest {
            name: "New User".to_string(),
            email: "new@example.com".to_string(),
            password: "password123".to_string(),
        };
        assert_eq!(request.name, "New User");
        assert_eq!(request.email, "new@example.com");
        assert_eq!(request.password, "password123");
    }

    #[test]
    fn test_update_user_request() {
        let request = UpdateUserRequest {
            id: 1,
            name: "Updated User".to_string(),
            email: "updated@example.com".to_string(),
            password: "newpassword".to_string(),
        };
        assert_eq!(request.id, 1);
        assert_eq!(request.name, "Updated User");
        assert_eq!(request.email, "updated@example.com");
        assert_eq!(request.password, "newpassword");
    }

    #[test]
    fn test_http_client_creation() {
        let client = HttpUserApiClient::new();
        assert_eq!(client.base_url, API_BASE_URL);
    }

    #[test]
    fn test_http_client_default() {
        let client = HttpUserApiClient::default();
        assert_eq!(client.base_url, API_BASE_URL);
    }

    #[test]
    fn test_http_client_with_base_url() {
        let custom_url = "http://localhost:3000/api".to_string();
        let client = HttpUserApiClient::with_base_url(custom_url.clone());
        assert_eq!(client.base_url, custom_url);
    }
}
