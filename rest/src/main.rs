use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message, Filter, Rejection, path};
use reqwest::Method;

mod handler;
mod ws;

type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<RwLock<HashMap<String, Client>>>;

#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    let static_assets = warp::path("overlay").and(warp::fs::dir("overlay"))
    .map(|reply| {
        warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*")
    })
    .map(|reply| {
        warp::reply::with_header(reply, "Cross-Origin-Embedder-Policy", "require-corp")
    })
    .map(|reply| {
        warp::reply::with_header(reply, "Cross-Origin-Opener-Policy", "same-origin")
    });

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let webhook_route = path!("webhook")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::handle_webhook);

    let register = warp::path("register");
    let register_routes = register
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::register_handler)
        .or(register
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_clients(clients.clone()))
            .and_then(handler::unregister_handler));

    let publish = warp::path!("publish")
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::publish_handler);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(handler::ws_handler);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Access-Control-Allow-Origin", "Origin", "Accept", "X-Requested-With", "Content-Type"])
        .allow_methods(&[Method::GET, Method::POST]);

    let routes = health_route
        .or(static_assets)
        .or(webhook_route)
        .or(register_routes)
        .or(ws_route)
        .or(publish)
        .with(&cors);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 1338))
        .await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

