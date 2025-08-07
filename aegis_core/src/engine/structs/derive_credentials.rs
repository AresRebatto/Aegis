use argon2::{password_hash, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use aes_gcm::{aead::AeadMutInPlace, Aes256Gcm, Key, KeyInit, Nonce};
use hkdf::Hkdf;
use rand::{rngs::OsRng, RngCore};
use sha2::{digest::block_buffer::Eager, Sha256};
use password_hash::{SaltString};
use super::super::errors::{
    auth::{LoginError},
    cryptograpy_errors::CryptographyError

};

//for local use
pub struct DeriveCredentials{
    pub password_hash: String,
    pub encryption_key: [u8; 32],
    pub salt: String
}

impl DeriveCredentials{
    pub fn new(master_pwd: &str)-> Result<Self,LoginError>{
        if master_pwd == ""{return Err(LoginError::EmptyPassword);}

        let salt = SaltString::generate(&mut OsRng);


        let pwd_hash = Self::hash_password(&salt, master_pwd)
                        .map_err(|_|LoginError::HashingError)?;
    
        let encryption_key: [u8; 32] = Self::generate_key(master_pwd, &salt)
                                            .map_err(|_| LoginError::GeneratingEnryptionKeyError)?;
        
        
        
        Ok(Self{
            password_hash: pwd_hash,
            encryption_key: encryption_key,
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

    fn create_chiper(encryption_key: &[u8; 32])-> Aes256Gcm{

        let key = Key::<Aes256Gcm>::from_slice(encryption_key);

        Aes256Gcm::new(key)
    }
                                                                            
    pub fn encrypt_text(
        encryption_key: &[u8; 32], 
        plain_text: &str
            // encr text  nonce
    )-> Result<(Vec<u8>, Vec<u8>),CryptographyError>{
        let mut chiper = Self::create_chiper(encryption_key);

        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);


        let mut buffer = plain_text.as_bytes().to_vec();

        chiper.encrypt_in_place(nonce, b"", &mut buffer)
            .map_err(|_| CryptographyError::EncryptionError)?;
        
        Ok((buffer, nonce_bytes.to_vec()))
    }
}