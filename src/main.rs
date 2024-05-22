use actix_web::{web, App, HttpResponse, HttpServer};

mod admin;
mod api;

async fn front_page() -> HttpResponse {
    HttpResponse::Ok().body("Mi Inicio!")
}

fn main_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(front_page));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(main_config)
            .service(web::scope("/api").configure(api::config))
            .service(web::scope("admin").configure(admin::config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
