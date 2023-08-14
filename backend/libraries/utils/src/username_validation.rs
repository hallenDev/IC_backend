const MAX_USERNAME_LENGTH: u16 = 25;
const MIN_USERNAME_LENGTH: u16 = 5;

pub enum UsernameValidationError {
    TooLong(u16),
    TooShort(u16),
    Invalid,
}

pub fn validate_username(username: &str) -> Result<(), UsernameValidationError> {
    if username.len() > MAX_USERNAME_LENGTH as usize {
        return Err(UsernameValidationError::TooLong(MAX_USERNAME_LENGTH));
    }

    if username.len() < MIN_USERNAME_LENGTH as usize {
        return Err(UsernameValidationError::TooShort(MIN_USERNAME_LENGTH));
    }

    if username.starts_with('_')
        || username.ends_with('_')
        || username.contains("__")
        || username.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_')
    {
        return Err(UsernameValidationError::Invalid);
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_usernames() {
        assert!(matches!(validate_username("abcde"), Ok(_)));
        assert!(matches!(validate_username("12345"), Ok(_)));
        assert!(matches!(validate_username("SNSABC"), Ok(_)));
        assert!(matches!(validate_username("1_2_3_4_5_6_7_8_9_0_1_2_3"), Ok(_)));
    }

    #[test]
    fn invalid_usernames() {
        assert!(matches!(validate_username("abcde "), Err(UsernameValidationError::Invalid)));
        assert!(matches!(validate_username("ab cde"), Err(UsernameValidationError::Invalid)));
        assert!(matches!(validate_username("_abcde"), Err(UsernameValidationError::Invalid)));
        assert!(matches!(validate_username("abcde_"), Err(UsernameValidationError::Invalid)));
        assert!(matches!(validate_username("ab__cde"), Err(UsernameValidationError::Invalid)));
        assert!(matches!(validate_username("ab,cde"), Err(UsernameValidationError::Invalid)));
        assert!(matches!(validate_username("abcéd"), Err(UsernameValidationError::Invalid)));
        assert!(matches!(validate_username("abcṷd"), Err(UsernameValidationError::Invalid)));
        assert!(matches!(validate_username("abc王d"), Err(UsernameValidationError::Invalid)));
    }
}
