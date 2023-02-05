mod api;
mod auth;

use crate::{api::CreateOrder, auth::Auth};
use dotenvy::dotenv;
use owo_colors::OwoColorize;
use qrcode::{render::unicode, QrCode};
use serde_json::Value;

fn start() {
    let my_order = CreateOrder {
        from_currency: "BTCLN".to_owned(),
        to_currency: "XMR".to_owned(),
        from_qty: 0.017999,
        to_qty: 0.0025,
        to_address: std::env::var("ENVADDRESS").unwrap(),
        extra: "54132".to_string(),
        conversation_type: "float".to_owned(),
    };

    let mut auth = Auth {
        api_key: std::env::var("ENVTAPIKEY").unwrap(),
        secret_key: std::env::var("ENVSECRET").unwrap(),
    };

    let resp = ureq::post("https://fixedfloat.com/api/v1/createOrder")
        .set("X-API-KEY", &auth.api_key)
        .set("X-API-SIGN", &Auth::x_api_sign(&mut auth, my_order.clone()))
        .send_form(&[
            ("fromCurrency", &my_order.from_currency),
            ("toCurrency", &my_order.to_currency),
            ("fromQty", &my_order.from_qty.to_string()),
            ("toAddress", &my_order.to_address),
            ("type", &my_order.conversation_type),
        ])
        .unwrap()
        .into_string();

    let v: Value = serde_json::from_str(resp.unwrap().as_str()).unwrap();
    println!("{}", v["data"]["id"].green());
    println!("{}", v["data"]["from"]["address"].blue());

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
