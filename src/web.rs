use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use thalo::event_store::EventStore;
use thalo_eventstoredb::ESDBEventStore;
use uuid::Uuid;

use crate::order::{
    aggregate::{LineItem, OrderCommand, Order, Address},
    projection::OrderProjection,
};

pub struct WebServer {
    pub event_store: ESDBEventStore,
    pub orders_projection: OrderProjection,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlaceOrderRequest {
    order_type: String,
    line_items: Vec<LineItem>,
    address: Option<Address>
}

#[post("/order")]
async fn place_order(
    order_request: web::Json<PlaceOrderRequest>,
    data: web::Data<WebServer>,
) -> impl Responder {
    let event = data.event_store
        .execute(Uuid::new_v4().to_string(), |order: &Order| {
            order.order_placed(
                order_request.order_type.clone(),
                order_request.line_items.clone(),
                order_request.address.clone(),
            )
        })
        .await;

    match event {
        Ok(event) => println!("GOT EVENT! {:?}", event),
        Err(e) => println!("GOT ERROR {:?}", e),
    }

    HttpResponse::Ok().body(format!("Order: {order:?}", order = order_request))
}

#[get("/order")]
async fn get_orders(data: web::Data<WebServer>) -> impl Responder {
    let all = data.orders_projection.get_all();
    println!("GOT All! {:?}", all);
    HttpResponse::Ok().body("All orders!")
}
