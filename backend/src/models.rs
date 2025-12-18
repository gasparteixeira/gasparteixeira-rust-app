use rocket::serde::{Deserialize, Serialize};

/// User domain model - Single Responsibility Principle
/// This struct is only responsible for representing a user entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        User {
            id: None,
            name,
            email,
            password,
        }
    }

    pub fn with_id(id: i32, name: String, email: String, password: String) -> Self {
        User {
            id: Some(id),
            name,
            email,
            password,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if self.email.trim().is_empty() {
            return Err("Email cannot be empty".to_string());
        }
        if !self.email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        if self.password.trim().is_empty() {
            return Err("Password cannot be empty".to_string());
        }
        if self.password.len() < 6 {
            return Err("Password must be at least 6 characters".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_user() {
        let user = User::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "password123".to_string(),
        );
        assert_eq!(user.id, None);
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
        assert_eq!(user.password, "password123");
    }

    #[test]
    fn test_user_with_id() {
        let user = User::with_id(
            1,
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "password123".to_string(),
        );
        assert_eq!(user.id, Some(1));
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
        assert_eq!(user.password, "password123");
    }

    #[test]
    fn test_validate_valid_user() {
        let user = User::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "password123".to_string(),
        );
        assert!(user.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_name() {
        let user = User::new(
            "".to_string(),
            "john@example.com".to_string(),
            "password123".to_string(),
        );
        assert!(user.validate().is_err());
        assert_eq!(user.validate().unwrap_err(), "Name cannot be empty");
    }

    #[test]
    fn test_validate_empty_email() {
        let user = User::new(
            "John Doe".to_string(),
            "".to_string(),
            "password123".to_string(),
        );
        assert!(user.validate().is_err());
        assert_eq!(user.validate().unwrap_err(), "Email cannot be empty");
    }

    #[test]
    fn test_validate_invalid_email() {
        let user = User::new(
            "John Doe".to_string(),
            "invalid_email".to_string(),
            "password123".to_string(),
        );
        assert!(user.validate().is_err());
        assert_eq!(user.validate().unwrap_err(), "Invalid email format");
    }

    #[test]
    fn test_validate_empty_password() {
        let user = User::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "".to_string(),
        );
        assert!(user.validate().is_err());
        assert_eq!(user.validate().unwrap_err(), "Password cannot be empty");
    }

    #[test]
    fn test_validate_short_password() {
        let user = User::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "12345".to_string(),
        );
        assert!(user.validate().is_err());
        assert_eq!(
            user.validate().unwrap_err(),
            "Password must be at least 6 characters"
        );
    }
}
