use actix_web::middleware::cors;
use http::header;

pub fn options() -> cors::Cors {
    cors::Cors::build()
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600)
        .finish()
        .expect("Can not create CORS middleware")
}
