use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

#[post("/process-webhook")]
async fn process_webhook(payload: String) -> impl Responder {
    payload
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(process_webhook)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

