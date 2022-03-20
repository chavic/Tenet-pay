#[macro_use]
extern crate serde_derive;
extern crate juniper;
extern crate log;

mod auth;
mod errors;
mod graphql;
mod jwt;
mod router;

mod settings;

use crate::jwt::manager::*;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use mysql::*;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let settings = {
        use settings::Settings;
        Settings::new().unwrap()
    };

    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    let demo_issuer = "victor cha".to_string();
    let demo_phone = "09856786756465".to_string();
    println!(
        "Demo Issuer: {}, Demo Phone Number: {}",
        demo_issuer, demo_phone
    );
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    let code = create_token(demo_issuer, demo_phone, 30);
    println!("Code: {:?}", code);
    let decoded = decode_token(&code.unwrap());
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("{:?}", decoded);
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    // mysql
    let opts = OptsBuilder::new();
    let db = Arc::new(Pool::new(opts));
    // graphql
    let schema = Arc::new(crate::graphql::create_schema());

    // Server port
    let port = settings.server.port;
    // let domain = settings.server.domain;
    // HTTP server
    let server = HttpServer::new(move || {
        let cors = Cors::permissive().allowed_origin("http://localhost:8080");
        let schema: web::Data<graphql::Schema> = schema.clone().into();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(schema)
            .app_data(db.clone())
            .app_data(web::Data::new(settings.clone()))
            .configure(graphql::route)
            .configure(router::frontend_routes)
    })
    .bind(("127.0.0.1", port))
    .unwrap()
    .run();

    eprintln!("Listening on 127.0.0.1:{}", port);

    server.await
}
