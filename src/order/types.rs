use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderLineItem {
    item_id: Uuid,
    quantity: i32,
    notes: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Address {
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

impl FromStr for OrderType {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "Delivery" => Ok(Self::Delivery),
            "Carryout" => Ok(Self::CarryOut),
            _ => Err(()),
        }
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
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &OrderStatus::Preparing => write!(f, "Preparing"),
            &OrderStatus::InOven => write!(f, "InOven"),
            &OrderStatus::EnRoute => write!(f, "EnRoute"),
            &OrderStatus::Delivered => write!(f, "Delivered"),
        }
    }
}
