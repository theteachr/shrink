use regex::Regex;

use super::Validate;

pub struct DefaultValidator(Regex);

impl Validate for DefaultValidator {
    fn validate(&self, code: &str) -> bool {
        self.0.is_match(code)
    }
}

impl Default for DefaultValidator {
    fn default() -> Self {
        let unreserved = r"A-Za-z0-9\-._~";
        let pct_encoded = "%[0-9A-Fa-f]{2}";
        let sub_delims = "!$&'()*+,;=";
        let other_chars = ":@";

        let pchar = format!("[{unreserved}{sub_delims}{other_chars}]|{pct_encoded}");
        let re = Regex::new(&format!("^(?:{})*$", pchar)).unwrap();

        Self(re)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid(code: &str) -> bool {
        DefaultValidator::default().validate(code)
    }

    #[test]
    fn valid_code() {
        assert!(is_valid("code"))
    }

    #[test]
    fn invalid_code() {
        assert!(!is_valid("/code"))
    }

    #[test]
    fn spaces_are_invalid() {
        assert!(!is_valid("this is not valid"))
    }

    #[test]
    fn at_is_allowed() {
        assert!(is_valid("blazing@fast"))
    }
}
