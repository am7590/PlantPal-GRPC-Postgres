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
   let database_url = "postgres://ztrdkufeonducd:2e5880a3b5d3a7b5af08f115caad38ba2c37e22cb394808c8583b157db106e80@ec2-34-202-53-101.compute-1.amazonaws.com:5432/dancbqcggc4md3";
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
