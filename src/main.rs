use actix_web::{web::Data, App, HttpServer};
use api::OrderService;
use eventstore::{
    Client, ClientSettings, PersistentSubscriptionToAllOptions,
    SubscribeToPersistentSubscriptionOptions, SubscriptionFilter,
};
use order::aggregate::Order;
use thalo::event::EventHandler;
use thalo_eventstoredb::ESDBEventPayload;

mod api;
mod order;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::new(
        "esdb://localhost:2113?tls=false"
            .parse::<ClientSettings>()
            .unwrap(),
    )
    .unwrap();

    println!("Got Client!");

    let options = PersistentSubscriptionToAllOptions::default()
        .filter(SubscriptionFilter::on_stream_name().add_prefix("order"));

    let res = client
        .clone()
        .create_persistent_subscription_to_all("order-group", &options)
        .await;

    println!("Did create persistent sub? {:?}", res);

    let service = OrderService::new(client.clone());
    println!("Service Init!");

    let app_data = Data::new(service);
    let sub_data = app_data.clone();

    tokio::spawn(async move {
        let sub_options = SubscribeToPersistentSubscriptionOptions::default();
        let mut sub = client
            .clone()
            .subscribe_to_persistent_subscription_to_all("order-group", &sub_options)
            .await
            .unwrap();

        println!("Got persistent subscription!");

        loop {
            let event = sub.next().await.unwrap();
            let event_data = event.get_original_event();
            let ee = event_data
                .as_json::<ESDBEventPayload>()
                .unwrap()
                .event_envelope::<Order>(event_data.revision as usize)
                .unwrap();

            println!("Received new envelope! {:?}", ee);

            if let Ok(_) = sub_data.orders_projection.handle(ee).await {
                let _ = sub.ack(event).await;
                println!("Sub Ack!");
            }
        }
    });

    HttpServer::new(move || {
        println!("Firing server!");
        App::new()
            .app_data(app_data.clone())
            .service(web::place_order)
            .service(web::get_orders)
    })
    .bind("127.0.0.1:8086")?
    .run()
    .await
}
