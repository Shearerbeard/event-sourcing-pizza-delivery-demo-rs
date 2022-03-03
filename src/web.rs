use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use thalo::event_store::EventStore;
use uuid::Uuid;

use crate::{order::{
    aggregate::{LineItem, OrderCommand, Order, Address},
}, api::OrderService};

#[derive(Serialize, Deserialize, Debug)]
struct PlaceOrderRequest {
    order_type: String,
    line_items: Vec<LineItem>,
    address: Option<Address>
}

#[post("/order")]
async fn place_order(
    order_request: web::Json<PlaceOrderRequest>,
    service: web::Data<OrderService>,
) -> impl Responder {
    let event = service.event_store
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
async fn get_orders(service: web::Data<OrderService>) -> impl Responder {
    let all = service.orders_projection.get_all();
    println!("GOT All! {:?}", all);
    HttpResponse::Ok().body("All orders!")
}
