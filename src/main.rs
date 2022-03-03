use actix_web::{App, HttpServer, web::Data};
use eventstore::{Client, ClientSettings};
use order::projection::OrderProjection;
use thalo_eventstoredb::ESDBEventStore;
use web::WebServer;

mod order;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let settings = "esdb://localhost:2113?tls=false"
            .parse::<ClientSettings>()
            .unwrap();

        let event_store = ESDBEventStore::new(Client::new(settings).unwrap());
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
