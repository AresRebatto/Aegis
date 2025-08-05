pub struct User{
    pub id: i32,
    pub name: String,
    pub pwd: String, //hash
    pub salt: String //usefull for a master password
}

impl User{

    //Take all users from json
    pub fn get_users()-> Vec<User>{
        vec![]
    }
}