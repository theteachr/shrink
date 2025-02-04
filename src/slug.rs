use regex::Regex;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Slug(String);
pub struct Validator(Regex);

impl Slug {
    pub(crate) fn new(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Validator {
    pub fn validate<'a>(&self, code: &'a str) -> Option<Slug> {
        self.0.is_match(code).then_some(Slug(code.to_owned()))
    }
}

impl Default for Validator {
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
