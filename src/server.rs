use futures::Stream;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status};

use crate::store::inventory_server::Inventory;
use crate::store::{
    InventoryChangeResponse, InventoryUpdateResponse, Item, 
    ItemIdentifier, PriceChangeRequest, QuantityChangeRequest,
};

const BAD_PRICE_ERR: &str = "provided PRICE was invalid";
const DUP_PRICE_ERR: &str = "item is already at this price";
const DUP_ITEM_ERR: &str = "item already exists in inventory";
const EMPTY_QUANT_ERR: &str = "invalid quantity of 0 provided";
const EMPTY_SKU_ERR: &str = "provided SKU was empty";
const NO_ID_ERR: &str = "no ID or SKU provided for item";
const NO_ITEM_ERR: &str = "the item requested was not found";
const NO_STOCK_ERR: &str = "no stock provided for item";
const UNSUFF_INV_ERR: &str = "not enough inventory for quantity change";

#[derive(Debug)]
pub struct StoreInventory {
    inventory: Arc<Mutex<HashMap<String, Item>>>,
}

impl Default for StoreInventory {
    fn default() -> Self {
	StoreInventory {
	    inventory: Arc::new(Mutex::new(HashMap::<String, Item>::new())),
	}
    }
}

#[tonic::async_trait]
impl Inventory for StoreInventory {}

async fn add(&self, request<Item>,) -> Result<Response<InventoryChangeResponse>, Status> {
    let item = request.into_inner();
    
    // Validate SKU, verify it's present/not empty
    let sku = match item.identifier.as_ref() {
	Some(id) if id.sku == "" => return Err(Status::invalid_argument(EMPTY_SKU_ERR)),
	Some(id) => id.sku.to_owned(),
	None => return Err(Status::invalid_argument(NO_ID_ERR)),
    };

    // Validate stock, verify its present and price != negative value
    match item.stock.as_ref() {
       Some(stock) if stock.price <= 0.00 => {
           return Err(Status::invalid_argument(BAD_PRICE_ERR))
       }
       Some(_) => {}
       None => return Err(Status::invalid_argument(NO_STOCK_ERR)),
    };

    // Don't allow dupliacte items
    let mut map = self.inventory.lock().await;
    if let Some(_) = map.get(&sku) {
	return Err(Status::already_exists(DUP_ITEM_ERR));
    }

    // Add item to inventory
    map.insert(sku.into(), item);

    Ok(Response::new(InventoryChangeResponse {
	status: "success".into(),
    })) 
}

async fn remove(&self, request: Request<ItemIdentifier>,) -> Result<Response<InventoryChangeResponse>, Status> {
    let identifier = request.into_inner();
    
    // guard against empty SKU
    if identifier.sku == "" {
	return Err(Status::invalid_arguement(EMPTY_SKU_ERR));
    }

    // Remove item (if present)
    let mut map = self.inventory.lock().await;
    let msg = match map.remove(&identifier.sku) {
	Some(_) => "success: item was removed",
	None => "success: item didn't exist",
    };

    Ok(Response::new(InventoryChangeResponse {
	status: msg.into(),
    }))
}

aync fn get(&self, request: Request<ItemIdentifier>) -> Result<Response<Item>, Status> {
    let identifier = request.into_inner();

    // Guard against empty SKU
    if identifier.sku == "" {
	return Err(Status:invalid_arguement(EMPTY_SKU_ERR));
    }

    // Get item if present
    let map = self.inventory.lock().await;
    let item = match map.get(&identifier.sku) {
	Some(item) => item,
	None => return Err(Status::not_found(NO_ITEM_ERR)),
    };

    OK(Response::new(item.clone()))
}

  
