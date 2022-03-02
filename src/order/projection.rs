use std::{collections::HashMap, sync::Mutex, str::FromStr};

use crate::order::aggregate;
use async_trait::async_trait;
use chrono::{DateTime, Utc, FixedOffset};
use serde::{Deserialize, Serialize};
use thalo::event::{EventEnvelope, EventHandler};

use super::{
    aggregate::{OrderEvent, OrderPlacedEvent, OrderStatusChangedEvent},
    types::{Address, OrderLineItem, OrderStatus, OrderType},
};

pub struct Error();

#[derive(Default)]
pub struct OrderProjection {
    view: Mutex<HashMap<String, OrderView>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderView {
    id: String,
    order_status: OrderStatus,
    // line_items: Vec<OrderLineItem>,
    order_type: OrderType,
    // address: Option<Address>,
    sub_total: i64,
    tax: i64,
    total: i64,
    last_modified: DateTime<Utc>,
    position: usize,
}

impl OrderProjection {
    fn handle_order_placed(
        &self,
        id: String,
        _line_items: Vec<aggregate::LineItem>,
        order_type: String,
        _address: Option<aggregate::Address>,
        order_status: String,
        last_modified: DateTime<Utc>,
        position: usize,
    ) {
        let mut view = self.view.lock().unwrap();
        let order_id = id.to_owned();

        view.entry(id).or_insert(OrderView {
            id: order_id,
            // line_items,
            // address,
            sub_total: 0,
            tax: 0,
            total: 0,
            last_modified,
            position,
            order_status: OrderStatus::from_str(&order_status).unwrap(),
            order_type: OrderType::from_str(&order_type).unwrap(),
        });
    }

    fn handle_order_status_changed(&self, id: String, order_status: String) {
        todo!()
    }
}

#[async_trait]
impl EventHandler<OrderEvent> for OrderProjection {
    type Error = Error;

    async fn handle(
        &self,
        EventEnvelope {
            aggregate_id,
            sequence,
            created_at,
            event,
            ..
        }: EventEnvelope<OrderEvent>,
    ) -> Result<(), Self::Error> {
        match event {
            OrderEvent::OrderPlaced(OrderPlacedEvent {
                line_items,
                order_type,
                address,
                order_status,
                ..
            }) => Ok(self.handle_order_placed(
                aggregate_id,
                line_items,
                order_type,
                address,
                order_status,
                created_at.into(),
                sequence,
            )),
            OrderEvent::OrderStatusChanged(OrderStatusChangedEvent { order_status, .. }) => {
                Ok(self.handle_order_status_changed(aggregate_id, order_status))
            }
        }
    }
}
