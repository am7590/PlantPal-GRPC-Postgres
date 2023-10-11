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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   dotenv().ok();
   let addr = "127.0.0.1:9001".parse()?;
   let database_url = "postgres://odketbknkrveew:265f2a852242377ba006b0f6250ed8fbf27c0c7bf1ee1a352835c9f3a7e85646@ec2-44-213-228-107.compute-1.amazonaws.com:5432/d96remn27v216p
   ";
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
