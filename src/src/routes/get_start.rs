use actix_web::{get, web, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;

use crate::{error::Error, libs::session::{create_session::create_session, insure_sessions::insure_sessions}, SharedSessions};

#[derive(Serialize, Debug)]
pub struct Res {
    pub status: &'static str,
    pub data: String
}

#[utoipa::path(
    get,
    path = "/start",
    responses(
        (status = 200, description = "starting the game", body = GetSubicronResDocs, example = json!({
            "status": "success",
            "data": "UUID"
        }))
    )
)]
#[get("")]
async fn get_start(sessions: web::Data<SharedSessions>) -> Result<HttpResponse, Error> {

    insure_sessions(&sessions).await;

    return Ok(HttpResponse::Ok().json(Res {
        status: "Success",
        data: create_session(&sessions).await,
    }))

}


#[derive(Serialize, ToSchema)]
struct GetSubicronResDocs {
    status: &'static str,
    data: Option<Vec<SubicronSearchResDocs>>
}

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct SubicronSearchResDocs {
    pub image_id: i64,
    pub name: String,
    #[schema(example = "2025-01-22T15:04:05")]
    pub created_at: String,
    pub subicron_id: i64
}