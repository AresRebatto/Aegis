pub enum LoginError{
    UserAlreadyExists,
    HashingError,
    GeneratingEnryptionKeyError,
    WrongPassword,
    UnknownError(String)
}