use crate::order::aggregate;
use async_trait::async_trait;
use thalo::event::{EventEnvelope, EventHandler};

pub struct OrderProjection();
pub struct Error();

#[async_trait]
impl EventHandler<aggregate::OrderEvent> for OrderProjection {
    type Error = Error;

    async fn handle(
        &self,
        _event: EventEnvelope<aggregate::OrderEvent>,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
