extern crate actix_web;
#[macro_use]
extern crate serde_json;
extern crate attendance_rs;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use actix_web::{http::*, HttpMessage};
use attendance_rs::schema::*;
use diesel::prelude::*;

mod test_server;

#[test]
fn user_test() {
    let urls = vec!["/login".to_string(), "/register".to_string()];
    let (mut srv, pool) = test_server::create_server(urls);

    {
        let conn: &PgConnection = &pool.get().expect("Cannot get connection");
        conn.begin_test_transaction()
            .expect("Failed to begin test transaction");

        diesel::insert_into(users::table)
            .values((
                users::username.eq("test"),
                users::password.eq("$2y$12$mBpFsVrXcCegx9en5cTDjeSfBXT4jbk4WRuijk/O2D1KgcKOiTE7O"),
                users::email.eq("test"),
            ))
            .execute(conn)
            .unwrap();
    }

    //login with incorrect password
    let request = srv.client(Method::POST, "/login")
        .timeout(std::time::Duration::new(120, 0))
        .json(json!({"username":"test", "password":"bad_pass"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    //login non-existing user
    let request = srv.client(Method::POST, "/login")
        .timeout(std::time::Duration::new(120, 0))
        .json(json!({"username":"no_user", "password":"test"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    //login
    let request = srv.client(Method::POST, "/login")
        .timeout(std::time::Duration::new(120, 0))
        .json(json!({"username":"test", "password":"test"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let token = response.cookie("token").unwrap().value().to_string();

    //register
    let request = srv.client(Method::POST, "/register")
        .json(json!({"username":"test3", "password":"test3", "email":"test3"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    //register bad input
    let request = srv.client(Method::POST, "/register")
        .json(json!({"username":"bad_input", "password":123, "email":"test2"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    //register username exists
    let request = srv.client(Method::POST, "/register")
        .json(json!({"username":"test", "password":"test", "email":"test"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);

    //who_am_i returns username
    let request = srv.client(Method::GET, "/whoami")
        .header("Cookie", format!("token={}", token))
        .finish()
        .unwrap();
    let response = srv.execute(request.send()).unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let username = srv.execute(response.body()).unwrap();
    assert_eq!(username, "test");

    //logout
    let request = srv.get()
        .uri(srv.url("/logout"))
        .header("Cookie", format!("token={}", token))
        .finish()
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    //token expires in year 1970
    let c = response.cookie("token").unwrap();

    assert_eq!(c.expires().map(|t| t.tm_year), Some(70));
}
