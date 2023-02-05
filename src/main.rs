mod api;
mod auth;

use dotenvy::dotenv;
use hmac::{Hmac, Mac};
use qrcode::{render::unicode, QrCode};
use serde_json::Value;
use sha2::Sha256;
use std::fmt::Write;

use crate::auth::Auth;

type HmacSha256 = Hmac<Sha256>;

struct Transaction<'a> {
    from_currency: &'a str,
    to_currency: &'a str,
    from_qty: &'a str,
    to_address: &'a str,
    swap_type: &'a str,
}

fn start() {
    let my_order = Transaction {
        from_currency: "BTCLN",
        to_currency: "XMR",
        from_qty: "0.017999",
        to_address: &std::env::var("ENVADDRESS").unwrap(),
        swap_type: "float",
    };

    let query_convert_mac = "fromCurrency=".to_string()
        + my_order.from_currency
        + "&toCurrency="
        + my_order.to_currency
        + "&fromQty="
        + my_order.from_qty
        + "&toAddress="
        + my_order.to_address
        + "&type="
        + my_order.swap_type;

    let mut auth = Auth {
        api_key: std::env::var("ENVTAPIKEY").unwrap(),
        secret_key: std::env::var("ENVSECRET").unwrap(),
    };

    println!("{query_convert_mac}");
    let mut mac = HmacSha256::new_from_slice(b"").unwrap();
    mac.update(query_convert_mac.as_bytes());
    let hashed_query = mac.finalize().into_bytes();
    println!("{hashed_query:x}");
    let mut x_api_sign = String::new();
    for bytes in hashed_query {
        write!(&mut x_api_sign, "{bytes:02x}").unwrap();
    }

    let deneme = Auth::x_api_sign(&mut auth);
    println!("{deneme}");

    println!("{}", &x_api_sign);

    let resp = ureq::post("https://fixedfloat.com/api/v1/createOrder")
        .set("X-API-KEY", &auth.api_key)
        .set("X-API-SIGN", &deneme)
        .send_form(&[
            ("fromCurrency", my_order.from_currency),
            ("toCurrency", my_order.to_currency),
            ("fromQty", my_order.from_qty),
            ("toAddress", my_order.to_address),
            ("type", my_order.swap_type),
        ])
        .unwrap()
        .into_string();

    let v: Value = serde_json::from_str(resp.unwrap().as_str()).unwrap();
    println!("{}", v["data"]["id"]);
    println!("{}", v["data"]["from"]["address"]);

    let invoice = v["data"]["from"]["address"].to_string();
    let clean_invoice = invoice.replace('"', "");
    let code = QrCode::with_version(
        clean_invoice,
        qrcode::Version::Normal(10),
        qrcode::EcLevel::L,
    )
    .unwrap();
    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{image}");
}
fn main() {
    dotenv().ok();
    start()
}
