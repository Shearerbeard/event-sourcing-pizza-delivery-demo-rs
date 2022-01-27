mod types;

use std::str::FromStr;

use thalo::{
    aggregate::{Aggregate, TypeId},
    include_aggregate,
};
use uuid::Uuid;

include_aggregate!("Order");

#[derive(Clone, Debug, Default, PartialEq, TypeId, Aggregate)]
struct Order {
    id: String,
    order_status: types::OrderStatus,
    line_items: Vec<types::OrderLineItem>,
    order_type: types::OrderType,
    address: Option<types::Address>
}

impl OrderCommand for Order {
    type Error = Error;

    fn order_placed(
        &self,
        order_type: String,
        line_items: Vec<LineItem>,
        address: Option<Address>,
    ) -> Result<OrderPlacedEvent, Error> {
        if line_items.len() < 1 {
            return Result::Err(Error::OrderCouldNotBePlaced(
                String::from("Must include at least one line item")
            ));
        } else {
            match types::OrderType::from_str(&order_type) {
                Ok(types::OrderType::Delivery) => {
                    if let Some(_) = address {
                        Ok(OrderPlacedEvent {
                            line_items,
                            order_type,
                            address,
                            id: Uuid::new_v4().to_string(),
                            order_status: types::OrderStatus::Preparing.to_string(),
                        })
                    } else {
                        Err(Error::OrderCouldNotBePlaced(
                            String::from("Address required for Delivery")
                        ))
                    }
                }
                Ok(types::OrderType::CarryOut) => Ok(OrderPlacedEvent {
                    line_items,
                    order_type,
                    address,
                    id: Uuid::new_v4().to_string(),
                    order_status: "Preparing".to_string(),
                }),
                Err(_) => Err(Error::OrderCouldNotBePlaced(
                    String::from("Invalid OrderType")
                )),
            }
        }
    }

    fn order_status_changed(
        &self,
        id: String,
        order_status: String,
    ) -> Result<OrderStatusChangedEvent, Error> {
        match types::OrderStatus::from_str(&order_status) {
            Ok(_) => Ok(OrderStatusChangedEvent { id, order_status }),
            Err(_) => Err(Error::OrderStatusCouldNotBeChanged(
                String::from("Incorrect order status"),
            )),
        }
    }
}

fn apply(_order: &mut Order, event: OrderEvent) {
    // use OrderEvent::*;

    match event {
        _ => (),
    }
}

pub enum Error {
    OrderCouldNotBePlaced(String),
    OrderStatusCouldNotBeChanged(String),
}

fn main() {
    println!("Hello, world!");
}
