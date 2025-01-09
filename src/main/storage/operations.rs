use futures::TryStreamExt;
use mongodb::{
    bson::doc,
    error::Error,
    results::InsertOneResult,
    Collection
};
use crate::model::NewEvent;
use crate::storage::connection;
use crate::storage::db_config::SchemaCollection;

/// https://www.mongodb.com/docs/drivers/rust/current/fundamentals/crud/read-operations/retrieve/
pub async fn save_event(data: &NewEvent) -> Result<InsertOneResult, Error> {
    // ดึง collection ของ Event
    let collection: Collection<NewEvent> = connection(SchemaCollection::Event).await?;

    // บันทึกข้อมูล
    collection
        .insert_one(data)
        .await
}


pub async fn fetch_event() -> Result<Vec<NewEvent>, Error> {
    // เชื่อมต่อกับฐานข้อมูล
    let collection: Collection<NewEvent> = connection(SchemaCollection::Event).await?;

    // ดึงข้อมูลทั้งหมดและจัดเรียงตาม timestamp จากมากไปหาน้อย
    let mut command = collection
        .find(doc! {}).sort(doc! { "timestamp": -1 })
        .await?;

    let mut greetings = Vec::new();
    while let Some(result) = command.try_next().await? {
        greetings.push(result);
    }


    Ok(greetings)
}
