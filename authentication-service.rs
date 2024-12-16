use crate::models::{User, UserRole};
use thiserror::Error;
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum AuthenticationError {
    #[error("Invalid username or password")]
    InvalidCredentials,
    #[error("User not found")]
    UserNotFound,
    #[error("Insufficient permissions")]
    InsufficientPermissions,
}

pub struct AuthenticationService {
    users: HashMap<String, User>,
}

impl AuthenticationService {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        
        // Create default admin user
        let admin = User::new(
            "admin".to_string(), 
            "admin_password".to_string(), 
            UserRole::Admin
        );
        users.insert(admin.username.clone(), admin);

        AuthenticationService { users }
    }

    pub fn register_user(&mut self, username: String, password: String, role: UserRole) -> Result<(), AuthenticationError> {
        if self.users.contains_key(&username) {
            return Err(AuthenticationError::InvalidCredentials);
        }

        let new_user = User::new(username.clone(), password, role);
        self.users.insert(username, new_user);
        Ok(())
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<&User, AuthenticationError> {
        match self.users.get(username) {
            Some(user) if user.verify_password(password) => Ok(user),
            _ => Err(AuthenticationError::InvalidCredentials),
        }
    }

    pub fn check_permission(&self, username: &str, required_role: UserRole) -> Result<(), AuthenticationError> {
        match self.users.get(username) {
            Some(user) => {
                match (&user.role, required_role) {
                    (UserRole::Admin, _) => Ok(()),
                    (UserRole::Manager, UserRole::Staff) => Ok(()),
                    (role, required) if role == &required => Ok(()),
                    _ => Err(AuthenticationError::InsufficientPermissions),
                }
            },
            None => Err(AuthenticationError::UserNotFound),
        }
    }
}
