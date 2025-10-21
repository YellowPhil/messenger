use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CreateMessageRequest {
    from: String,
    to: String,
    content: String,
}
// #[post("/send")]
// async fn create_message(message: web::Json<CreateMessageRequest>) -> impl Responder {
//     // HttpResponse::Ok().json(&*crate::USER_ID_KEY)
// }

// #[post("/new")]
// async fn list_messages(username: web::Path<String>) -> impl Responder {
// }