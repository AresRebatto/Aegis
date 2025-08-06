use argon2::{PasswordHash, PasswordVerifier, Argon2};
use super::super::engine::structs::derive_credentials::*;
    #[test]
    fn test_derive_credentials_success() {
        let password = "supersecurepassword123";
        let creds = DeriveCredentials::new(password).expect("Should derive credentials successfully");

        // Check if salt is not empty
        assert!(!creds.salt.is_empty(), "Salt should not be empty");

        // Check if password_hash is not empty
        assert!(!creds.password_hash.is_empty(), "Password hash should not be empty");

        // Check encryption key length
        assert_eq!(creds.encryption_key.len(), 32, "Encryption key should be 32 bytes");
    }

    #[test]
    fn test_password_verification() {
        let password = "mypassword";
        let creds = DeriveCredentials::new(password).unwrap();

        let parsed_hash = PasswordHash::new(&creds.password_hash)
            .expect("Should parse password hash");

        assert!(
            Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok(),
            "Password should verify correctly"
        );
    }

    #[test]
    fn test_different_salts_generate_different_hashes_and_keys() {
        let password = "samepassword";

        let creds1 = DeriveCredentials::new(password).unwrap();
        let creds2 = DeriveCredentials::new(password).unwrap();

        assert_ne!(creds1.salt, creds2.salt, "Salts should be different");
        assert_ne!(creds1.password_hash, creds2.password_hash, "Hashes should differ with different salts");
        assert_ne!(creds1.encryption_key, creds2.encryption_key, "Keys should differ with different salts");
    }

    #[test]
    fn test_empty_password() {
        let result = DeriveCredentials::new("");
        assert!(result.is_err(), "Empty password should return error");
    }
