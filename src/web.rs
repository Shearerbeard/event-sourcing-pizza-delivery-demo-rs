use actix_web::{get, post, web, HttpResponse, Responder};

use crate::api::{PlaceOrderArgs, OrderService};


#[post("/order")]
async fn place_order(
    order_request: web::Json<PlaceOrderArgs>,
    service: web::Data<OrderService>,
) -> impl Responder {
    let data = order_request.into_inner();
    let event = service.command_place_order(data.clone()).await;

    match event {
        Ok(event) => println!("GOT EVENT! {:?}", event),
        Err(e) => println!("GOT ERROR {:?}", e),
    }

    HttpResponse::Ok().body(format!("Order: {order:?}", order = data))
}

#[get("/order")]
async fn get_orders(service: web::Data<OrderService>) -> impl Responder {
    let all = service.read_all_orders().await.unwrap();
    println!("GOT All! {:?}", all);
    HttpResponse::Ok().body("All orders!")
}
