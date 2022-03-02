use actix_web::{App, HttpServer};
use eventstore::{Client, ClientSettings};
use order::projection::OrderProjection;
use thalo_eventstoredb::ESDBEventStore;
use web::WebServer;

mod order;
mod web;

// enum Error {
//     ClientSettingsError(ClientSettingsParseError),
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//    let settings = "esdb://localhost:2113?tls=false"
//        .parse::<ClientSettings>()
//        // .map_err(Error::ClientSettingsParseError)
//        .unwrap();
//    let client = Client::new(settings).unwrap();
//    let event_store = ESDBEventStore::new(client);
//     let order_id = String::from("1");

//     let exists = event_store
//         .load_aggregate_sequence::<Order>(&order_id)
//         .await
//         .unwrap()
//         .is_some();

//     if exists {
//         println!("EXISTS");
//     } else {
//         println!("DOES NOT EXIST... CREATING");
//         let event = event_store
//             .execute(order_id, |s: &Order| {
//                 s.order_placed(
//                     "Carryout".to_string(),
//                     vec![aggregate::LineItem {
//                         item_id: "1".to_string(),
//                         quantity: 1,
//                         notes: None,
//                     }],
//                     None,
//                 )
//             })
//             .await;

//         match event {
//             Ok(event) => println!("GOT EVENT! {:?}", event),
//             Err(e) => println!("GOT ERROR {:?}", e),
//         }
//     }
//     Ok(())
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let settings = "esdb://localhost:2113?tls=false"
            .parse::<ClientSettings>()
            // .map_err(Error::ClientSettingsParseError)
            .unwrap();

        App::new()
            .app_data(WebServer {
                event_store: ESDBEventStore::new(Client::new(settings.clone()).unwrap()),
                orders_projection: OrderProjection::default(),
            })
            .service(web::place_order)
            .service(web::get_orders)
    })
    .bind("127.0.0.1:8086")?
    .run()
    .await
}
