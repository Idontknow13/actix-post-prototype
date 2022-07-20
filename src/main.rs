use actix_web::{HttpServer, App, middleware::{NormalizePath, Logger}, get, post, web, Responder, HttpResponse};
use std::{io::Result as IOResult, sync::Mutex};
use serde::Deserialize;

const APP_URI: &str = "127.0.0.1";
const PORT: u16 = 5555;

#[actix_web::main]
async fn main() -> IOResult<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let state = web::Data::new(Mutex::new(0usize));
    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::trim())
            .wrap(Logger::default())
            .app_data(state.clone())
            .service(get_count)
            .service(add_count)
    })
    .bind((APP_URI, PORT))?
    .run()
    .await
}

//* ---- Request Schema ---- *//

#[derive(Deserialize)]
struct CountRequest {
    count_to: usize
}

//* ---- Endpoints ---- *//

#[get("/count")]
async fn get_count(state: web::Data<Mutex<usize>>) -> impl Responder {
    let count = state.lock().unwrap();
    HttpResponse::Ok().body(format!("The current count is: {}\n", count))
}

#[post("/count")]
async fn add_count(state: web::Data<Mutex<usize>>, req: web::Form<CountRequest>) -> impl Responder {
    let mut count = state.lock().unwrap();
    let req = req.into_inner();
    
    *count += req.count_to;

    HttpResponse::Ok().body(format!("{} added to count.", req.count_to))
}
