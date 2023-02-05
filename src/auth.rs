#![allow(dead_code)]

use crate::api::CreateOrder;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::fmt::Write;

pub struct Auth {
    pub api_key: String,
    pub secret_key: String,
}

impl Auth {
    pub fn x_api_sign(&mut self, order: CreateOrder) -> String {
        let query_convert_mac = "fromCurrency=".to_string()
            + &order.from_currency
            + "&toCurrency="
            + &order.to_currency
            + "&fromQty="
            + &order.from_qty.to_string()
            + "&toAddress="
            + &order.to_address
            + "&type="
            + &order.conversation_type;
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes()).unwrap();
        mac.update(query_convert_mac.as_bytes());
        let hashed_query = mac.finalize().into_bytes();
        let mut x_api_sign = String::new();
        for bytes in hashed_query {
            write!(&mut x_api_sign, "{bytes:02x}").unwrap();
        }
        x_api_sign
    }
}
