use futures::Stream;
use sqlx_postgres::{PgPool, PgPoolOptions};
use std::borrow::BorrowMut;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status};
use sqlx;
use chrono::{DateTime, TimeZone, Utc};

mod push;

use crate::plant::plant_service_server::PlantService;
use crate::plant::{
    PlantResponse, PlantUpdateResponse, Plant, PlantIdentifier, PlantUpdateRequest,
};

const DUP_ITEM_ERR: &str = "plant already exists";
const EMPTY_SKU_ERR: &str = "provided SKU was empty";
const NO_ID_ERR: &str = "no ID or SKU provided for plant";
const NO_ITEM_ERR: &str = "the item requested was not found";
const WOMP_ERR: &str = "womp womp...";

#[derive(Debug)]
pub struct StorePlant {
    pool: PgPool,
}

impl StorePlant {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .connect(database_url)
            .await?;

        Ok(StorePlant { pool })
    }
}

fn convert_i64_to_date(timestamp: i64) -> DateTime<Utc> {
    let utc = Utc.timestamp(timestamp, 0);
    utc
}  

#[tonic::async_trait]
impl PlantService for StorePlant {  

    async fn add(
        &self,
        request: Request<Plant>,
    ) -> Result<Response<PlantResponse>, Status> {
        let item = request.into_inner();

        let sku = match item.identifier.as_ref() {
            Some(id) if id.sku == "" => return Err(Status::invalid_argument(EMPTY_SKU_ERR)),
            Some(id) => id.sku.to_owned(),
            None => return Err(Status::invalid_argument(NO_ID_ERR)),
        };

        
        // let mut conn = self.pool.acquire().await.map_err(|err| Status::internal(err.into()))?;
        

        // let result = sqlx::query!("select (1) as id, 'Herp Derpinson' as name")
        // .fetch_one(&mut conn)
        // .await?;
        let information = item.information.ok_or(Status::invalid_argument("Missing information"))?;
        let name = information.name;
        let last_watered = information.last_watered;
        let last_health_check = information.last_health_check;
        let last_identification = information.last_identification;

        let result = sqlx::query!(
            "INSERT INTO plants (sku, name, last_watered, last_health_check, last_identification) VALUES ($1, $2, $3, $4, $5)",
            sku,
            name,
            last_watered,
            last_health_check,
            last_identification,
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => {
                println!("Add item");
                let _ = push::apns::run().await;
                Ok(Response::new(PlantResponse {
                    status: "success".into(),
                }))
            }
            Err(err) => Err(Status::internal(format!("Failed to add item to the database: {}", err))),
        }
    }


    async fn remove(
        &self,
        request: Request<PlantIdentifier>,
    ) -> Result<Response<PlantResponse>, Status> {
        let identifier = request.into_inner();

        // guard against empty SKU
        // if identifier.sku == "" {
            return Err(Status::invalid_argument(EMPTY_SKU_ERR));
        // }

        // Remove item (if present)
        // let mut map = self.plant.lock().await;
        // let msg = match map.remove(&identifier.sku) {
        //     Some(_) => "success: item was removed",
        //     None => "success: item didn't exist",
        // };

        // Ok(Response::new(PlantResponse {
        //     status: msg.into(),
        // }))
    }

    async fn get(&self, request: Request<PlantIdentifier>) -> Result<Response<Plant>, Status> {
        let identifier = request.into_inner();

        // Guard against empty SKU
        // if identifier.sku == "" {
            return Err(Status::invalid_argument(EMPTY_SKU_ERR));
        // }

        // Get item if present
        // let map = self.plant.lock().await;
        // let item = match map.get(&identifier.sku) {
        //     Some(item) => item,
        //     None => return Err(Status::not_found(NO_ITEM_ERR)),
        // };

        // Ok(Response::new(item.clone()))
    }

    // async fn update_quantity(
    //     &self,
    //     request: Request<QuantityChangeRequest>,
    // ) -> Result<Response<PlantUpdateResponse>, Status> {
    //     let change = request.into_inner();

    //     // guard against empty sku
    //     if change.sku == "" {
    //         return Err(Status::invalid_argument(EMPTY_SKU_ERR));
    //     }

    //     // guard against no change
    //     if change.change == 0 {
    //         return Err(Status::invalid_argument(EMPTY_QUANT_ERR));
    //     }

    //     // get item data
    //     let mut map = self.plant.lock().await;
    //     let item = match map.get_mut(&change.sku) {
    //         Some(item) => item,
    //         None => return Err(Status::not_found(NO_ITEM_ERR)),
    //     };

    //     // get stock mutable to update quantity
    //     let mut stock = match item.stock.borrow_mut() {
    //         Some(stock) => stock,
    //         None => return Err(Status::internal(NO_STOCK_ERR)),
    //     };

    //     // validate and handle quantity change
    //     stock.quantity = match change.change {
    //         change if change < 0 => {
    //             if change.abs() as u32 > stock.quantity {
    //                 return Err(Status::resource_exhausted(UNSUFF_INV_ERR));
    //             }
    //             stock.quantity - change.abs() as u32
    //         }

    //         change => stock.quantity + change as u32,
    //     };

    //     Ok(Response::new(PlantUpdateResponse {
    //         status: "success".into(),
    //         price: stock.price,
    //         quantity: stock.quantity,
    //     }))
    // }

    async fn update_plant(
        &self,
        request: Request<PlantUpdateRequest>,
    ) -> Result<Response<PlantUpdateResponse>, Status> {
        let change = request.into_inner();

        // guard against empty sku
        // if change.sku == "" {
            return Err(Status::invalid_argument(EMPTY_SKU_ERR));
        // }

        // guard against 0 or negative price change
        // if change.price <= 0.0 {
        //     return Err(Status::invalid_argument(BAD_PRICE_ERR));
        // }

        // get item data
        // let mut map = self.plant.lock().await;
        // let item = match map.get_mut(&change.sku) {
        //     Some(item) => item,
        //     None => return Err(Status::not_found(NO_ITEM_ERR)),
        // };

        // get stock mutable to update quantity
        // let mut stock = match item.identifier.borrow_mut() {
        //     Some(stock) => stock,
        //     None => return Err(Status::internal(WOMP_ERR)),
        // };

        // stock.price = change.price;

        // Ok(Response::new(PlantUpdateResponse {
        //     status: "success".into(),
        // }))
    }

    // type WatchStream = Pin<Box<dyn Stream<Item = Result<Plant, Status>> + Send>>;

    // async fn watch(
    //     &self,
    //     request: Request<PlantIdentifier>,
    // ) -> Result<Response<Self::WatchStream>, Status> {
    //     // retrieve the relevant item and get a baseline
    //     let id = request.into_inner();
    //     let mut item = self.get(Request::new(id.clone())).await?.into_inner();

    //     // the channel will be our stream back to the client, we'll send copies
    //     // of the requested item any time we notice a change to it in the
    //     // inventory.
    //     let (tx, rx) = mpsc::unbounded_channel();

    //     // we'll loop and poll new copies of the item until either the client
    //     // closes the connection, or an error occurs.
    //     let plant = self.plant.clone();
    //     tokio::spawn(async move {
    //         loop {
    //             // it's somewhat basic, but for this demo we'll just check the
    //             // item every second for any changes.
    //             tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    //             // pull a fresh copy of the item in the inventory
    //             let map = plant.lock().await;
    //             let item_refresh = match map.get(&id.sku) {
    //                 Some(item) => item,
    //                 // the item has been removed from the inventory. Let the
    //                 // client know, and stop the stream.
    //                 None => {
    //                     if let Err(err) = tx.send(Err(Status::not_found(NO_ITEM_ERR))) {
    //                         println!("ERROR: failed to update stream client: {:?}", err);
    //                     }
    //                     return;
    //                 }
    //             };

    //             // check to see if the item has changed since we last saw it,
    //             // and if it has inform the client via the stream.
    //             if item_refresh != &item {
    //                 if let Err(err) = tx.send(Ok(item_refresh.clone())) {
    //                     println!("ERROR: failed to update stream client: {:?}", err);
    //                     return;
    //                 }
    //             }

    //             // cache the most recent copy of the item
    //             item = item_refresh.clone()
    //         }
    //     });

    //     let stream = UnboundedReceiverStream::new(rx);
    //     Ok(Response::new(Box::pin(stream) as Self::WatchStream))
    // }
}
