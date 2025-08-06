use argon2::{password_hash, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use aes_gcm::{Aes256Gcm, Key, KeyInit};
use hkdf::Hkdf;
use rand::{rngs::OsRng, RngCore};
use sha2::Sha256;
use password_hash::{Salt, SaltString};
use super::super::errors::auth::{LoginError};

//for local use
pub struct DeriveCredentials{
    pub password_hash: String,
    pub encryption_key: [u8; 32],
    pub salt: String
}

impl DeriveCredentials{
    pub fn new(master_pwd: &str)-> Result<Self,LoginError>{
        let mut salt = SaltString::generate(&mut OsRng);

        
        let argon2 = Argon2::default();

        let pwd_hash = Self::hash_password(&salt, master_pwd)
                        .map_err(|_|LoginError::HashingError)?;
    
        let enctyption_key: [u8; 32] = Self::generate_key(master_pwd, &salt)
                                            .map_err(|_| LoginError::GeneratingEnryptionKeyError)?;
        
        
        
        Ok(Self{
            password_hash: pwd_hash,
            encryption_key: enctyption_key,
            salt: salt.to_string()
        })
    }

    fn hash_password(salt: &SaltString, master_pwd: &str)-> Result<String, ()>{
        let argon2 = Argon2::default();
        match argon2
                .hash_password(master_pwd.as_bytes(), salt.as_salt()){
            Ok(hash_pwd)=> Ok(hash_pwd.to_string()),
            Err(_)=> Err(())
        }
                        
    }

    fn generate_key(master_pwd: &str, salt: &SaltString)-> Result<[u8; 32], ()>{
        
        let salt_bytes = salt.as_str().as_bytes();

        let hkdf = Hkdf::<Sha256>::new(Some(salt_bytes), master_pwd.as_bytes());

        let mut encription_key= [0u8; 32];


        if let Err(_) = hkdf.expand(b"encryption-key-v1", &mut encription_key){
            return Err(());
        }
            
        
        Ok(encription_key)

    }
}