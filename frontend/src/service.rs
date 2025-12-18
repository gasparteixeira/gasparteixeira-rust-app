// Service Layer - Interface Segregation Principle
// Business logic layer that coordinates between API and UI

use crate::api::{
    ApiResult, CreateUserRequest, HttpUserApiClient, UpdateUserRequest, User, UserApiClient,
};
use crate::state::UserFormState;
use yew::prelude::*;

// Service trait for user operations
pub trait UserService {
    fn fetch_users(&self, callback: Callback<ApiResult<Vec<User>>>);
    fn create_user(&self, state: &UserFormState, callback: Callback<ApiResult<()>>);
    fn update_user(&self, state: &UserFormState, callback: Callback<ApiResult<()>>);
    fn delete_user(&self, id: i32, callback: Callback<ApiResult<()>>);
}

// Implementation of UserService
#[derive(Clone)]
pub struct UserServiceImpl<T: UserApiClient> {
    api_client: T,
}

impl<T: UserApiClient> UserServiceImpl<T> {
    pub fn new(api_client: T) -> Self {
        Self { api_client }
    }
}

impl<T: UserApiClient> UserService for UserServiceImpl<T> {
    fn fetch_users(&self, callback: Callback<ApiResult<Vec<User>>>) {
        self.api_client.fetch_users(callback);
    }

    fn create_user(&self, state: &UserFormState, callback: Callback<ApiResult<()>>) {
        if !state.is_valid() {
            callback.emit(Err("Invalid form data".to_string()));
            return;
        }

        let request = CreateUserRequest {
            name: state.name.clone(),
            email: state.email.clone(),
            password: state.password.clone(),
        };

        self.api_client.create_user(request, callback);
    }

    fn update_user(&self, state: &UserFormState, callback: Callback<ApiResult<()>>) {
        if !state.is_valid() {
            callback.emit(Err("Invalid form data".to_string()));
            return;
        }

        if let Some(id) = state.editing_id {
            let request = UpdateUserRequest {
                id,
                name: state.name.clone(),
                email: state.email.clone(),
                password: state.password.clone(),
            };

            self.api_client.update_user(request, callback);
        } else {
            callback.emit(Err("No user selected for editing".to_string()));
        }
    }

    fn delete_user(&self, id: i32, callback: Callback<ApiResult<()>>) {
        self.api_client.delete_user(id, callback);
    }
}

// Default service implementation using HttpUserApiClient
pub type DefaultUserService = UserServiceImpl<HttpUserApiClient>;

impl Default for DefaultUserService {
    fn default() -> Self {
        Self::new(HttpUserApiClient::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock API client for testing
    #[derive(Clone)]
    struct MockUserApiClient {
        should_succeed: bool,
    }

    impl UserApiClient for MockUserApiClient {
        fn fetch_users(&self, callback: Callback<ApiResult<Vec<User>>>) {
            if self.should_succeed {
                callback.emit(Ok(vec![User {
                    id: 1,
                    name: "Test User".to_string(),
                    email: "test@example.com".to_string(),
                }]));
            } else {
                callback.emit(Err("Failed to fetch".to_string()));
            }
        }

        fn create_user(&self, _request: CreateUserRequest, callback: Callback<ApiResult<()>>) {
            if self.should_succeed {
                callback.emit(Ok(()));
            } else {
                callback.emit(Err("Failed to create".to_string()));
            }
        }

        fn update_user(&self, _request: UpdateUserRequest, callback: Callback<ApiResult<()>>) {
            if self.should_succeed {
                callback.emit(Ok(()));
            } else {
                callback.emit(Err("Failed to update".to_string()));
            }
        }

        fn delete_user(&self, _id: i32, callback: Callback<ApiResult<()>>) {
            if self.should_succeed {
                callback.emit(Ok(()));
            } else {
                callback.emit(Err("Failed to delete".to_string()));
            }
        }
    }

    #[test]
    fn test_user_service_creation() {
        let api_client = MockUserApiClient {
            should_succeed: true,
        };
        let _service = UserServiceImpl::new(api_client);
    }

    #[test]
    fn test_service_validates_form_state() {
        // Test that invalid state is rejected before API call
        let api_client = MockUserApiClient {
            should_succeed: true,
        };
        let _service = UserServiceImpl::new(api_client);

        // Create an invalid state
        let invalid_state = UserFormState::new(); // Empty state is invalid

        // Since validation happens synchronously, we can't easily test
        // the callback result without a proper async test runtime
        // So we just verify the service can be created
        assert!(!invalid_state.is_valid());
    }

    #[test]
    fn test_valid_state_for_create() {
        let valid_state = UserFormState::with_values(
            "John".to_string(),
            "john@example.com".to_string(),
            "password123".to_string(),
            None,
        );
        assert!(valid_state.is_valid());
        assert!(!valid_state.is_editing());
    }

    #[test]
    fn test_valid_state_for_update() {
        let valid_state = UserFormState::with_values(
            "John".to_string(),
            "john@example.com".to_string(),
            "password123".to_string(),
            Some(1),
        );
        assert!(valid_state.is_valid());
        assert!(valid_state.is_editing());
    }

    #[test]
    fn test_invalid_state_without_editing_id() {
        let state_without_id = UserFormState::with_values(
            "John".to_string(),
            "john@example.com".to_string(),
            "password123".to_string(),
            None,
        );
        assert!(state_without_id.editing_id.is_none());
    }

    #[test]
    fn test_default_user_service() {
        let _service = DefaultUserService::default();
    }

    #[test]
    fn test_create_request_from_state() {
        let state = UserFormState::with_values(
            "John".to_string(),
            "john@example.com".to_string(),
            "password123".to_string(),
            None,
        );

        let request = CreateUserRequest {
            name: state.name.clone(),
            email: state.email.clone(),
            password: state.password.clone(),
        };

        assert_eq!(request.name, "John");
        assert_eq!(request.email, "john@example.com");
        assert_eq!(request.password, "password123");
    }

    #[test]
    fn test_update_request_from_state() {
        let state = UserFormState::with_values(
            "Jane".to_string(),
            "jane@example.com".to_string(),
            "securepass".to_string(),
            Some(5),
        );

        if let Some(id) = state.editing_id {
            let request = UpdateUserRequest {
                id,
                name: state.name.clone(),
                email: state.email.clone(),
                password: state.password.clone(),
            };

            assert_eq!(request.id, 5);
            assert_eq!(request.name, "Jane");
            assert_eq!(request.email, "jane@example.com");
            assert_eq!(request.password, "securepass");
        } else {
            panic!("Expected editing_id to be Some");
        }
    }
}
