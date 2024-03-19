use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

use crate::auth::domain::password_hasher::{HashError, UserPasswordHasher};

#[derive(Clone, Default)]
pub struct ArgonHasher {
    pub argon2: Argon2<'static>,
}

impl ArgonHasher {
    pub fn new(argon2: Argon2<'static>) -> Self {
        Self { argon2 }
    }
}

impl UserPasswordHasher for ArgonHasher {
    fn hash(&self, password: &str) -> Result<String, HashError> {
        let salt = SaltString::generate(&mut OsRng);

        Ok(self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| HashError::InternalError(e.to_string()))?
            .to_string())
    }

    fn verify(&self, password: &str, hash: &str) -> Result<(), HashError> {
        let password_hash = PasswordHash::new(hash).map_err(|_| HashError::InvalidHash)?;
        let ok = self
            .argon2
            .verify_password(password.as_bytes(), &password_hash)
            .is_ok();

        if ok {
            Ok(())
        } else {
            Err(HashError::InvalidPassword)
        }
    }
}
