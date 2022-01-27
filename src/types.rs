use std::str::FromStr;

use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct OrderLineItem {
    item_id: Uuid,
    quantity: i32,
    notes: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Address {
    address_1: String,
    address_2: Option<String>,
    city: String,
    state: String,
    zip: String,
}


#[derive(Clone, Debug, PartialEq)]
pub enum OrderType {
    Delivery(Address),
    CarryOut,
}


#[derive(Clone, Debug, PartialEq)]
pub enum OrderStatus {
    Preparing,
    InOven,
    EnRoute,
    Delivered,
}

impl Default for OrderType {
    fn default() -> Self {
        Self::CarryOut
    }
}

impl Default for OrderStatus {
    fn default() -> Self {
        Self::Preparing
    }
}

impl FromStr for OrderStatus {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "Preparing" => Ok(Self::Preparing),
            "InOven" => Ok(Self::InOven),
            "EnRoute" => Ok(Self::EnRoute),
            "Delivered" => Ok(Self::Delivered),
            _ => Err(())
        }
    }
}
