use actix_web::{get, web, Responder};



#[get("/{room_id}")]
async fn get_room(room_id: web::Path<uuid::Uuid>, data: web::Data<crate::AppState>) -> impl Responder{
    data.db.rooms().
}