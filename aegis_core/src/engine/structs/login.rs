
//for eveery field there's a nonce
pub struct Login{
    email: (Option<String>, [u8; 12]),
    username: (Option<String>, [u8; 12]),
    pwd: (String, [u8; 12])
}