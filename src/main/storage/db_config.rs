use dotenvy::dotenv;
use lazy_static::lazy_static;
use mongodb::{error::Error, options::ClientOptions, Client, Collection};
use once_cell::sync::OnceCell;
use std::env;
use std::io::ErrorKind;
use std::sync::Arc;
use std::time::Duration;
use actix_web::rt::time;
use tokio::sync::RwLock;
use tracing::{info, warn};


lazy_static! {
    static ref DB_CLIENT: OnceCell<Arc<RwLock<Client>>> = OnceCell::new();
}

// ฟังก์ชันสำหรับสร้าง MongoDB client
async fn create_client() -> Result<Arc<RwLock<Client>>, Error> {
    dotenv().ok();
    let database_url = env::var("MONGODB_URI").map_err(|_| {
        Error::from(std::io::Error::new(
            ErrorKind::InvalidInput,
            "MONGODB_URI is not set",
        ))
    })?;

    // ตั้งค่า ClientOptions
    let mut options = ClientOptions::parse(&database_url).await?;
    options.app_name = Some("ActixMongoApp".to_string());
    options.min_pool_size = Some(4);
    options.max_pool_size = Some(32);
    options.max_idle_time = Some(Duration::from_secs(600));
    options.connect_timeout = Some(Duration::from_secs(30));

    let client = Client::with_options(options)?;

    // ทดสอบการเชื่อมต่อใน loop
    loop {
        match client.list_database_names().await {
            Ok(_) => {
                info!("Connected to MongoDB success...");
                break;
            }
            Err(err) => warn!("Connection failed: {}. Retrying...", err)
        }
    }

    Ok(Arc::new(RwLock::new(client)))
}

// ฟังก์ชันสำหรับ initialize database client
pub async fn init_db() -> Result<(), Error> {
    let client = create_client().await?;
    DB_CLIENT.set(client).map_err(|_| {
        Error::from(std::io::Error::new(
            ErrorKind::Other,
            "Failed to initialize client",
        ))
    })
}

// ดึง Collection จาก DB
#[derive(Debug, Clone, Copy)]
pub enum SchemaCollection {
    Event,
}

impl SchemaCollection {
    fn as_str(&self) -> &'static str {
        match self {
            SchemaCollection::Event => "Event",
        }
    }
}

pub async fn connection<T: Send + Sync>(collection: SchemaCollection) -> Result<Collection<T>, Error> {
    let client = DB_CLIENT
        .get()
        .ok_or_else(|| Error::from(std::io::Error::new(
            ErrorKind::NotConnected,
            "DB not initialized. Call `init_db()` first.",
        )))?
        .read()
        .await;

    let db_name = env::var("DB_NAME").map_err(|_| {
        Error::from(std::io::Error::new(
            ErrorKind::InvalidInput,
            "DB_NAME not set",
        ))
    })?;

    let db = client.database(&db_name);
    Ok(db.collection::<T>(collection.as_str()))
}
