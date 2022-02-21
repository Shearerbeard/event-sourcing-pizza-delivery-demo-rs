use actix_web::{get, post, HttpResponse, Responder, web};
use serde::{Serialize, Deserialize};

use crate::order::aggregate;

#[derive(Serialize, Deserialize, Debug)]
struct PlaceOrderRequest {
    order_type: String,
    line_items: Vec<aggregate::LineItem>,
    notes: Option<String>,
}

#[post("/order")]
async fn place_order(order: web::Json<PlaceOrderRequest>) -> impl Responder {
    HttpResponse::Ok().body(format!("Order: {order:?}", order=order))
}

#[get("/order")]
async fn get_orders() -> impl Responder {
    HttpResponse::Ok().body("All orders!")
}
