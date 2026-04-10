use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};

fn main() {
    let password = std::env::args()
        .nth(1)
        .unwrap_or_else(|| {
            eprintln!("Usage: cargo run --bin hash_password -- <plain_password>");
            std::process::exit(1);
        });

    let salt = SaltString::encode_b64(uuid::Uuid::new_v4().as_bytes()).expect("failed to generate salt");
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("failed to hash password")
        .to_string();

    println!("{hash}");
}
