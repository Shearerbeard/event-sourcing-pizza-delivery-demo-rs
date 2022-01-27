use uuid::Uuid;

pub struct OrderLineItem {
    item_id: Uuid,
    quantity: i32,
    notes: Option<String>,
}

pub struct Address {
    address_1: String,
    address_2: Option<String>,
    city: String,
    state: String,
    zip: String,
}

pub enum OrderType {
    Delivery(Address),
    CarryOut,
}

pub enum OrderStatus {
    Preparing,
    InOven,
    EnRoute,
    Delivered,
}
