
pub mod apns {
    use std::fs::File;
    use a2::{NotificationOptions, DefaultNotificationBuilder, NotificationBuilder, Client, Endpoint};

    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        println!("Pushing him...");
        
        let topic: Option<String> = Some("com.alek.SucculentAndPlantApp".to_owned());
    
        let options = NotificationOptions {
            apns_topic: topic.as_deref(),
            ..Default::default()
        };
    
        let builder = DefaultNotificationBuilder::new()
            .set_body("Hi there")
            .set_badge(420)
            .set_category("cat1")
            .set_sound("ping.flac");
    
        let payload = builder.build("47c3d1239a3242d1a7768ae81daa9cde5c133d9b13d13e5b30520c7b4b0a9170", options);
        let mut file = File::open("./src/server/private_key.p8")?;
    
        let client = Client::token(
            &mut file,
            "RNL77KG567",
            "7W5ZSCHZ2W",
            Endpoint::Sandbox)?;
    
        let response = client.send(payload).await?;
        println!("Sent: {:?}", response);
    
        Ok(())
    }
}

