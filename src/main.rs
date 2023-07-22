use actix_web::{post, web, App, HttpServer, Responder};
use serde_json::Value;
use teloxide::{
    payloads,
    prelude::*,
    types::{ParseMode, Recipient},
};

const CHAT_ID: i64 = -1001986164831;
const THREAD_ID: i32 = 13915;

struct AppState {
    bot: Bot,
}

async fn bot_send_message(bot: &Bot, text: String) -> <Bot as Requester>::SendMessage {
    bot.send_message_to_thread(ChatId(CHAT_ID), THREAD_ID, text)
}

#[post("/issues")]
async fn process_issue(payload: String, data: web::Data<AppState>) -> impl Responder {
    let webhook_data: Value = serde_json::from_str(&payload.as_str()).unwrap();
    let text = format!(
        "Issue is <b>{}</b>, for more details see {}",
        webhook_data["action"].as_str().unwrap_or(""),
        webhook_data["issue"]["html_url"].as_str().unwrap_or("")
    );
    bot_send_message(&data.bot, text).await;
    ""
}

#[post("/pr-review-comment")]
async fn process_pr_review_comment(payload: String, data: web::Data<AppState>) -> impl Responder {
    let webhook_data: Value = serde_json::from_str(&payload.as_str()).unwrap();
    let text = format!(
        "New review comment for PR: {}\n\n<i>{}</i>\n\nBy {}\n\n{}",
        webhook_data["pull_request"]["html_url"],
        webhook_data["comment"]["body"],
        webhook_data["comment"]["user"]["name"],
        webhook_data["comment"]["html_url"],
    );

    bot_send_message(&data.bot, text).await;
    ""
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                bot: Bot::from_env(),
            }))
            .service(process_issue)
            .service(process_pr_review_comment)
    })
    .bind(("0.0.0.0", 8080))?
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
