use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use teloxide::prelude::*;

struct AppState {
    bot: Bot,
}

#[post("/process-webhook")]
async fn process_webhook(payload: String, data: web::Data<AppState>) -> impl Responder {
    println!("{}", payload);
    data.bot.send_message(ChatId(502462376), payload.clone());
    payload
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                bot: Bot::from_env(),
            }))
            .service(process_webhook)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
