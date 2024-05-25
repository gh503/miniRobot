use argon2::{self, password_hash::{SaltString, PasswordHasher, PasswordVerifier, PasswordHash, rand_core::OsRng}};
use std::error::Error;
use std::fmt;

// 定义一个新的错误类型来封装argon2::password_hash::Error
#[derive(Debug)]
struct Argon2Error(argon2::password_hash::Error);

impl fmt::Display for Argon2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Argon2 error: {}", self.0)
    }
}

impl Error for Argon2Error {}

pub struct Authenticator {
    username: String,
    password_hash: String,
}

impl Authenticator {
    pub fn new(username: String, password: String) -> Result<Self, Box<dyn Error>> {
        let password_hash = Self::hash_password(&password)?;
        Ok(Authenticator { username, password_hash })
    }

    pub fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = argon2::Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| Box::new(Argon2Error(e)) as Box<dyn Error>)?
            .to_string();
        Ok(password_hash)
    }

    pub fn verify_password(hash: &str, password: &str) -> Result<bool, Box<dyn Error>> {
        let argon2 = argon2::Argon2::default();
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| Box::new(Argon2Error(e)) as Box<dyn Error>)?;
        let is_valid = argon2.verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|e| Box::new(Argon2Error(e)) as Box<dyn Error>)
            .is_ok();
        Ok(is_valid)
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<bool, Box<dyn Error>> {
        if self.username == username {
            let is_valid = Self::verify_password(&self.password_hash, password)?;
            Ok(is_valid)
        } else {
            Ok(false)
        }
    }
}
