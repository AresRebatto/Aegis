#[derive(Debug)]
pub enum LoginError{
    UserAlreadyExists,
    HashingError,
    GeneratingEnryptionKeyError,
    WrongPassword,
    EmptyPassword,
    UnknownError(String)
}