use std::str::FromStr;

use crate::order::types;
use serde::{Deserialize, Serialize};
use thalo::{
    aggregate::{Aggregate, TypeId},
    include_aggregate,
};
use uuid::Uuid;

use super::types::{OrderAddress, OrderLineItem};

include_aggregate!("Order");

#[derive(Clone, Debug, Default, PartialEq, TypeId, Aggregate)]
pub struct Order {
    id: String,
    order_status: types::OrderStatus,
    line_items: Vec<types::OrderLineItem>,
    order_type: types::OrderType,
    address: Option<types::OrderAddress>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Error {
    OrderCouldNotBePlaced(String),
    OrderStatusCouldNotBeChanged(String),
}

impl OrderCommand for Order {
    type Error = Error;

    fn order_placed(
        &self,
        order_type: String,
        line_items: Vec<LineItem>,
        address: Option<Address>,
    ) -> Result<OrderPlacedEvent, Error> {
        if line_items.is_empty() {
            return Result::Err(Error::OrderCouldNotBePlaced(String::from(
                "Must include at least one line item",
            )));
        }

        let order_type_enum = types::OrderType::from_str(&order_type).map_err(|_| {
            return Error::OrderCouldNotBePlaced(format!(
                "{:?} could not be converted to OrderStatus!",
                order_type
            ));
        })?;

        match order_type_enum {
            types::OrderType::Delivery => {
                if address.is_some() {
                    Ok(OrderPlacedEvent {
                        line_items,
                        order_type,
                        address,
                        id: Uuid::new_v4().to_string(),
                        order_status: types::OrderStatus::Preparing.to_string(),
                    })
                } else {
                    Err(Error::OrderCouldNotBePlaced(String::from(
                        "Address required for Delivery",
                    )))
                }
            }
            types::OrderType::CarryOut => Ok(OrderPlacedEvent {
                line_items,
                order_type,
                address,
                id: Uuid::new_v4().to_string(),
                order_status: "Preparing".to_string(),
            }),
        }
    }

    fn order_status_changed(
        &self,
        id: String,
        order_status: String,
    ) -> Result<OrderStatusChangedEvent, Error> {
        let order_status_2 = order_status.clone();
        types::OrderStatus::from_str(&order_status)
            .map(|_| OrderStatusChangedEvent { id, order_status })
            .map_err(|_| {
                Error::OrderStatusCouldNotBeChanged(format!(
                    "{:?} could not be converted to OrderStatus!",
                    order_status_2
                ))
            })
    }
}

pub fn apply(order: &mut Order, event: OrderEvent) {
    match event {
        OrderEvent::OrderPlaced(OrderPlacedEvent {
            address,
            line_items,
            order_status,
            order_type,
            ..
        }) => {
            order.address = address.map(OrderAddress::from_event_address);
            order.line_items = line_items
                .into_iter()
                .map(OrderLineItem::from_event_line_item)
                .collect();
            order.order_status = types::OrderStatus::from_str(&order_status).unwrap();
            order.order_type = types::OrderType::from_str(&order_type).unwrap();
        }
        OrderEvent::OrderStatusChanged(OrderStatusChangedEvent { ref order_status, .. }) => {
            order.order_status = types::OrderStatus::from_str(order_status).unwrap();
        }
    }
}
