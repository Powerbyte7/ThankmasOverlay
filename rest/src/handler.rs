use crate::{ws, Client, Clients, Result};
use serde::{Deserialize, Serialize};
use tiltify::Campaign;
use tokio::sync::RwLock;
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, ws::Message, Reply};

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    user_id: usize,
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    url: String,
    latest_data: Option<Campaign>,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    topic: String,
    user_id: Option<usize>,
    message: String,
}

static CAMPAIGN_DATA: RwLock<Option<Campaign>> = RwLock::<Option<Campaign>>::const_new(None);

pub async fn handle_webhook(body: Campaign, clients: Clients) -> Result<impl Reply> {
    let amount_raised = body.data.amount_raised.as_ref().map(|x| x.value.clone());
    if let Some(amount) = amount_raised {
        println!("New total: ${}", amount);
    }

    let mut write_guard = CAMPAIGN_DATA.write().await;
    write_guard.replace(body.clone());

    publish_handler(
        Event {
            topic: "donation".to_string(),
            user_id: None,
            message: serde_json::to_string(&body).unwrap(),
        },
        clients,
    )
    .await
    .expect("Publishing event data failed!");

    Ok::<_, warp::Rejection>(warp::reply())
}

pub async fn publish_handler(body: Event, clients: Clients) -> Result<impl Reply> {
    clients
        .read()
        .await
        .iter()
        .filter(|(_, client)| match body.user_id {
            Some(v) => client.user_id == v,
            None => true,
        })
        .filter(|(_, client)| client.topics.contains(&body.topic))
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::text(body.message.clone())));
            }
        });

    Ok(StatusCode::OK)
}

pub async fn register_handler(body: RegisterRequest, clients: Clients) -> Result<impl Reply> {
    let user_id = body.user_id;
    let uuid = Uuid::new_v4().as_simple().to_string();

    register_client(uuid.clone(), user_id, clients).await;
    Ok(json(&RegisterResponse {
        url: format!("../ws/{}", uuid),
        latest_data: CAMPAIGN_DATA.read().await.clone(),
    }))
}

async fn register_client(id: String, user_id: usize, clients: Clients) {
    clients.write().await.insert(
        id,
        Client {
            user_id,
            topics: vec![String::from("donation")],
            sender: None,
        },
    );
}

pub async fn unregister_handler(id: String, clients: Clients) -> Result<impl Reply> {
    clients.write().await.remove(&id);
    Ok(StatusCode::OK)
}

pub async fn ws_handler(ws: warp::ws::Ws, id: String, clients: Clients) -> Result<impl Reply> {
    let client = clients.read().await.get(&id).cloned();
    match client {
        Some(c) => Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, id, clients, c))),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn health_handler() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}
