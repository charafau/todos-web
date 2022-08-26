use actix_web::{Responder, Result, HttpResponse, get, post, web::{self, Data}, App, HttpServer};
use controllers::todo_controller::todo_controller_config;
extern crate dotenv;

#[macro_use]
extern crate diesel;

mod models;
mod controllers;
mod db;
mod repositories;
mod schema;


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.configure(todo_controller_config);
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Started server on http://localhost:8080");

    let pool = db::establish_connection();


    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(pool.clone()))
        .service(index)
        .service(web::scope("/api").configure(api_config))
    })  
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
