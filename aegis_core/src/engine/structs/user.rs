use std::{ fs::{self, create_dir, File}, path::Path};
use serde::{Serialize, Deserialize};
use super::super::errors::auth::{LoginError};
use argon2::{Argon2, PasswordHasher, PasswordVerifier, PasswordHash};
use aes_gcm::{Aes256Gcm, Key, KeyInit};
use hkdf::Hkdf;
use rand::{rngs::OsRng, RngCore};
use sha2::Sha256;
use zeroize::{Zeroize, ZeroizeOnDrop};
use std::fmt;
use super::login::Login;

#[derive(Serialize, Deserialize)]
pub struct User{
    pub id: i32,
    pub name: String,
    pub pwd: String, //hash
    pub salt: [u8; 32] //usefull for a master password
}

impl User{

    //Take all users from json
    pub fn get_users()-> Result<Vec<User>, String>{
        let path = Path::new("data/users.json");
        if Path::exists(&path){
            let data = fs::read_to_string(path);
            let users: Vec<User> = serde_json::from_str(&(data.unwrap())).unwrap_or(vec![]);

            return Ok(users);
        }else{
            if let Err(e) = create_dir("data"){
                return Err(format!("It wasn't possible to create dir: {}", e));
            }

            if let Err(e) = File::create_new(&path){
                return Err(format!("It wasn't possible to create file: {}", e));
            }

            return Ok(vec![]);
        }

        
    }

    pub fn login_new_user(name: String, pwd: String)-> Result<(), LoginError>{

        todo!()
    }

    pub fn login_user(name: String, pwd: String)-> Result<Vec<Login>, LoginError>{
        todo!()
    }
}