extern crate actix_web;
extern crate attendance_rs;
#[macro_use]
extern crate serde_json;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use actix_web::http::*;
use attendance_rs::schema::*;
use diesel::prelude::*;

mod test_server;

fn get_token(srv: &mut actix_web::test::TestServer) -> String {
    let request = srv.client(Method::POST, "/login")
        .timeout(std::time::Duration::new(120, 0))
        .json(json!({"username":"test", "password":"test"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    let token = response.cookie("token").unwrap().value();
    token.to_string()
}

#[test]
fn update_test() {
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

        diesel::insert_into(students::table)
            .values((
                students::id.eq("s32"),
                students::name.eq("bedki"),
                students::roll_no.eq(32),
                students::attendance.eq(12.0),
            ))
            .execute(conn)
            .unwrap();

        diesel::insert_into(students::table)
            .values((
                students::id.eq("s36"),
                students::name.eq("yogesh"),
                students::roll_no.eq(36),
                students::attendance.eq(16.0),
            ))
            .execute(conn)
            .unwrap();
    }

    let token = get_token(&mut srv);

    //update student bad input
    let body = json!({"attendance": "float"});
    let request = srv.client(Method::PUT, "/students/s32")
        .header("Cookie", format!("token={}", token))
        .json(body)
        .unwrap();

    let response = srv.execute(request.send()).unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    //update student
    let body = json!({"attendance": 33.33});
    let request = srv.client(Method::PUT, "/students/s32")
        .header("Cookie", format!("token={}", token))
        .json(body)
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    //update non-existing student
    let body = json!({"attendance": 33.33});
    let request = srv.get()
        .uri(srv.url("/students/s100"))
        .header("Cookie", format!("token={}", token))
        .json(body)
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    //update without token fails
    let body = json!({"attendance": 10.33});
    let request = srv.client(Method::PUT, "/students/s32").json(body).unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
