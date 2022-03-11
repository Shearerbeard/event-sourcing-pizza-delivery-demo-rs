use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use api::OrderService;
use eventstore::{Client, ClientSettings, SubscribeToAllOptions, SubscriptionFilter};
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

    let service = OrderService::new(client.clone());
    println!("Service Init!");

    let app_data = Data::new(service);
    let sub_data = app_data.clone();

    tokio::spawn(async move {
        let sub_options = SubscribeToAllOptions::default()
            .filter(SubscriptionFilter::on_stream_name().add_prefix("order"));

        let mut sub = client.clone().subscribe_to_all(&sub_options).await;

        println!("Got subscription!");

        loop {
            let event = sub.next().await.unwrap();
            let event_data = event.get_original_event();
            let ee = event_data
                .as_json::<ESDBEventPayload>()
                .unwrap()
                .event_envelope::<Order>(event_data.revision as usize)
                .unwrap();
            println!("--------------------------------------");
            println!("Received new envelope! \r\n {:?}", ee);
            println!("--------------------------------------");

            if sub_data.orders_projection.handle(ee).await.is_ok() {
                // let _ = sub.ack(event).await;
                println!("Projection handled Sub Event!");
            }
        }
    });

    HttpServer::new(move || {
        println!("Firing server!");
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(app_data.clone())
            .service(web::place_order)
            .service(web::update_order_status)
            .service(web::get_orders)
            .service(web::get_order)
    })
    .bind("127.0.0.1:8086")?
    .run()
    .await
}
