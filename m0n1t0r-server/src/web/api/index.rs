use crate::web::Result as WebResult;
use actix_web::{get, web::Redirect, Responder};

#[get("/")]
pub async fn get() -> WebResult<impl Responder> {
    Ok(Redirect::to("/server").permanent())
}
