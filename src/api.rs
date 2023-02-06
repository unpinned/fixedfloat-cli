#![allow(dead_code)]

pub const GET_CURRENCIES: &str = "https://fixedfloat.com/api/v1/getCurrencies";
pub const GET_PRICE: &str = "https://fixedfloat.com/api/v1/getPrice";
pub const GET_ORDER: &str = "https://fixedfloat.com/api/v1/getOrder";
pub const SET_EMERGENCY: &str = "https://fixedfloat.com/api/v1/setEmergency";
pub const CREATE_ORDER: &str = "https://fixedfloat.com/api/v1/createOrder";

struct GetCurrencies {
    currency: String,
    symbol: String,
    name: String,
    network: String,
    alias: String,
    currency_type: String,
    send: bool,
    recv: bool,
}

struct GetPrice {
    from_currency: String,
    from_qty: f64,
    to_currency: String,
    to_qty: f64,
    conversation_type: String,
}

struct GetOrder {
    id: String,
    token: String,
}

struct SetEmergency {
    id: String,
    token: String,
    choice: String,
    address: String,
}

pub struct CreateOrder {
    pub from_currency: String,
    pub to_currency: String,
    pub from_qty: f64,
    pub to_qty: f64,
    pub to_address: String,
    pub extra: String,
    pub conversation_type: String,
}
