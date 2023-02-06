#![allow(dead_code)]

pub const GET_CURRENCIES: &str = "https://fixedfloat.com/api/v1/getCurrencies";
pub const GET_PRICE: &str = "https://fixedfloat.com/api/v1/getPrice";
pub const GET_ORDER: &str = "https://fixedfloat.com/api/v1/getOrder";
pub const CREATE_ORDER: &str = "https://fixedfloat.com/api/v1/createOrder";
pub const SET_EMERGENCY: &str = "https://fixedfloat.com/api/v1/setEmergency";

pub struct GetCurrencies {
    pub currency: String,
    pub symbol: String,
    pub name: String,
    pub network: String,
    pub alias: String,
    pub currency_type: String,
    pub send: bool,
    pub recv: bool,
}

pub struct GetPrice {
    pub from_currency: String,
    pub from_qty: f64,
    pub to_currency: String,
    pub to_qty: f64,
    pub conversation_type: String,
}

pub struct GetOrder {
    pub id: String,
    pub token: String,
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

struct SetEmergency {
    id: String,
    token: String,
    choice: String,
    address: String,
}
