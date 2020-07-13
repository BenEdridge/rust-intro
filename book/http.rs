use actix_web::body::Body;
use actix_web::{client, web, App, HttpResponse, HttpServer, Responder};
use std::io::Error;
use std::io::ErrorKind;
use std::sync::Mutex;
// use tokio::sync::Mutex;


struct DataStore {
    req_count: Mutex<i32>
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    println!("Server running on port 8088");

    let req_count = web::Data::new(DataStore {
        req_count: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(req_count.clone())
            .route("/", web::get().to(index))
            .route("/get", web::get().to(getter))
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}

async fn index(data: web::Data<DataStore>) -> impl Responder {

    let mut counter = data.req_count.lock().unwrap();
    *counter += 1;

    println!("Requests made: {}", counter);
    HttpResponse::Ok()
    .content_type("application/txt")
    .body("Hello world!")
}

async fn getter(data: web::Data<DataStore>) -> impl Responder {

    let mut counter = data.req_count.lock().unwrap();
    *counter += 1;

    println!("Requests made: {}", counter);
    let result = req().await;
    HttpResponse::Ok()
    .content_type("application/json")
    .body(result.unwrap())
}

async fn req() -> reqwest::Result<String> {
    let body = reqwest::get("https://www.reddit.com/r/LifeProTips.json")
    .await?
    .text()
    .await?;
    Ok(body)
}
