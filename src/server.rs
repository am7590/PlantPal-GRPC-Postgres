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
use crate::plant::PlantInformation;


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

        // TODO: check for dups

        let information = item.information.ok_or(Status::invalid_argument("Missing information"))?;
        let name = information.name;
        let last_watered = information.last_watered;
        let last_health_check = information.last_health_check;
        let last_identification = information.last_identification;

        let result = sqlx::query!(
            "INSERT INTO plants (sku, name, last_watered, last_health_check, last_identification) VALUES ($1, $2, $3, $4, $5)
            ",
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
        let sku = identifier.sku;

        let result = sqlx::query!(
            "DELETE FROM plants WHERE sku = $1",
            sku,
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => {
                println!("Remove plant");
                Ok(Response::new(PlantResponse {
                    status: "success".into(),
                }))
            }
            Err(err) => Err(Status::internal(format!("Failed to remove plant from the database: {}", err))),
        }
    }

    async fn get(&self, request: Request<PlantIdentifier>) -> Result<Response<Plant>, Status> {
        let identifier = request.into_inner();
        let sku = identifier.sku;

        let result = sqlx::query!(
            "SELECT * FROM plants WHERE sku = $1",
            sku,
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(row) => {
                let plant = Plant {
                    identifier: Some(PlantIdentifier { sku }),
                    information: Some(PlantInformation {
                        name: row.name,
                        last_watered: row.last_watered,
                        last_health_check: row.last_health_check,
                        last_identification: row.last_identification,
                    }),
                };

                Ok(Response::new(plant))
            }
            Err(err) => Err(Status::internal(format!("Failed to get plant from the database: {}", err))),
        }
    }

    
    async fn update_plant(
        &self,
        request: Request<PlantUpdateRequest>,
    ) -> Result<Response<PlantUpdateResponse>, Status> {
        let update_request = request.into_inner();
    
        let sku = match update_request.identifier.as_ref() {
            Some(id) if id.sku == "" => return Err(Status::invalid_argument(EMPTY_SKU_ERR)),
            Some(id) => id.sku.to_owned(),
            None => return Err(Status::invalid_argument(NO_ID_ERR)),
        };
    
        let information = update_request.information.ok_or(Status::invalid_argument("Missing information"))?;
        let last_watered = information.last_watered;
        let last_health_check = information.last_health_check;
        let last_identification = information.last_identification;
    
        let result = sqlx::query!(
            "UPDATE plants SET last_watered = $1, last_health_check = $2, last_identification = $3 WHERE sku = $4",
            last_watered,
            last_health_check,
            last_identification,
            sku,
        )
        .execute(&self.pool)
        .await;
    
        match result {
            Ok(_) => {
                println!("Update plant");
                Ok(Response::new(PlantUpdateResponse {
                    status: "success".into(),
                }))
            }
            Err(err) => Err(Status::internal(format!("Failed to update plant in the database: {}", err))),
        }
    }
}
