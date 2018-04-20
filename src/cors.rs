use actix_web::middleware::cors;

pub fn options() -> cors::Cors {
    cors::Cors::build()
        .send_wildcard()
        .allowed_methods(vec!["DELETE", "GET", "POST", "PUT"])
        .max_age(3600)
        .finish()
}
