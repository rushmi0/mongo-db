use actix_web::{get, post, web, HttpResponse, Responder};
use crate::module::{greet, save_event};
use crate::module::greet::fetch_event;
use crate::model::{Greeting, NewEvent};

#[post("/save/{name}")]
pub async fn save_event_service(event: web::Json<NewEvent>, name: web::Path<String>) -> impl Responder {
    let greeting_message: String = greet(&name).await;
    let created_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as u32;

    let response = Greeting {
        timestamp: created_at,
        message: greeting_message,
    };

    let save_data = NewEvent {
        name: event.name.clone(),
        description: event.description.clone(),
        timestamp: event.timestamp.clone(),
    };

    // ใช้ match หรือ map_err เพื่อจัดการกับข้อผิดพลาด
    match save_event(&save_data).await {
        Ok(_) => HttpResponse::Ok()
            .content_type("application/json")
            .json(response),
        Err(e) => {
            // ถ้าเกิดข้อผิดพลาดในการบันทึก greeting ให้คืนค่าผลลัพธ์เป็น HttpResponse::InternalServerError
            HttpResponse::InternalServerError().json(format!("Database error: {}", e))
        }
    }
}


#[get("/fetch/event")]
pub async fn fetch_event_service() -> impl Responder {
    match fetch_event().await {
        Ok(greetings) => HttpResponse::Ok().json(greetings),
        Err(e) => HttpResponse::InternalServerError().json(format!("Database error: {}", e)),
    }
}
