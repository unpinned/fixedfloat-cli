#![allow(dead_code)]

use crate::api::CreateOrder;
use crate::HmacSha256;

use hmac::Mac;
use std::fmt::Write;

pub struct Auth {
    pub api_key: String,
    pub secret_key: String,
}

impl Auth {
    pub fn x_api_sign(&mut self) -> String {
        let my_order = CreateOrder {
            from_currency: "BTCLN".to_owned(),
            to_currency: "XMR".to_owned(),
            from_qty: 0.017999,
            to_qty: 0.0025,
            to_address: std::env::var("ENVADDRESS").unwrap(),
            extra: "54132".to_string(),
            conversation_type: "float".to_owned(),
        };
        let query_convert_mac = "fromCurrency=".to_string()
            + &my_order.from_currency
            + "&toCurrency="
            + &my_order.to_currency
            + "&fromQty="
            + &my_order.from_qty.to_string()
            + "&toAddress="
            + &my_order.to_address
            + "&type="
            + &my_order.conversation_type;
        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes()).unwrap();
        mac.update(query_convert_mac.as_bytes());
        let hashed_query = mac.finalize().into_bytes();
        println!("{hashed_query:x}");
        let mut x_api_sign = String::new();
        for bytes in hashed_query {
            write!(&mut x_api_sign, "{bytes:02x}").unwrap();
        }
        x_api_sign
    }
}
