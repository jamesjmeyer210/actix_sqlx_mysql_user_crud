use std::convert::TryFrom;
use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};
use argon2::password_hash::rand_core::RngCore;

pub(crate) struct Password([u8;32]);

impl TryFrom<&str> for Password {
    type Error = argon2::Error;

    fn try_from(value: &str) -> Result<Self,argon2::Error> {
        let argon2 = Argon2::default();
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);

        let mut bytes = [0u8; 32];
        argon2.hash_password_into(value.as_bytes(), &salt, &mut bytes)?;
        Ok(Password(bytes))
    }
}

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[cfg(test)]
mod password_test {
    use std::convert::TryFrom;
    use super::Password;

    #[test]
    fn try_from_test() {
        let password = Password::try_from("password123");
        assert!(password.is_ok());

        /*let other = Password::try_from("password123");
        assert!(password.is_ok());

        let password = password.unwrap();
        let other = other.unwrap();
        assert_eq!(password, other);*/
    }
}