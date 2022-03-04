use std::{collections::HashMap, str::FromStr, sync::Mutex};

use crate::order::aggregate;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thalo::event::{EventEnvelope, EventHandler};

use super::{
    aggregate::{OrderEvent, OrderPlacedEvent, OrderStatusChangedEvent},
    types::{OrderAddress, OrderLineItem, OrderStatus, OrderType},
};

pub enum Error {
    OrderNotFound,
}

#[derive(Default)]
pub struct OrderProjection {
    view: Mutex<HashMap<String, OrderView>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderView {
    id: String,
    order_status: OrderStatus,
    line_items: Vec<OrderLineItem>,
    order_type: OrderType,
    address: Option<OrderAddress>,
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
        line_items: Vec<aggregate::LineItem>,
        order_type: String,
        address: Option<aggregate::Address>,
        order_status: String,
        last_modified: DateTime<Utc>,
        position: usize,
    ) {
        let mut view = self.view.lock().unwrap();
        let line_items_2 = line_items.clone();
        let key = id.to_owned();
        let sub_total = line_items_2.into_iter().fold(0, |acc, item| acc + item.price);
        let tax = (sub_total as f64 * 0.05).floor() as i64;
        let total = sub_total + tax;

        view.entry(key).or_insert(OrderView {
            id,
            sub_total,
            tax,
            total,
            last_modified,
            position,
            line_items: line_items
                .into_iter()
                .map(OrderLineItem::from_event_line_item)
                .collect(),
            address: address.map(OrderAddress::from_event_address),
            order_status: OrderStatus::from_str(&order_status).unwrap(),
            order_type: OrderType::from_str(&order_type).unwrap(),
        });
    }

    fn handle_order_status_changed(&self, id: String, order_status: String) {
        let mut view = self.view.lock().unwrap();

        if let Some(mut order) = view.get_mut(&id) {
            order.order_status = OrderStatus::from_str(&order_status).unwrap()
        }
    }

    pub fn get(&self, id: String) -> Result<OrderView, Error> {
        let view = self.view.lock().unwrap();
        let res = view.get(&id);

        match res {
            Some(order) => Ok(order.clone()),
            None => Err(Error::OrderNotFound),
        }
    }

    pub fn get_all(&self) -> Vec<OrderView> {
        let view = self.view.lock().unwrap();
        view.clone().into_iter().map(|(_k, v)| v.clone()).collect::<Vec<OrderView>>()
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
