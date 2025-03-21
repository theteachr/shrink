use rand::Rng;

use crate::{validator::Code, Generator};

const CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Random Base62 generator.
#[derive(Default)]
pub struct RB62;

impl Generator for RB62 {
    fn generate(&mut self, _: &url::Url) -> Code {
        // Thought of reusing the random nubmer generator (`rng`) by putting
        // storing it in the struct, but that would make the struct not `Send`.
        let mut rng = rand::rng();

        let code = (0..7)
            .map(|_| CHARS[rng.random_range(0..CHARS.len())] as char)
            .collect();

        Code::new(code)
    }
}
