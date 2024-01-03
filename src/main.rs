use std::{net::SocketAddr, str::FromStr};
use tonic::transport::Server;

use server::StorePlant;
use plant::plant_service_server::PlantServiceServer;
use dotenv::dotenv;

pub mod server;
pub mod plant;

mod plant_proto {
   include!("plant.rs");

   pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
      tonic::include_file_descriptor_set!("plant_descriptor");
}

// cargo sqlx prepare --database-url postgres://aqbmkgzhbwtamx:76d2f99d8b483c20682f648d4e82e30564e756d8503eeda43d99453bcf182695@ec2-52-206-14-156.compute-1.amazonaws.com:5432/d3ruob2eu9kscb


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   dotenv().ok();
   let port = std::env::var("PORT").unwrap_or(String::from("8080"));
   // let addr = SocketAddr::from_str(&format!("0.0.0.0:{}", port)).unwrap();
   let addr = "127.0.0.1:9001".parse()?;
   let database_url = "postgres://aqbmkgzhbwtamx:76d2f99d8b483c20682f648d4e82e30564e756d8503eeda43d99453bcf182695@ec2-52-206-14-156.compute-1.amazonaws.com:5432/d3ruob2eu9kscb";
   let inventory = StorePlant::new(database_url).await?;

   let reflection_service = tonic_reflection::server::Builder::configure()
           .register_encoded_file_descriptor_set(plant_proto::FILE_DESCRIPTOR_SET)
           .build()
           .unwrap();

   Server::builder()
           .add_service(PlantServiceServer::new(inventory))
           .add_service(reflection_service)
           .serve(addr)
           .await?;
   Ok(())
}
