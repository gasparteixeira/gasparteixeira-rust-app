// User State Module - Single Responsibility Principle
// Manages user form state and validation

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct UserFormState {
    pub name: String,
    pub email: String,
    pub password: String,
    pub editing_id: Option<i32>,
}

impl Default for UserFormState {
    fn default() -> Self {
        Self::new()
    }
}

impl UserFormState {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            password: String::new(),
            editing_id: None,
        }
    }

    pub fn with_values(
        name: String,
        email: String,
        password: String,
        editing_id: Option<i32>,
    ) -> Self {
        Self {
            name,
            email,
            password,
            editing_id,
        }
    }

    pub fn is_editing(&self) -> bool {
        self.editing_id.is_some()
    }

    pub fn is_valid(&self) -> bool {
        !self.name.trim().is_empty()
            && !self.email.trim().is_empty()
            && !self.password.trim().is_empty()
            && self.is_valid_email()
    }

    pub fn is_valid_email(&self) -> bool {
        self.email.contains('@') && self.email.len() > 3
    }

    pub fn reset(&mut self) {
        self.name.clear();
        self.email.clear();
        self.password.clear();
        self.editing_id = None;
    }

    pub fn set_for_editing(&mut self, id: i32, name: String, email: String, password: String) {
        self.name = name;
        self.email = email;
        self.password = password;
        self.editing_id = Some(id);
    }
}

// Hook for managing user form state
#[hook]
pub fn use_user_form_state() -> UseStateHandle<UserFormState> {
    use_state(UserFormState::default)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_form_state_new() {
        let state = UserFormState::new();
        assert_eq!(state.name, "");
        assert_eq!(state.email, "");
        assert_eq!(state.password, "");
        assert_eq!(state.editing_id, None);
    }

    #[test]
    fn test_user_form_state_default() {
        let state = UserFormState::default();
        assert_eq!(state.name, "");
        assert_eq!(state.email, "");
        assert_eq!(state.password, "");
        assert_eq!(state.editing_id, None);
    }

    #[test]
    fn test_user_form_state_with_values() {
        let state = UserFormState::with_values(
            "John".to_string(),
            "john@example.com".to_string(),
            "password123".to_string(),
            Some(1),
        );
        assert_eq!(state.name, "John");
        assert_eq!(state.email, "john@example.com");
        assert_eq!(state.password, "password123");
        assert_eq!(state.editing_id, Some(1));
    }

    #[test]
    fn test_is_editing() {
        let mut state = UserFormState::new();
        assert!(!state.is_editing());

        state.editing_id = Some(1);
        assert!(state.is_editing());
    }

    #[test]
    fn test_is_valid() {
        let mut state = UserFormState::new();
        assert!(!state.is_valid());

        state.name = "John".to_string();
        assert!(!state.is_valid());

        state.email = "john@example.com".to_string();
        assert!(!state.is_valid());

        state.password = "password123".to_string();
        assert!(state.is_valid());
    }

    #[test]
    fn test_is_valid_with_whitespace() {
        let mut state = UserFormState::new();
        state.name = "   ".to_string();
        state.email = "test@test.com".to_string();
        state.password = "password".to_string();
        assert!(!state.is_valid());
    }

    #[test]
    fn test_is_valid_email() {
        let mut state = UserFormState::new();

        state.email = "invalid".to_string();
        assert!(!state.is_valid_email());

        state.email = "a@b".to_string();
        assert!(!state.is_valid_email());

        state.email = "test@example.com".to_string();
        assert!(state.is_valid_email());
    }

    #[test]
    fn test_reset() {
        let mut state = UserFormState::with_values(
            "John".to_string(),
            "john@example.com".to_string(),
            "password123".to_string(),
            Some(1),
        );

        state.reset();
        assert_eq!(state.name, "");
        assert_eq!(state.email, "");
        assert_eq!(state.password, "");
        assert_eq!(state.editing_id, None);
    }

    #[test]
    fn test_set_for_editing() {
        let mut state = UserFormState::new();
        state.set_for_editing(
            5,
            "Jane".to_string(),
            "jane@example.com".to_string(),
            "password456".to_string(),
        );

        assert_eq!(state.name, "Jane");
        assert_eq!(state.email, "jane@example.com");
        assert_eq!(state.password, "password456");
        assert_eq!(state.editing_id, Some(5));
    }

    #[test]
    fn test_partial_eq() {
        let state1 = UserFormState::with_values(
            "John".to_string(),
            "john@example.com".to_string(),
            "password123".to_string(),
            Some(1),
        );
        let state2 = UserFormState::with_values(
            "John".to_string(),
            "john@example.com".to_string(),
            "password123".to_string(),
            Some(1),
        );
        let state3 = UserFormState::with_values(
            "Jane".to_string(),
            "jane@example.com".to_string(),
            "password456".to_string(),
            Some(1),
        );

        assert_eq!(state1, state2);
        assert_ne!(state1, state3);
    }

    #[test]
    fn test_clone() {
        let state = UserFormState::with_values(
            "John".to_string(),
            "john@example.com".to_string(),
            "password123".to_string(),
            Some(1),
        );
        let cloned = state.clone();
        assert_eq!(state, cloned);
    }
}
