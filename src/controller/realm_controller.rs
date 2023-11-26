use actix_web::{get, HttpResponse, Responder, web};
use crate::AppState;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_realm_by_name);
}

#[get("/role/{id}")]
async fn get_realm_by_name(
    name: web::Path<String>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {

    let a = app_state.context.realms.add_realm("foo").await;

    let x = app_state.context.realms.get_realm_by_name(name.as_str()).await;

    match x {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(realm) => HttpResponse::Ok().json(realm),
    }
}