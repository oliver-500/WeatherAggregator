use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {

    #[error("Provided password is too short. Minimum password length is 8 characters.")]
    PasswordTooShort,

    #[error("Provided password is too long. Maximum password length is 20 characters.")]
    PasswordTooLong,

    #[error("Password must include at least one uppercase letter.")]
    PasswordDoesNotIncludeOneUppercaseLetter,

    #[error("Password must include at least one lowercase letter.")]
    PasswordDoesNotIncludeOneLowercaseLetter,

    #[error("Password must include at least one digit.")]
    PasswordDoesNotIncludeOneDigit,

    #[error("Password cannot include a whitespace character.")]
    PasswordIncludesWhitespaces,

    #[error("User with this email or username already exists. Username and email should be unique.")]
    UserAlreadyExists,

    #[error("Provided email is in incorrect format. Required format is xxx@yyy.zzz")]
    EmailInIncorrectFormat,

}