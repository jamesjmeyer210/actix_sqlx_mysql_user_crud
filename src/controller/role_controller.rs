use super::log_request;
use super::AppState;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_role_by_id);
    cfg.service(post_role);
    cfg.service(patch_role_by_name);
    cfg.service(delete_role_by_name);
}

#[get("/role/{id}")]
async fn get_role_by_id(
    group_id: web::Path<i32>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    log_request("GET: /group", &app_state.connections);

    let x = app_state
        .context
        .roles
        .get_role_by_id(group_id.into_inner())
        .await;

    match x {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(group) => HttpResponse::Ok().json(group),
    }
}

#[derive(Deserialize, Serialize)]
pub struct RoleAdd {
    pub role_name: String,
    pub realm_name: String,
    pub max: Option<i32>,
}

#[post("/role")]
async fn post_role(
    role: web::Json<RoleAdd>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    log_request("POST: /role", &app_state.connections);

    let realm = app_state.context.realms
        .get_realm_by_name(role.realm_name.as_str())
        .await;
    if realm.is_err() {
        return HttpResponse::NotFound().finish();
    }
    let realm = realm.unwrap();

    let x = app_state.context.roles.add_role(&realm, role.realm_name.as_str(), &role.max).await;

    match x {
        Ok(_) => {
            let group = app_state
                .context
                .roles
                .get_role_by_name(role.role_name.as_str())
                .await;

            match group {
                Ok(g) => HttpResponse::Accepted().json(g),
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize, Serialize)]
pub struct RoleUpdate {
    pub old: String,
    pub new: String,
}

#[patch("/role")]
async fn patch_role_by_name(
    update: web::Json<RoleUpdate>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    log_request("PATCH: /user", &app_state.connections);

    let x = app_state
        .context
        .roles
        .update_role(&update.old, &update.new)
        .await;

    match x {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(_) => HttpResponse::Accepted().body(update.new.clone()),
    }
}

#[delete("/role/{name}")]
async fn delete_role_by_name(
    name: web::Path<String>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    log_request("DELETE: /group", &app_state.connections);

    let x = app_state.context.roles.delete_role(name.as_str()).await;

    match x {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(_) => HttpResponse::Ok().body(format!("Successfully deleted group {}", name)),
    }
}
