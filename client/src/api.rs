use std::fmt;
use chrono::{DateTime, Utc};
use reqwasm::http::Request;
use serde::{Serialize, Deserialize};

static ROOT: &str = "http://localhost:8086";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OrderView {
    pub id: String,
    pub order_status: OrderStatus,
    pub line_items: Vec<OrderLineItem>,
    pub order_type: OrderType,
    pub address: Option<OrderAddress>,
    pub sub_total: i64,
    pub tax: i64,
    pub total: i64,
    pub last_modified: DateTime<Utc>,
    pub position: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderLineItem {
    item_id: String,
    quantity: i64,
    notes: Option<String>,
    price: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderAddress {
    pub address_1: String,
    pub address_2: Option<String>,
    pub city: String,
    pub state: String,
    pub zip: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OrderType {
    Delivery,
    CarryOut,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OrderStatus {
    Preparing,
    InOven,
    EnRoute,
    Delivered,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OrderStatus::Preparing => write!(f, "Preparing"),
            OrderStatus::InOven => write!(f, "InOven"),
            OrderStatus::EnRoute => write!(f, "EnRoute"),
            OrderStatus::Delivered => write!(f, "Delivered"),
        }
    }
}

pub async fn fetch_orders() -> Result<Vec<OrderView>, reqwasm::Error> {
    let uri = format!("{}/order", ROOT);
    let resp = Request::get(&uri).send().await?;

    let body = resp.json::<Vec<OrderView>>().await?;
    Ok(body)
}
