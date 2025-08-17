use actix_web::{App, HttpResponse, HttpServer, Responder};

#[actix_web::get("/")]
async fn hello() -> impl Responder {
    println!("Received a request at /");
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = String::from("127.0.0.1:8080");
    println!("Running server on http://{address}");
    HttpServer::new(|| {
        App::new().service(hello)
    })
    .bind(address)?
    .run()
    .await
}