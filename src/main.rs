mod types;

use types::{OrderStatus, OrderLineItem, OrderType};


struct Order {
    id: String,
    order_status: OrderStatus,
    line_items: Vec<OrderLineItem>,
    order_type: OrderType,
}

fn main() {
    println!("Hello, world!");
}
