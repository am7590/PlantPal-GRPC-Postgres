use futures::Stream;
use futures_util::io::Empty;
use sqlx_postgres::{PgPool, PgPoolOptions};
use std::borrow::BorrowMut;
use std::pin::Pin;
use std::ptr::null;
use std::string;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status};
use sqlx;
use serde_json::{Value, json};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, TimeZone, Utc};
use crate::plant::{HealthCheckDataRequest, HealthCheckDataResponse};
use serde_json::Value as JsonValue;
use crate::plant::{
    HealthCheckInformation, HistoricalProbabilities, Probabilities, ListOfPlants, PlantInformation 
};

use sqlx::Error;

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
        // let result = sqlx::query!(
        //     "ALTER TABLE health_check ADD UNIQUE (sku);"
        // )
        // .execute(&pool)
        // .await;

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
        println!("add: Received request with {:?}", item);

        let sku = match item.identifier.as_ref() {
            Some(id) if id.sku == "" => return Err(Status::invalid_argument(EMPTY_SKU_ERR)),
            Some(id) => id.sku.to_owned(),
            None => return Err(Status::invalid_argument(NO_ID_ERR)),
        };

        let device_identifier: String = match item.identifier.as_ref() {
            Some(id) if id.device_identifier == "" => return Err(Status::invalid_argument(EMPTY_SKU_ERR)),
            Some(id) => id.device_identifier.to_owned(),
            None => return Err(Status::invalid_argument(NO_ID_ERR)),
        };

        // TODO: check for dups

        let information = item.information.ok_or(Status::invalid_argument("Missing information"))?;
        let name = information.name;
        let last_watered = information.last_watered;
        let last_health_check = information.last_health_check;
        let last_identification = information.last_identification;

        let result = sqlx::query!(
            "INSERT INTO plants (sku, device_identifier, name, last_watered, last_health_check, last_identification) VALUES ($1, $2, $3, $4, $5, $6)
            ",
            sku,
            device_identifier,
            name,
            last_watered,
            last_health_check,
            last_identification,
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => {
                println!("add: success\n");
                // let _ = push::apns::run().await;
                Ok(Response::new(PlantResponse {
                    status: "success".into(),
                }))
            }
            Err(err) => {
                println!("add: error {:?}", err);
                Err(Status::internal(format!("Failed to add item to the database: {}\n", err)))
            },
        }
    }


    async fn remove(
        &self,
        request: Request<PlantIdentifier>,
    ) -> Result<Response<PlantResponse>, Status> {
        let identifier = request.into_inner();
        let sku = identifier.sku;

        println!("remove: Received request with sku: {}", sku);

        let result = sqlx::query!(
            "DELETE FROM plants WHERE sku = $1",
            sku,
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => {
                println!("remove: success\n");
                Ok(Response::new(PlantResponse {
                    status: "success".into(),
                }))
            }
            Err(err) => {
                println!("remove: error {:?}\n", err);
                return Err(Status::internal(format!("Failed to remove plant from the database: {}", err)))
            },
        }
    }

    async fn get(&self, request: Request<PlantIdentifier>) -> Result<Response<Plant>, Status> {
        let identifier = request.into_inner();
        let sku = identifier.sku;
        let device_identifier = identifier.device_identifier;
    
        let result = sqlx::query!(
            "SELECT sku, name, last_watered, last_health_check, last_identification, device_identifier, identifiedspeciesname FROM plants WHERE sku = $1",
            sku,
        )
        .fetch_one(&self.pool)
        .await;
    
        match result {
            Ok(row) => {
                let plant = Plant {
                    identifier: Some(PlantIdentifier { sku, device_identifier }),
                    information: Some(PlantInformation {
                        name: row.name,
                        last_watered: row.last_watered,
                        last_health_check: row.last_health_check,
                        last_identification: row.last_identification,
                        identified_species_name: row.identifiedspeciesname,
                    }),
                };
                println!("get: success\n");
                Ok(Response::new(plant))
            }
            Err(err) => {
                println!("get: error {:?}", err);
                Err(Status::internal(format!("Failed to get plant from the database: {}\n", err)))
            },
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
        let identified_species_name = information.identified_species_name;

    
        let result = sqlx::query!(
            "UPDATE plants SET last_watered = $1, last_health_check = $2, last_identification = $3, identifiedspeciesname = $4 WHERE sku = $5",
            information.last_watered,
            information.last_health_check,
            information.last_identification,
            identified_species_name,
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
            Err(err) => {
                println!("update_plant: error {:?}\n", err);
                Err(Status::internal(format!("Failed to update plant in the database: {}", err)))
            },
        }
    }


    async fn get_watered(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ListOfPlants>, Status> {
        let int64_current_date = SystemTime::now();
        let unix_timestamp = match int64_current_date.duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs() as i64,
            Err(_) => -1, // Handle errors here if necessary
        };

        println!("get watered");

        let result = sqlx::query!(
            "SELECT * FROM plants WHERE last_watered <= $1",
            unix_timestamp,
        )
        .fetch_all(&self.pool)
        .await;        
    
        match result {
            Ok(rows) => {
                let plants = rows
                    .into_iter()
                    .map(|row| {
                        let sku = row.sku.ok_or(Status::internal("Missing SKU"))?;
                        let device_identifier = row.device_identifier.ok_or(Status::internal("Missing Device ID"))?;
                        let name = row.name; //.ok_or(Status::internal("Missing name"))?;
                        let last_watered = row.last_watered; //.ok_or(Status::internal("Missing last_watered"))?;
                        let last_health_check = row.last_health_check; //.ok_or(Status::internal("Missing last_health_check"))?;
                        let last_identification = row.last_identification; //.ok_or(Status::internal("Missing last_identification"))?;
                        let identified_species_name = None;

                        println!("{} is getting a push", sku);

                        Ok::<_, Status>(Plant {
                            identifier: Some(PlantIdentifier { sku, device_identifier }),
                            information: Some(PlantInformation {
                                name,
                                last_watered,
                                last_health_check,
                                last_identification,
                                identified_species_name,
                            }),
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;

    
                let list_of_plants = ListOfPlants { plants };
    
                Ok(Response::new(list_of_plants))
            }
            Err(err) => {
                println!("get_watered: error {:?}\n", err);
                Err(Status::internal(format!(
                "Failed to get plants from the database: {}",
                err
            )))},
        }
    }

    async fn identification_request(
        &self,
        request: Request<PlantIdentifier>,
    ) -> Result<Response<PlantInformation>, Status> {
        let _identifier = request.into_inner();
        println!("identification_request: Received request for sku: {}", _identifier.sku);

        // Return dummy species name as identification information
        let identification_information = PlantInformation {
            name: Some("Dummy Plant Name".to_string()),
            last_watered: Some(1617948000), // Example timestamp
            last_health_check: Some(1617948000), // Example timestamp
            last_identification: Some(1617948000), // Example timestamp
            identified_species_name: Some("Plantae".to_string()),
        };

        println!("identification_request: Success for sku: {}", _identifier.sku);

        Ok(Response::new(identification_information))
    }

    // Responds with health historical data based on the updated proto definition
    async fn health_check_request(
        &self,
        request: Request<PlantIdentifier>,
    ) -> Result<Response<HealthCheckInformation>, Status> {
        let identifier = request.into_inner();
        let sku = identifier.sku;
        let device_identifier = identifier.device_identifier;
    
        println!("health_check_request: Received request for sku: {}, device_identifier: {}", sku, device_identifier);

        let health_check_result = sqlx::query!(
            "SELECT health_check_info FROM health_check WHERE sku = $1 AND device_identifier = $2",
            sku,
            device_identifier
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Status::internal(format!("Database error: {}", e)))?;
    
        match health_check_result {
            Some(result) => {
                if let Some(info) = result.health_check_info {
                    // Clone the info into a String
                    let info_string = info.clone();
    
                    // Deserialize the JSON string into a Value
                    let health_check_json: JsonValue = serde_json::from_str(info_string.as_str().unwrap_or("default string"))
                        .map_err(|e| Status::internal(format!("Failed to parse JSON: {}", e)))?;
    
                    let probability = health_check_json["probability"].as_f64()
                        .ok_or(Status::internal("JSON does not contain 'probability' field"))?;
    
                    let historical_probabilities = health_check_json["historicalProbabilities"]
                        .as_array()
                        .ok_or(Status::internal("JSON does not contain 'historicalProbabilities' array"))?
                        .iter()
                        .map(|prob| {
                            let id = prob["id"].as_str()
                                .ok_or(Status::internal("Missing 'id' in probabilities"))?
                                .to_owned();
                            let name = prob["name"].as_str()
                                .ok_or(Status::internal("Missing 'name' in probabilities"))?
                                .to_owned();
                            let probability = prob["probability"].as_f64()
                                .ok_or(Status::internal("Missing 'probability' in probabilities"))?;
                            let date = prob["date"].as_i64()
                                .ok_or(Status::internal("Missing 'date' in probabilities"))?;
    
                            Ok(Probabilities { id, name, probability, date })
                        })
                        .collect::<Result<Vec<_>, Status>>()?;
    
                        let health_check_info = HealthCheckInformation {
                            probability,
                            historical_probabilities: Some(HistoricalProbabilities{probabilities:historical_probabilities}),
                        };

                println!("health_check_request: Success for sku: {}", sku);
                Ok(Response::new(health_check_info))
                } else {
                    Err(Status::internal("No health check data available"))
                }
            },
            None => {
                println!("health_check_request: No data found for sku: {}", sku);
                Err(Status::not_found("No health check data found for the specified plant"))
            }
        }
    }
    
    

    // async fn save_health_check_data(
    //     &self,
    //     request: Request<HealthCheckDataRequest>,
    // ) -> Result<Response<HealthCheckDataResponse>, Status> {
    //     let data_request = request.into_inner();
    //     let plant_identifier = data_request.identifier.unwrap();
    //     let sku = plant_identifier.sku;
    //     let device_identifier = plant_identifier.device_identifier;
    //     let health_check_info = data_request.health_check_information;
    
    //     // Convert health check info from String to JSON
    //     let health_check_json: JsonValue = serde_json::from_str(&health_check_info)
    //         .map_err(|e| Status::internal(format!("Failed to parse JSON: {}", e)))?;
    
    //     // Upsert health check data using the SKU and device identifier
    //     let upsert_result = sqlx::query!(
    //         "INSERT INTO health_check (sku, device_identifier, health_check_info) VALUES ($1, $2, $3)
    //         ON CONFLICT (sku) DO UPDATE SET health_check_info = EXCLUDED.health_check_info",
    //         sku,
    //         device_identifier,
    //         health_check_json
    //     )
    //     .execute(&self.pool)
    //     .await
    //     .map_err(|e| Status::internal(format!("Failed to upsert health check data: {}", e)))?;
    
    //     Ok(Response::new(HealthCheckDataResponse {
    //         status: "success".into(),
    //     }))
    // }

    async fn save_health_check_data(
        &self,
        request: Request<HealthCheckDataRequest>,
    ) -> Result<Response<HealthCheckDataResponse>, Status> {
        let data_request = request.into_inner();
        let plant_identifier = data_request.identifier.unwrap();
        let sku = plant_identifier.sku;
        let device_identifier = plant_identifier.device_identifier;
        let health_check_info_str = data_request.health_check_information;
    
        println!("save_health_check_data: id: {device_identifier} with data: {health_check_info_str}");

        // Parse the health check information string into JSON
        let health_check_info_json: serde_json::Value = serde_json::from_str(&health_check_info_str)
            .map_err(|e| Status::internal(format!("Failed to parse JSON: {}", e)))?;
    
        // Check if an entry with the given SKU already exists
        let existing_entry = sqlx::query!(
            "SELECT health_check_info FROM health_check WHERE sku = $1",
            sku
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Status::internal(format!("Database query error: {}", e)))?;
    
        let new_historical_entry = json!({
            "id": "new_entry_id", // Generate or fetch an appropriate ID
            "name": "new_entry_name", // Provide a meaningful name
            "probability": health_check_info_json["probability"],
            "date": chrono::Utc::now().timestamp()
        });
    
        if let Some(existing_entry) = existing_entry {
            // Parse the existing entry's health check info
            let existing_info_str = existing_entry.health_check_info.unwrap_or_default().to_string();
            let mut existing_data: serde_json::Value = serde_json::from_str(&existing_info_str)
            .map_err(|e| {
                println!("Failed to parse JSON: {}", e);
                Status::internal(format!("Failed to parse JSON: {}", e))
            })?;

            // Append the new entry to the historical data
            existing_data["historicalProbabilities"].as_array_mut()
                .ok_or(Status::internal("Invalid historicalProbabilities format"))?
                .push(new_historical_entry);
    
            // Update the entry in the database
            let updated_json_str = serde_json::to_string(&existing_data).unwrap();
            let updated_json_value = serde_json::Value::String(updated_json_str);
            sqlx::query!(
                "UPDATE health_check SET health_check_info = $1 WHERE sku = $2",
                updated_json_value,
                sku
            )
            .execute(&self.pool)
            .await
            .map_err(|e| {
                println!("Failed to parse JSON: {}", e);
                Status::internal(format!("Failed to parse JSON: {}", e))
            })?;
        } else {
            // Create new entry for first health check
            let new_data = serde_json::json!({
                "probability": health_check_info_json["probability"],
                "historicalProbabilities": [new_historical_entry],
            });
            let new_json_str = serde_json::to_string(&new_data).unwrap();
            let new_json_str = serde_json::to_string(&new_data).unwrap();
            let new_json_value = serde_json::Value::String(new_json_str);
            sqlx::query!(
                "INSERT INTO health_check (sku, device_identifier, health_check_info) VALUES ($1, $2, $3)",
                sku,
                device_identifier,
                new_json_value
            )
            .execute(&self.pool)
            .await
            .map_err(|e| {
                println!("Failed to parse JSON: {}", e);
                Status::internal(format!("Failed to parse JSON: {}", e))
            })?;
        }
    
        println!("save_health_check_data: success");

        Ok(Response::new(HealthCheckDataResponse {
            status: "success".into(),
        }))
    }  
}

struct HealthCheckData {
    health_check_info: serde_json::Value,  // JSON
}

fn convert_json_to_health_check_info(json: serde_json::Value) -> Result<HealthCheckInformation, Status> {
    
    let probability = json.get("probability")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| Status::internal("Missing probability"))?;

    // Extract the historicalProbabilities from the JSON
    let historical_probabilities_json = json.get("historicalProbabilities")
        .and_then(|v| v.as_array())
        .ok_or_else(|| Status::internal("Missing historicalProbabilities"))?;

    let probabilities = historical_probabilities_json.iter().map(|prob| {
        let id = prob.get("id").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let name = prob.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let probability = prob.get("probability").and_then(|v| v.as_f64())
            .ok_or(Status::internal("Missing probability in historical data"))?;
        let date = prob.get("date").and_then(|v| v.as_i64()).unwrap_or_default();

        Ok(Probabilities { id, name, probability, date })
    }).collect::<Result<Vec<_>, Status>>()?;

    let health_check_info = HealthCheckInformation {
        probability,
        historical_probabilities: Some(HistoricalProbabilities { probabilities }),
    };
    
    println!("{:?}", health_check_info);
    
    Ok(health_check_info)
}
