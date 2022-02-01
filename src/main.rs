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
                "Must include at least one line item".to_string(),
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
                            "Address required for Delivery".to_string(),
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
                    "Invalid OrderType".to_string(),
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
                "Incorrect order status".to_string(),
            )),
        }
    }
}

fn apply(order: &mut Order, event: OrderEvent) {
    match event {
        OrderEvent::OrderPlaced(e)=> {
            order.id = e.id;
            order.order_status = types::OrderStatus::from_str(&e.order_status).unwrap();
            order.order_type = types::OrderType::from_str(&e.order_type).unwrap();
            // order.address = e.address;
            // order.line_items = e.line_items;
        },
        _ => ()
    }
}

pub enum Error {
    OrderCouldNotBePlaced(String),
    OrderStatusCouldNotBeChanged(String),
}

fn main() {
    println!("Hello, world!");
}
