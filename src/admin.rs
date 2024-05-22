use actix_web::{web, HttpResponse};

async fn login() -> HttpResponse {
    HttpResponse::Ok().body("Login Form")
}

async fn init() -> HttpResponse {
    HttpResponse::Ok().body("Hola Admin!")
}

async fn usuarios() -> HttpResponse {
    HttpResponse::Ok().body("Lista de usuarios")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("login", web::get().to(login))
        .route("init", web::get().to(init))
        .route("usuarios", web::get().to(usuarios));
}
