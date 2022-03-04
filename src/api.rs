use eventstore::Client;
use serde::{Deserialize, Serialize};
use thalo::event_store::EventStore;
use thalo_eventstoredb::ESDBEventStore;
use uuid::Uuid;

use crate::order::{
    aggregate,
    aggregate::{
        Address, LineItem, Order, OrderCommand, OrderPlacedEvent, OrderStatusChangedEvent,
    },
    projection::{OrderProjection, OrderView},
};

pub struct OrderService {
    event_store: ESDBEventStore,
    pub orders_projection: OrderProjection,
}

#[derive(Debug)]
pub enum Error {
    PlaceOrderError(aggregate::Error),
    EventStoreError(thalo_eventstoredb::Error),
    NotFoundError,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaceOrderArgs {
    order_type: String,
    line_items: Vec<LineItem>,
    address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangeOrderStatusArgs {
    pub id: String,
    pub order_status: String,
}

impl OrderService {
    pub fn new(client: Client) -> Self {
        println!("Calling Order Service Init!");

        Self {
            event_store: ESDBEventStore::new(client),
            orders_projection: OrderProjection::default(),
        }
    }

    pub async fn command_place_order(
        &self,
        PlaceOrderArgs {
            order_type,
            line_items,
            address,
        }: PlaceOrderArgs,
    ) -> Result<OrderPlacedEvent, Error> {
        self.event_store
            .execute(Uuid::new_v4().to_string(), |order: &Order| {
                order.order_placed(order_type.clone(), line_items.clone(), address.clone())
            })
            .await
            .map_err(Error::EventStoreError)?
            .map_err(Error::PlaceOrderError)
    }

    pub async fn command_change_order_status(
        &self,
        ChangeOrderStatusArgs { id, order_status }: ChangeOrderStatusArgs,
    ) -> Result<OrderStatusChangedEvent, Error> {
        self.event_store
            .execute(id.clone(), |order: &Order| {
                order.order_status_changed(id.clone(), order_status)
            })
            .await
            .map_err(Error::EventStoreError)?
            .map_err(Error::PlaceOrderError)
    }

    pub async fn read_order(&self, id: String) -> Result<OrderView, Error> {
        self.orders_projection.get(id).map_err(|_| Error::NotFoundError)
    }

    pub async fn read_all_orders(&self) -> Result<Vec<OrderView>, Error> {
        Ok(self.orders_projection.get_all())
    }
}
