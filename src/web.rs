use actix_web::{get, post, web, HttpResponse, Responder};

use crate::api::{ChangeOrderStatusArgs, OrderService, PlaceOrderArgs};

#[post("/order")]
async fn place_order(
    order_request: web::Json<PlaceOrderArgs>,
    service: web::Data<OrderService>,
) -> impl Responder {
    let data = order_request.into_inner();
    let event = service.command_place_order(data.clone()).await;

    match event {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::BadRequest().body(format!("Error: {:?}", e)),
    }
}

#[post("/order/status")]
async fn update_order_status(
    order_update_request: web::Json<ChangeOrderStatusArgs>,
    service: web::Data<OrderService>,
) -> impl Responder {
    let data = order_update_request.clone();
    let event = service
        .command_change_order_status(data)
        .await;

    match event {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::BadRequest().body(format!("Error: {:?}", e)),
    }
}

#[get("/order")]
async fn get_orders(service: web::Data<OrderService>) -> impl Responder {
    let all = service.read_all_orders().await;

    match all {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::BadRequest().body(format!("Error: {:?}", e)),
    }
}

#[get("/order/{order_id}")]
async fn get_order(
    path: web::Path<String>,
    service: web::Data<OrderService>
) -> impl Responder {
    let id = path.into_inner();
    let res = service.read_order(id).await;

    match res {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(e) => HttpResponse::BadRequest().body(format!("Error: {:?}", e)),
    }
}
