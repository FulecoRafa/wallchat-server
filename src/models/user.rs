use pbkdf2::password_hash::{SaltString, Error};
use serde::{Serialize, Deserialize};

use crate::util::crypto;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub name: String,
    pub password: String,
    pub profile_picture: Option<String>,
}

impl User {
    pub fn new(email: String, name: String, password: String) -> Result<User, Error> {
        let encrypted_pass = crypto::encrypt_password(&password)?;
        Ok(User {
            email,
            name,
            password: encrypted_pass,
            profile_picture: None,
        })
    }
}
