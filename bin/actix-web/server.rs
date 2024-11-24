use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Deserialize)]
struct Info {
    name: String,
    age: u8,
}

#[get("/info")]
async fn info(query: web::Query<Info>) -> impl Responder {
    let info = query.into_inner();
    HttpResponse::Ok().body(format!("Name: {}, Age: {}", info.name, info.age))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // curl -XGET "127.0.0.1:8080"
            .service(hello)
            // curl -XPOST -d "hello world" "127.0.0.1:8080/echo"
            .service(echo)
            // curl -XGET "127.0.0.1:8080/info?name=John&age=20"
            .service(info)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
