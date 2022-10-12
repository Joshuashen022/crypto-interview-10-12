use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};

use num::BigInt;
pub fn test() {
    let mut insecure = "Insecure Pa55w0rd".as_bytes();
    insecure = &(&mut insecure)[..16];
    println!("{:?}", insecure.len());
    let password = b"aes-128-ctr";

    let salt = BigInt::parse_bytes(b"5031a85957e7fdb47d6ece9d95adbc36", 16)
        .unwrap()
        .to_str_radix(10);

    let password_hash = Pbkdf2.hash_password(password, &salt).unwrap().to_string();

    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    assert!(Pbkdf2.verify_password(password, &parsed_hash).is_ok());
}
