// Library exports for frontend crate
// This allows other modules and tests to access the internal modules

pub mod api;
pub mod components;
pub mod service;
pub mod state;

// Re-export commonly used types
pub use api::{
    ApiResult, CreateUserRequest, HttpUserApiClient, UpdateUserRequest, User, UserApiClient,
};
pub use components::{Button, UserForm, UserList, UserListItem};
pub use service::{DefaultUserService, UserService, UserServiceImpl};
pub use state::{use_user_form_state, UserFormState};

#[cfg(test)]
mod tests {
    #[test]
    fn test_lib_exports() {
        // Verify all exports are accessible
        use super::*;

        // This test ensures the library structure is correct
        let _state = UserFormState::new();
        let _client = HttpUserApiClient::new();
    }
}
