use eventstore::Client;
use thalo_eventstoredb::ESDBEventStore;

use crate::order::projection::OrderProjection;

pub struct OrderService {
    pub event_store: ESDBEventStore,
    pub orders_projection: OrderProjection,
}

impl OrderService {
    pub fn new(client: Client) -> Self {
        println!("Calling Order Service Init!");

        Self {
            event_store: ESDBEventStore::new(client),
            orders_projection: OrderProjection::default(),
        }
    }
}
