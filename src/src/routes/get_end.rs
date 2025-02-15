use actix_web::{get, web, HttpMessage, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::error::Error;

#[derive(Serialize, Debug)]
pub struct Res {
    pub status: &'static str,
    pub data: String
}

#[utoipa::path(
    get,
    path = "/end",
    responses(
        (status = 200, description = "Signup successful", body = GetSubicronResDocs, example = json!({
            "status": "success",
            "data": "UUD"
        })),
        (status = 401, description = "Unauthorized", body = GetSubicronResDocs, example = json!({
            "status": "Invalid premisions",
            "data": ""
        })),
        (status = 400, description = "Bad Request", body = GetSubicronResDocs, example = json!({
            "status": "Bad request data",
            "data": ""
        }))
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Subicron"
)]
#[get("")]
async fn post_start() -> Result<HttpResponse, Error> {


    return Ok(HttpResponse::Ok().json(Res {
        status: "Success",
        data: Some(subicrons),
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