use actix_web::{
    get,
    http::header::{self, ContentType},
    web, HttpResponse,
};

#[get("/usuarios")]
async fn get_usuarios() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(header::ContentType::json())
        .body(
            r#"
        [
            {
                "id": 1,
                "nombre": "Rusty"
            },
            {
                "id": 2,
                "nombre": "Full Stack"
            }
        ]
        "#,
        )
}

#[get("/notas")]
async fn get_notas() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::json()).body(
        r#"
        [
            {
                "id": 1,
                "contenido": "Nota 1"
            },
            {
                "id": 2,
                "contenido": "Nota 2"
            }
        ]
        "#,
    )
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_usuarios).service(get_notas);
}
