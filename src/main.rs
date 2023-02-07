mod api;
mod auth;

use crate::{
    api::{CreateOrder, GetOrder, GetPrice, CREATE_ORDER, GET_CURRENCIES, GET_ORDER, GET_PRICE},
    auth::Auth,
};
use dotenvy::dotenv;
use owo_colors::OwoColorize;
use qrcode::{render::unicode, QrCode};
use serde_json::Value;

fn auth() -> Auth {
    Auth {
        api_key: std::env::var("ENVTAPIKEY").unwrap(),
        secret_key: std::env::var("ENVSECRET").unwrap(),
    }
}

fn create_order() -> (String, String) {
    let mut auth = auth();
    let my_order = CreateOrder {
        from_currency: std::env::var("ENVFROMCURRENCRY").unwrap(),
        to_currency: std::env::var("ENVTOCURRENCRY").unwrap(),
        from_qty: std::env::var("ENVFROMQTY").unwrap().parse::<f64>().unwrap(),
        to_qty: 0.0025,
        to_address: std::env::var("ENVADDRESS").unwrap(),
        extra: "54132".to_string(),
        conversation_type: std::env::var("ENVCONVERSATIONTYPE").unwrap(),
    };

    let sign_query = "fromCurrency=".to_string()
        + &my_order.from_currency
        + "&toCurrency="
        + &my_order.to_currency
        + "&fromQty="
        + &my_order.from_qty.to_string()
        + "&toAddress="
        + &my_order.to_address
        + "&type="
        + &my_order.conversation_type;

    let resp = ureq::post(CREATE_ORDER)
        .set("X-API-KEY", &auth.api_key)
        .set("X-API-SIGN", &Auth::x_api_sign(&mut auth, sign_query))
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
    let id = v["data"]["id"].to_string();
    let token = v["data"]["token"].to_string();
    println!("Your ID is: {}", v["data"]["id"].green());
    println!("Your TOKEN is: {}", v["data"]["token"].green());
    println!(
        "You will receive to this address: {}",
        std::env::var("ENVADDRESS").unwrap().green()
    );
    println!(
        "Pay this amount to below address: {}",
        std::env::var("ENVFROMQTY")
            .unwrap()
            .parse::<f64>()
            .unwrap()
            .green()
    );
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
    (id, token)
}

fn get_currencies() {
    let mut auth = auth();

    let resp = ureq::get(GET_CURRENCIES)
        .set("X-API-KEY", &auth.api_key)
        .set("X-API-SIGN", &Auth::x_api_sign(&mut auth, "".to_owned()))
        .call()
        .unwrap()
        .into_string()
        .unwrap();

    println!("{resp}");
}

fn get_price() {
    let mut auth = auth();
    let my_order = GetPrice {
        from_currency: std::env::var("ENVFROMCURRENCRY").unwrap(),
        to_currency: std::env::var("ENVTOCURRENCRY").unwrap(),
        from_qty: std::env::var("ENVFROMQTY").unwrap().parse::<f64>().unwrap(),
        to_qty: 0.0025,
        conversation_type: std::env::var("ENVCONVERSATIONTYPE").unwrap(),
    };

    let sign_query = "fromCurrency=".to_string()
        + &my_order.from_currency
        + "&toCurrency="
        + &my_order.to_currency
        + "&fromQty="
        + &my_order.from_qty.to_string()
        + "&type="
        + &my_order.conversation_type;

    let resp = ureq::post(GET_PRICE)
        .set("X-API-KEY", &auth.api_key)
        .set("X-API-SIGN", &Auth::x_api_sign(&mut auth, sign_query))
        .send_form(&[
            ("fromCurrency", &my_order.from_currency),
            ("toCurrency", &my_order.to_currency),
            ("fromQty", &my_order.from_qty.to_string()),
            ("type", &my_order.conversation_type),
        ])
        .unwrap()
        .into_string();

    let v: Value = serde_json::from_str(resp.unwrap().as_str()).unwrap();

    println!("{}", v.red());
}

fn get_order(id: String, token: String) {
    let mut auth = auth();
    let my_order = GetOrder { id, token };

    let sign_query = "id=".to_string() + &my_order.id + "&token=" + &my_order.token;

    let resp = ureq::post(GET_ORDER)
        .set("X-API-KEY", &auth.api_key)
        .set("X-API-SIGN", &Auth::x_api_sign(&mut auth, sign_query))
        .send_form(&[("id", &my_order.id), ("token", &my_order.token)])
        .unwrap()
        .into_string();

    let v: Value = serde_json::from_str(resp.unwrap().as_str()).unwrap();

    println!("{}", v.blue());
}

fn backup_link(id: String) {
    let backup_link = "https://fixedfloat.com/order/";
    let backup_id = id.trim_matches('"');
    let combine = backup_link.to_owned() + backup_id;
    println!("Your backup link is: {}", combine.blue());
    println!(
        "The link will ask for your receive address: {}",
        std::env::var("ENVADDRESS").unwrap().blue()
    );
}

fn main() {
    dotenv().ok();
    let (id, token) = create_order();
    get_currencies();
    get_price();
    get_order(id.clone(), token);
    backup_link(id)
}
