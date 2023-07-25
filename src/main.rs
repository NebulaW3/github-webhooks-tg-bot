use actix_web::{post, web, App, HttpServer, Responder};
use github_webhooks_tg_bot::{
    get_issue_text, get_pr_review_comment_text, get_pr_review_text, get_pr_text,
};
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

fn bot_send_message(bot: &Bot, text: String) -> <Bot as Requester>::SendMessage {
    bot.send_message_to_thread(ChatId(CHAT_ID), THREAD_ID, text)
}

#[post("/issues")]
async fn process_issue(payload: String, data: web::Data<AppState>) -> impl Responder {
    let text = get_issue_text(payload);
    bot_send_message(&data.bot, text).await;
    ""
}

#[post("/pr")]
async fn process_pr(payload: String, data: web::Data<AppState>) -> impl Responder {
    let text = get_pr_text(payload);
    bot_send_message(&data.bot, text).await;
    ""
}

#[post("/pr-review")]
async fn process_pr_review(payload: String, data: web::Data<AppState>) -> impl Responder {
    let text = get_pr_review_text(payload);
    bot_send_message(&data.bot, text).await;
    ""
}

#[post("/pr-review-comment")]
async fn process_pr_review_comment(payload: String, data: web::Data<AppState>) -> impl Responder {
    let text = get_pr_review_comment_text(payload);
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
            .service(process_pr)
            .service(process_pr_review)
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
