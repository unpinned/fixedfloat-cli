#![allow(dead_code)]

const GET_CURRENCIES: &str = "https://fixedfloat.com/api/v1/getCurrencies";
const GET_PRICE: &str = "https://fixedfloat.com/api/v1/getPrice";
const GET_ORDER: &str = "https://fixedfloat.com/api/v1/getOrder";
const SET_EMERGENCY: &str = "https://fixedfloat.com/api/v1/setEmergency";
const CREATE_ORDER: &str = "https://fixedfloat.com/api/v1/createOrder";

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

#[derive(Clone)]
pub struct CreateOrder {
    pub from_currency: String,
    pub to_currency: String,
    pub from_qty: f64,
    pub to_qty: f64,
    pub to_address: String,
    pub extra: String,
    pub conversation_type: String,
}
