use dotenvy::dotenv;
use mongodb::{Client, options::ClientOptions};
use std::env;
use std::time::Duration;


pub async fn init_db() -> mongodb::error::Result<Client> {
    dotenv().ok();

    let database_url = env::var("MONGODB_URI").expect("MONGODB_URI is not set");

    let mut options = ClientOptions::parse(&database_url).await
        .expect("Failed to parse MongoDB URI");

    options.app_name = Some("ActixMongoApp".to_string());
    options.min_pool_size = Some(4);
    options.max_pool_size = Some(32);
    options.max_idle_time = Some(Duration::from_secs(600));
    options.connect_timeout = Some(Duration::from_secs(30));

    let client = Client::with_options(options);

    client
}
