use crate::web::{Response, Result as WebResult};
use actix_identity::Identity;
use actix_web::{
    HttpMessage, HttpRequest, Responder, post,
    web::{Form, Json},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct AuthForm {
    name: String,
}

#[post("/auth")]
pub async fn post(request: HttpRequest, Form(form): Form<AuthForm>) -> WebResult<impl Responder> {
    // TODO: implement
    Identity::login(&request.extensions(), form.name)?;
    Ok(Json(Response::success(())?))
}
