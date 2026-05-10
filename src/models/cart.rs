use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AddToCartRequest {
    pub items: Vec<CartItem>
}

// saved in redis
#[derive(Debug, Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: Uuid,
    pub quantity: u32
}

// combined data sent back to frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct GetCartResponse {
    pub items: Vec<CartItemDetail>,
    pub grand_total_in_cents: i32 // sum of line_total_cents
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CartItemDetail {
    pub product_id: String, // it is stored as a Uuid in the cart, but it is okay to be extracted as string
    pub quantity: i32,
    pub name: String,
    pub unit_price_in_cents: i32,
    pub line_total_in_cents: i32 // quantity * unit_price_in_cents
}

#[derive(Debug, Serialize)]
pub struct AddToCartResponse {
    pub approved_items: HashMap<String, i32>,
    pub rejected_items: HashMap<String, i32>
}