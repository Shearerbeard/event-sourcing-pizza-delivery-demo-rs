use actix_web::{get, post, HttpResponse, Responder, web};
use serde::{Serialize, Deserialize};
use thalo_eventstoredb::ESDBEventStore;

use crate::order::aggregate;

#[derive(Clone)]
pub struct WebServer {
    pub event_store: ESDBEventStore,
    // projection: OrderProjection
}

#[derive(Serialize, Deserialize, Debug)]
struct PlaceOrderRequest {
    order_type: String,
    line_items: Vec<aggregate::LineItem>,
    notes: Option<String>,
}

#[post("/order")]
async fn place_order(order: web::Json<PlaceOrderRequest>, _data: web::Data<WebServer>) -> impl Responder {
    HttpResponse::Ok().body(format!("Order: {order:?}", order=order))
}

#[get("/order")]
async fn get_orders(_data: web::Data<WebServer>) -> impl Responder {
    HttpResponse::Ok().body("All orders!")
}
