use pbkdf2::password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString, Error, PasswordHash};
use pbkdf2::Pbkdf2;

pub fn encrypt_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Pbkdf2.hash_password(password.as_bytes(), &salt)?.to_string())
}

pub fn validate_password(password: &str, hash: &PasswordHash) -> bool {
    Pbkdf2.verify_password(password.as_bytes(), &hash).is_ok()
}
