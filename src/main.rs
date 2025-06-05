use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use std::sync::Arc;
use std::thread;
use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::prelude::{Message, Requester};

struct AppState {
    bot: Bot,
}
#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let bot = Bot::new("7994351313:AAEj1BLqnadoD4UutVhLaD5ALYuGzO-KCTM");
    let shared_bot = Arc::new(AppState { bot });

    let app = Router::new()
        .route("/order", post(handle_order))
        .with_state(Arc::clone(&shared_bot));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_order(state: State<Arc<AppState>>, Json(order): Json<order_data>) {
    println!("{:?}", order);
    state
        .bot
        .send_message(ChatId(1771551917), order.to_telegram())
        .await
        .unwrap();
}

#[derive(Deserialize, Debug)]
struct order_data {
    server_type: u8,
    rent_type: u8,
    slots: u8,
    name: String,
    email: String,
    project: String,
}

impl order_data {
    pub(crate) fn to_telegram(&self) -> String {
        let msg = format!(
            "Новая заявка на хостинг:
{0}, {1},
Количество слотов: {2},
Тип сервера: {3},
Тип аренды: {4},
Цель: {5}
        ",
            self.name,
            self.email,
            self.slots,
            if self.server_type == 0 {
                "Rasberry"
            } else {
                "Unlimited"
            },
            if self.server_type == 0 {
                "Неделя"
            } else {
                "Месяц"
            },
            self.project
        );
        msg
    }
}
