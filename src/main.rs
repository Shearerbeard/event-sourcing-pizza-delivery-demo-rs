use actix_web::{web::Data, App, HttpServer};
use eventstore::{
    Client, ClientSettings, PersistentSubscriptionToAllOptions,
    SubscribeToPersistentSubscriptionOptions, SubscriptionFilter,
};
use order::{projection::OrderProjection, aggregate::Order};
use thalo::{event::EventHandler};
use thalo_eventstoredb::{ESDBEventStore, ESDBEventPayload};
use web::WebServer;

mod order;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = "esdb://localhost:2113?tls=false"
        .parse::<ClientSettings>()
        .unwrap();
    let client = Client::new(settings).unwrap();
    let sub_client = client.clone();

    let options = PersistentSubscriptionToAllOptions::default()
        .filter(SubscriptionFilter::on_stream_name().add_prefix("order"));

    let _ = client
        .clone()
        .create_persistent_subscription_to_all("order-group", &options)
        .await;

    let sub_options = SubscribeToPersistentSubscriptionOptions::default();
    let mut sub = sub_client.subscribe_to_persistent_subscription_to_all(
        "order-group",
        &sub_options,
    ).await.unwrap();

    let orders_projection = OrderProjection::default();

    tokio::spawn(async move {
        loop {
            let event = sub.next().await.unwrap();
            let event_data = event.get_original_event();
            let ee = event_data
                .as_json::<ESDBEventPayload>().unwrap()
                // .map_err(Error)?
                .event_envelope::<Order>(event_data.revision as usize).unwrap();

            if let Ok(_) = orders_projection.handle(ee).await {
                let _ = sub.ack(event).await;
            }
        }
    });


    HttpServer::new(move || {
        let event_store = ESDBEventStore::new(client.clone());
        let orders_projection = OrderProjection::default();

        App::new()
            .app_data(Data::new(WebServer {
                event_store,
                orders_projection,
            }))
            .service(web::place_order)
            .service(web::get_orders)
    })
    .bind("127.0.0.1:8086")?
    .run()
    .await
}
