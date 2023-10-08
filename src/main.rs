use tonic::transport::Server;

use server::StorePlant;
use plant::plant_service_server::PlantServiceServer;

pub mod server;
pub mod plant;

mod plant_proto {
   include!("plant.rs");

   pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
      tonic::include_file_descriptor_set!("plant_descriptor");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   let addr = "127.0.0.1:9001".parse()?;
   let inventory = StorePlant::default();

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
