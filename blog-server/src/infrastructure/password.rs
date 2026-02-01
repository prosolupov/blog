use crate::domain::error::BlogError;
use argon2::Argon2;
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};

pub fn password_hash(password: &str) -> Result<String, BlogError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();

    argon
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| BlogError::Internal)
}

pub async fn verify_password(password: &str, hash: &str) -> bool {
    if let Ok(parsed_hash) = PasswordHash::new(hash) {
        return Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();
    }
    false
}
