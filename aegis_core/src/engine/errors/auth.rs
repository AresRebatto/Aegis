#[derive(Debug)]
pub enum LoginError{
    UserAlreadyExists,
    HashingError,
    GeneratingEnryptionKeyError,
    WrongPassword,
    EmptyPassword,
    UserDataSerializzation,
    ImpossibleWriteFile,
    ImpossibileCreateUserFile,
    UnknownError(String)
}