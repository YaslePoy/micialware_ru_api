use axum::extract::State;
use axum::http::Method;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use std::sync::Arc;
use teloxide::prelude::Requester;
use teloxide::prelude::*;
use teloxide::Bot;
use tower_http::cors::{Any, CorsLayer};

struct AppState {
    bot: Bot,
}
#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let bot = Bot::new("7994351313:AAEj1BLqnadoD4UutVhLaD5ALYuGzO-KCTM");
    let shared_bot = Arc::new(AppState { bot });
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST]).allow_headers(Any);
    // Allow all origins (open policy)
    let app = Router::new()
        .route("/order", post(handle_order))
        .with_state(Arc::clone(&shared_bot))
        .layer(cors);
    

    println!("Server started!!!");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:81").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_order(state: State<Arc<AppState>>, Json(order): Json<OrderData>) {
    println!("{:?}", order);
    state
        .bot
        .send_message(ChatId(1771551917), order.to_telegram())
        .await
        .unwrap();
}

#[derive(Deserialize, Debug)]
struct OrderData {
    server_type: u8,
    rent_type: u8,
    slots: u8,
    name: String,
    email: String,
    project: String,
}

impl OrderData {
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
                "Raspberry"
            } else {
                "Unlimited"
            },
            if self.rent_type == 0 {
                "Неделя"
            } else {
                "Месяц"
            },
            self.project
        );
        msg
    }
}
