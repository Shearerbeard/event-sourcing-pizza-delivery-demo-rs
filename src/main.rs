use eventstore::{Client, ClientSettings, ClientSettingsParseError};
use order::aggregate::Order;
use thalo::event_store::EventStore;
use thalo_eventstoredb::ESDBEventStore;

use crate::order::aggregate;

mod order;

// enum Error {
//     ClientSettingsError(ClientSettingsParseError),
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = "esdb://localhost:2113?tls=false"
        .parse::<ClientSettings>()
        // .map_err(Error::ClientSettingsParseError)
        .unwrap();
    let client = Client::new(settings).unwrap();
    let event_store = ESDBEventStore::new(client);
    let order_id = String::from("1");

    let exists = event_store
        .load_aggregate_sequence::<Order>(&order_id)
        .await
        .unwrap()
        .is_some();

    if exists {
        println!("EXISTS");
    } else {
        println!("DOES NOT EXIST... CREATING");
        // let order = Order::new(order_id);
        let cmd = move |s: &Order| {
            aggregate::OrderCommand::order_placed(
                s,
                "Carryout".to_string(),
                [aggregate::LineItem {
                    item_id: "1".to_string(),
                    quantity: 1,
                    notes: None,
                }]
                .to_vec(),
                None,
            )
        };

        let event = event_store.execute(order_id, cmd).await;

        match event {
            Ok(event) => println!("GOT EVENT! {:?}", event),
            Err(e) => println!("GOT ERROR {:?}", e),
        }
    }
    Ok(())
}
