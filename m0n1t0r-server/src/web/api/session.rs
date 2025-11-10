use crate::web::{Response, Result as WebResult};
use actix_web::{
    HttpRequest, Responder, delete, post,
    web::{Form, Json},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct AuthForm {
    _name: String,
}

#[post("/session")]
pub async fn post(_request: HttpRequest, Form(_form): Form<AuthForm>) -> WebResult<impl Responder> {
    // TODO: implement
    Ok(Json(Response::success(())?))
}

#[delete("/session")]
pub async fn delete(_request: HttpRequest) -> WebResult<impl Responder> {
    // TODO: implement
    Ok(Json(Response::success(())?))
}
