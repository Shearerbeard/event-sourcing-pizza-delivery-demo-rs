mod types;

use thalo::{aggregate::{TypeId, Aggregate}, include_aggregate};
use types::{OrderStatus, OrderType, OrderLineItem};
use std::{result::Result, str::FromStr};
use std::option::Option;
use std::vec::Vec;
use uuid::Uuid;

include_aggregate!("Order");

#[derive(Clone, Debug, Default, PartialEq, TypeId, Aggregate)]
struct Order {
    id: String,
    order_status: OrderStatus,
    line_items: Vec<OrderLineItem>,
    order_type: OrderType,
}

impl OrderCommand for Order {
    type Error = Error;


    fn order_placed(&self, order_type: String, line_items: Vec<LineItem>, address: Option<Address>) -> Result<OrderPlacedEvent,Self::Error> {
        if line_items.len() < 1 {
            return Result::Err(Error::OrderCouldNotBePlaced("Must include at least one line item".to_string()))
        } else {
            let event = OrderPlacedEvent {
                line_items,
                order_type,
                address,
                id: Uuid::new_v4().to_string(),
                order_status: "Preparing".to_string(),
            };

            return Result::Ok(event)
        }
    }


    fn order_status_changed(&self, id: String, order_status: String) -> Result<OrderStatusChangedEvent,Self::Error> {
        match OrderStatus::from_str(&order_status) {
            Ok(_) => {
                let event = OrderStatusChangedEvent {
                    id,
                    order_status,
                };

                return Result::Ok(event);
            },

            Err(_) => {
                return Result::Err(Error::OrderStatusCouldNotBeChanged("Incorrect order status".to_string()))
            }
        }
    }

}

pub enum Error {
    OrderCouldNotBePlaced(String),
    OrderStatusCouldNotBeChanged(String),
}

fn main() {
    println!("Hello, world!");
}
