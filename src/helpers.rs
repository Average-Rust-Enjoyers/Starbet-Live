use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

/// For verification check example: <https://docs.rs/argon2/latest/argon2>
pub fn hash_password(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password, &salt)
        .expect("Password hashing failed")
        .to_string()
}
