use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn index(req: HttpRequest, req_body: String) -> impl Responder {
    println!("====================================");
    println!("method: {:?}", req.method());
    println!("uri: {:?}", req.uri());
    println!("headers:");
    for (name, value) in req.headers() {
        println!("  {}: {:?}", name, value);
    }
    println!("body: {}", req_body);
    println!("====================================");

    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().default_service(web::route().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
