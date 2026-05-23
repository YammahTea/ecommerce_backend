use std::collections::HashMap;
use sqlx::{Pool, Postgres};
use tracing::{debug, error, instrument, warn};
use uuid::{Uuid};
use crate::errors::cart_error::CartError;
use crate::models::cart::{AddToCartResponse, CartItem, CartItemDetail, GetCartResponse};
use crate::models::product::ProductsInfoForPurchase;
use crate::repositories::cart_repo::{get_cart, save_cart};
use crate::repositories::product_repo::{get_products_ids_and_stock_quantity, get_products_info_for_purchase};

#[instrument(skip(redis_pool, db_pool, items), fields(user_id = %user_id))]
pub async fn add_items_to_cart(redis_pool: &deadpool_redis::Pool, db_pool: &Pool<Postgres>, user_id: &Uuid, items: Vec<CartItem>) -> Result<AddToCartResponse, CartError> {

    let mut redis_connection = redis_pool.get().await
        .map_err(|error_message| {
            error!(error = ?error_message, "Failed to get Redis connection");
            CartError::RedisConnectionError
        })?;


    // bulk fetch only the products that were sent by the user from the db,
    // this also implicitly validates the product ids since invalid ones simply won't be returned
    let products: Vec<Uuid> = items.iter().map(|item| item.product_id).collect();
    let products_info_from_db = get_products_ids_and_stock_quantity(&db_pool, products).await?;

    debug!("Passed products");
    debug!(products_info_db = ?products_info_from_db, "Info for products from db");

    // map the db results to a hashmap for O(1) lookups instead of scanning the vec every iteration
    let mut inventory_map: HashMap<String, i32> = HashMap::new();
    for product in &products_info_from_db {
        inventory_map.insert(product.id.to_string(), product.stock_quantity);
    }

    // fetch the user's current cart from redis to calculate the new quantities
    // if the cart is empty, get_cart returns an empty hashmap so the logic below still works
    let redis_cart_map: HashMap<String, i32> = get_cart(&mut redis_connection, &user_id).await?;

    debug!("Passed redis cart 'get_cart'");
    debug!(cart_map_from_redis = ?redis_cart_map, "Info for products from db");

    let mut approved_items: HashMap<String, i32> = HashMap::new();
    let mut rejected_items: HashMap<String, i32> = HashMap::new();

    for item in items {
        let current_item_id = item.product_id.to_string();

        if inventory_map.contains_key(&current_item_id) {
            let stock = inventory_map[&current_item_id];

            // if the item already exists in the cart, add to the existing quantity
            // instead of replacing it, so the user doesnt lose what they already had
            let mut target_quantity: i32 = item.quantity as i32;
            if let Some(existing_quantity) = redis_cart_map.get(&current_item_id) {
                target_quantity += existing_quantity;
            }

            if target_quantity <= stock {
                approved_items.insert(current_item_id, target_quantity);
            } else {
                // store the difference so the frontend can tell the user
                // the maximum they can still add, not just that it failed
                let difference_quantity = stock - target_quantity;
                rejected_items.insert(current_item_id, difference_quantity);
            }
        } else {
            // rare case
            // product id does not exist in the db, frontend uses 0 as a signal
            // that the item itself is invalid, not just the quantity
            rejected_items.insert(current_item_id, 0);
        }
    }

    // hset_multiple does not accept a hashmap directly so it got converted here,
    // used iter() instead of into_iter() to keep approved_items alive for the return value
    let approved_items_vec: Vec<(String, i32)> = approved_items
        .iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();

    debug!("Before saving approved items to redis");
    debug!(approved_items_list = ?approved_items_vec, "Approved items");

    save_cart(&mut redis_connection, user_id, approved_items_vec).await?;

    debug!("Passed saving process");

    Ok(AddToCartResponse {
        approved_items,
        rejected_items
    })
}

#[instrument(skip(redis_pool, db_pool), fields(user_id = %user_id))]
pub async fn get_items_from_cart(redis_pool: &deadpool_redis::Pool, db_pool: Pool<Postgres>, user_id: &Uuid) -> Result<GetCartResponse, CartError> {

    let mut redis_connection = redis_pool.get().await
        .map_err(|error_message| {
            error!(error = ?error_message, "Failed to get Redis connection");
            CartError::RedisConnectionError
        })?;

    let items_in_cart = get_cart(&mut redis_connection, &user_id).await?;

    let products: Vec<Uuid> = items_in_cart
        .iter()
        .filter_map(|(k, _)| Uuid::parse_str(k).ok()) // for the app to not crash if a database migration happened with a malformed id
        .collect();

    let products_info_from_db = get_products_info_for_purchase(&db_pool, products).await?;

    // to improve look up
    let products_map: HashMap<String, &ProductsInfoForPurchase> = products_info_from_db
        .iter()
        .map(|p| (p.id.clone().to_string(), p))
        .collect();

    let mut items_combined_info: Vec<CartItemDetail> = Vec::new();
    let mut grand_total: i32 = 0; // sum of line_total (quantity * price)

    // looping through items_in_cart is okay because the cart will only have the correct ids when it was saved

    for item in items_in_cart {
        let current_item_id = item.0;
        let current_item_quantity = item.1;
        let current_item_info_from_db = products_map.get(&current_item_id);

        // An option needs to be handled because if for example and item was marked 'archived' after the user had
        // added to their cart and then requested '/GET cart', there will be a missing item in the items
        // fetched from the database for the item that was marked 'archived'


        if let Some(db_info) = current_item_info_from_db {
            // quantity * price
            let current_line_total = current_item_quantity * db_info.price_in_cents;

            grand_total += current_line_total;


            items_combined_info.push(CartItemDetail {
                product_id: current_item_id,
                quantity: current_item_quantity,
                name: db_info.name.clone(),
                unit_price_in_cents: db_info.price_in_cents,
                line_total_in_cents: current_line_total
            })
        }

        else {
            // TODO: Send a notification to the frontend if an item was skipped from being added to the cart
            warn!("The item with the product_id = {} was skipped, it may have been marked as 'archived' or it doesn't exist",current_item_id);
            continue;
        }

    }

    let cart_response: GetCartResponse = GetCartResponse {
        items: items_combined_info,
        grand_total_in_cents: grand_total
    };

    Ok(cart_response)
}
