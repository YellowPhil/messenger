use actix_web::{get, post, web, HttpResponse, Responder};
use actix_session::Session;
use db::models;

mod requests {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateRoomRequest {
        pub name: String,
        pub description: Option<String>,
        pub avatar_url: Option<String>,
    }
}


#[get("/{room_id}")]
async fn get_room(room_id: web::Path<uuid::Uuid>, data: web::Data<crate::AppState>) -> impl Responder{
    match data.db.rooms().get_room(*room_id).await {
        Ok(room) => HttpResponse::Ok().json(room),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/new")]
async fn create_room(data: web::Data<crate::AppState>, session: Session, req: web::Json<requests::CreateRoomRequest>) -> impl Responder{
    // SAFETY: This unwrap is safe because the user_id is guaranteed to be present
    // in the session after successful authentication middleware
    let user_id = session.get::<uuid::Uuid>(crate::USER_ID_KEY).unwrap().unwrap();
    let req = req.into_inner();
    let room_info = models::room::RoomInfo {
        name: req.name,
        description: req.description,
        avatar_url: req.avatar_url,
        role: models::roles::Role::Admin as i32,
    };
    match data.db.rooms().create_room(&room_info, user_id).await {
        Ok(room_id) => HttpResponse::Ok().json(room_id),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}