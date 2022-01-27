aggregate Order {
    order_placed(order_type: String!, line_items: [LineItem!]!, address: Address): OrderPlaced!
    order_status_changed(id: String!, order_status: String!): OrderStatusChanged!
}

event OrderPlaced {
    id: String!
    line_items: [LineItem!]!
    order_type: String!
    order_status: String!
    address: Address
}

event OrderStatusChanged {
    id: String!
    order_status: String!
}

type LineItem {
    item_id: String!
    quantity: Int!
    notes: String
    address: Address
}

type Address {
    address_1: String!
    address_2: String
    city: String!
    state: String!
    zip: String!
}