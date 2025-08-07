use std::{ fs::{self, create_dir, File}, path::Path};
use serde::{Serialize, Deserialize};
use argon2::{Argon2, PasswordHasher, PasswordVerifier, PasswordHash};
use aes_gcm::{Aes256Gcm, Key, KeyInit};
use std::fmt;
use super::{
    login::Login, 
    super::errors::auth::LoginError, 
    derive_credentials::DeriveCredentials
};

#[derive(Serialize, Deserialize)]
pub struct User{
    pub id: i32,
    pub name: String,
    pub pwd: String, //hash
    pub salt: String //usefull for a master password
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

    pub fn create_new_user(users: &mut Vec<User>, name: String, pwd: String)-> Result<DeriveCredentials, LoginError>{
        if users.iter().any(|u| u.name == name){
            return Err(LoginError::UserAlreadyExists);
        }
        match DeriveCredentials::new(&pwd){
            Ok(derive_credentials)=>{

                users.push(
                    User { 
                        id: (users.len()+1) as i32, 
                        name: name, 
                        pwd: derive_credentials.password_hash.clone(), 
                        salt: derive_credentials.salt.clone() 
                    }
                );

                let serialized_users: String = serde_json::to_string(&users)
                                                .map_err(|_| LoginError::UserDataSerializzation)?;
                let path = Path::new("data/users.json");
                if Path::exists(&path){
                    fs::write(path, serialized_users)
                        .map_err(|_|LoginError::ImpossibleWriteFile)?;
                                     
                }else{
                    fs::File::create("data/users.json")
                        .map_err(|_|LoginError::ImpossibileCreateUserFile)?;
                }
                


                return Ok(derive_credentials);
            },
            Err(e)=>{return Err(e);}
        }
        
    }

    pub fn login_user(users: &[User], name: String, pwd: String)-> Result<Vec<Login>, LoginError>{
        todo!()
    }
}