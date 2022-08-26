use actix_web::{Responder, Result, HttpResponse, get, post, web, App, HttpServer};
use controllers::todo_controller::todo_controller_config;


mod models;
mod controllers;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("hey there!")
// }

fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.configure(todo_controller_config);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Started server on http://localhost:8080");

    HttpServer::new(|| {
        App::new()
        .service(index)
        // .service(echo)
        .service(web::scope("/api").configure(api_config))
        // .route("/hey", web::get().to(manual_hello))
    })  
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
