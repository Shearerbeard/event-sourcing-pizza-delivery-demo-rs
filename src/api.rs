
use eventstore::Client;
use serde::{Serialize, Deserialize};
use thalo::event_store::EventStore;
use thalo_eventstoredb::ESDBEventStore;
use uuid::Uuid;

use crate::order::{
    aggregate::{Address, LineItem, Order, OrderCommand, OrderPlacedEvent, self},
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
}


#[derive(Serialize , Deserialize, Debug, Clone)]
pub struct PlaceOrderArgs {
    order_type: String,
    line_items: Vec<LineItem>,
    address: Option<Address>,
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

    pub async fn read_all_orders(&self) -> Result<Vec<OrderView>, Error> {
        Ok(self.orders_projection.get_all())
    }
}
