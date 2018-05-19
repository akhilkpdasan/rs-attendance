extern crate actix_web;
extern crate attendance_rs;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use attendance_rs::*;
use test_server::actix_web::http::Method;
use test_server::actix_web::test::TestServer;

pub fn create_server(urls: Vec<String>) -> (TestServer, DbPool) {
    use std::env;
    ::std::env::set_var(
        "DATABASE_URL",
        "postgres://postgres@localhost/attendance_management",
    );

    let database_url = env::var("DATABASE_URL").unwrap();
    let manager = r2d2_diesel::ConnectionManager::<diesel::PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Failed to build pool");

    let pool1 = pool.clone();

    let srv =
        TestServer::build_with_state(move || build_app_state(pool1.clone())).start(move |app| {
            app.middleware(Authorization {
                paths_to_ignore: urls.clone(),
            }).resource("/login", |r| r.method(Method::POST).with2(login))
                .resource("/logout", |r| r.method(Method::GET).h(logout))
                .resource("/whoami", |r| r.method(Method::GET).h(who_am_i))
                .resource("/register", |r| r.method(Method::POST).with2(register))
                .resource("/students", |r| {
                    r.method(Method::GET).with(get_all);
                    r.method(Method::POST).with2(new);
                })
                .resource("/students/{sid}", |r| {
                    r.method(Method::GET).with2(get_one);
                    r.method(Method::PUT).with3(update);
                    r.method(Method::DELETE).with2(delete);
                });
        });
    (srv, pool)
}
