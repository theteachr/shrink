use super::Validate;

#[derive(Default)]
pub struct Alnum;

impl Validate for Alnum {
    fn validate(&self, code: &str) -> bool {
        code.chars().all(char::is_alphanumeric)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid(code: &str) -> bool {
        Alnum::default().validate(code)
    }

    #[test]
    fn valid_code() {
        assert!(is_valid("code"))
    }

    #[test]
    fn slash_is_invalid() {
        assert!(!is_valid("/code"))
    }

    #[test]
    fn dots_are_invalid() {
        assert!(!is_valid(".."))
    }

    #[test]
    fn spaces_are_invalid() {
        assert!(!is_valid("this is not valid"))
    }

    #[test]
    fn at_is_invalid() {
        assert!(!is_valid("blazing@fast"))
    }
}
