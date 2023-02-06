use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::fmt::Write;

#[derive(Debug)]
pub struct Auth {
    pub api_key: String,
    pub secret_key: String,
}

impl Auth {
    pub fn x_api_sign(&mut self, sign_query: String) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes()).unwrap();
        mac.update(sign_query.as_bytes());
        let hashed_query = mac.finalize().into_bytes();
        let mut x_api_sign = String::new();
        for bytes in hashed_query {
            write!(&mut x_api_sign, "{bytes:02x}").unwrap();
        }
        x_api_sign
    }
}
