use actix_web::{post, web, App, HttpServer, Responder};
use serde_json::{Result, Value};
use teloxide::{
    payloads,
    prelude::*,
    types::{ParseMode, Recipient},
};

struct AppState {
    bot: Bot,
}

#[post("/issues")]
async fn process_webhook(payload: String, data: web::Data<AppState>) -> impl Responder {
    let webhook_data: Value = serde_json::from_str(&payload.as_str()).unwrap();
    let text = format!(
        "Issue is <b>{}</b>, for more details see {}",
        webhook_data["action"].as_str().unwrap_or(""),
        webhook_data["issue"]["html_url"].as_str().unwrap_or("")
    );
    data.bot
        .send_message_to_thread(ChatId(-1001986164831), 13915, text)
        .await;
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
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}

trait SendToThread: Requester {
    fn send_message_to_thread<C, T>(
        &self,
        chat_id: C,
        thread_id: i32,
        text: T,
    ) -> Self::SendMessage
    where
        C: Into<Recipient>,
        T: Into<String>;
}

impl SendToThread for Bot {
    fn send_message_to_thread<C, T>(&self, chat_id: C, thread_id: i32, text: T) -> Self::SendMessage
    where
        C: Into<Recipient>,
        T: Into<String>,
    {
        Self::SendMessage::new(
            self.clone(),
            payloads::SendMessage::new(chat_id, text)
                .message_thread_id(thread_id)
                .parse_mode(ParseMode::Html),
        )
    }
}
