extern crate actix;
extern crate actix_web;
extern crate attendance_rs;
extern crate env_logger;

use actix_web::server;
use attendance_rs::create_app;

fn main() {
    let sys = actix::System::new("example");
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    
    env_logger::init();
    server::new(create_app)
        .bind("192.168.0.103:8088")
        .expect("Couldn't bind server to addrsess")
        .start();
        
    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}
