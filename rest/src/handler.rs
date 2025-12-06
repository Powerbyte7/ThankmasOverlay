use crate::{ws, Client, Clients, Result};
use serde::{Deserialize, Serialize};
use tiltify::{Campaign, Donation, TiltifyReponse};
use tokio::sync::RwLock;
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, ws::Message, Reply};

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    user_id: usize,
    topics: Vec<String>
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse<'a> {
    url: String,
    latest_data: Option<&'a Campaign>,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    topic: String,
    user_id: Option<usize>,
    message: String,
}

// Keep track of the last known campaign state, useful when we need to reload the overlay
static CAMPAIGN_DATA: RwLock<Option<Campaign>> = RwLock::<Option<Campaign>>::const_new(None);

pub async fn handle_campaign(body: TiltifyReponse<Campaign>, clients: Clients) -> Result<impl Reply> {
    let campaign = body.data;
    println!("New total: ${}", campaign.amount_raised.value);

    let message_json = serde_json::to_string(&campaign).unwrap();
    
    {
        let mut write_guard = CAMPAIGN_DATA.write().await;
        write_guard.replace(campaign);
    }

    publish_handler(
        Event {
            topic: "campaign".to_string(),
            user_id: None,
            message: message_json,
        },
        clients,
    )
    .await
    .expect("Publishing campaign data failed!");

    Ok::<_, warp::Rejection>(warp::reply())
}

pub async fn handle_donation(body: TiltifyReponse<Donation>, clients: Clients) -> Result<impl Reply> {
    let donation = body.data;
    println!("Got donation: ${}", donation.amount.value);

    publish_handler(
        Event {
            topic: "donation".to_string(),
            user_id: None,
            message: serde_json::to_string(&donation).unwrap(),
        },
        clients,
    )
    .await
    .expect("Publishing donation data failed!");

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

    register_client(uuid.clone(), user_id, clients, body.topics).await;

    Ok(json(&RegisterResponse {
        url: format!("../ws/{}", uuid),
        latest_data: CAMPAIGN_DATA.read().await.as_ref()
    }))
}

async fn register_client(id: String, user_id: usize, clients: Clients, topics: Vec<String>) {
    clients.write().await.insert(
        id,
        Client {
            user_id,
            topics: topics,
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
