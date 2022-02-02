use std::str::FromStr;
use std::fmt;

use serde::{Serialize, Serializer, ser::SerializeStruct};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct OrderLineItem {
    item_id: Uuid,
    quantity: i32,
    notes: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Address {
    pub address_1: String,
    pub address_2: Option<String>,
    pub city: String,
    pub state: String,
    pub zip: String,
}


#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum OrderType {
    Delivery,
    CarryOut,
}


#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum OrderStatus {
    Preparing,
    InOven,
    EnRoute,
    Delivered,
}

// IMPL
impl Serialize for OrderLineItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut state: <S as Serializer>::SerializeStruct = serializer.serialize_struct("OrderLineItem", 3)?;
        state.serialize_field("item_id", &self.item_id.to_string())?;
        state.serialize_field("quantity", &self.quantity)?;
        state.serialize_field("notes", &self.notes)?;
        state.end()
    }
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
            _ => Err(())
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
            _ => Err(())
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
