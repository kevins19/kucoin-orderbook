mod orderbook;
use crate::orderbook::*;

use futures_util::{SinkExt, StreamExt};
use tokio::time::{sleep, Duration};
use tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::connect_async;

// Tokens used for connecting to the websocket stream
const ENDPOINT : &str = "wss://ws-api-spot.kucoin.com/";
const TOKEN : &str = "2neAiuYvAU61ZDXANAGAsiL4-iAExhsBXZxftpOeh_55i3Ysy2q2LEsEWU64mdzUOPusi34M_wGoSf7iNyEWJ8adAEYq1czsGvpzdfennGKUF1ngHlXCz9iYB9J6i9GjsxUuhPw3Blq6rhZlGykT3Vp1phUafnulOOpts-MEmEEBqZfO5rYmFS_2ljqSjSZLJBvJHl5Vs9Y=.JAH4MNtFbhCBMoisdrQSuQ==";

#[tokio::main]
async fn main() {

    // Connect to websocket stream
    println!("WebSocket Token: {}", TOKEN);
    println!("Connecting to WebSocket endpoint: {}", ENDPOINT);

    let ws_url = format!("{}?token={}", ENDPOINT, TOKEN);
    let request = ws_url.into_client_request().unwrap();
    let (ws, _) = connect_async(request).await.expect("Failed to connect to WebSocket");
    let (mut ws_writer, mut ws_stream) = ws.split();
    println!("Connected to WebSocket!");

    // Subscription message to subscribe to level 2 market incremental data
    let subscribe_message = serde_json::json!({
        "type": "subscribe",
        "topic": "/contractMarket/level2:ETHUSDTM",
        "response": true
    });
    /// Send subscription message to exchange
    ws_writer.send(tokio_tungstenite::tungstenite::protocol::Message::Text(subscribe_message.to_string(),))
        .await
        .expect("Failed to send subscription message");
    println!("Subscribed to market data.");

    // Spawn a task that periodically sends heartbeats to the exchange
    tokio::spawn(async move {
        loop {
            if let Err(e) = ws_writer.send(Message::Ping(vec![])).await {
                eprintln!("Failed to send heartbeat: {:?}", e);
                break;
            }
            println!("Sent Heartbeat.");
            sleep(Duration::from_secs(20)).await;
        }
    });

    // Initialize an empty orderbook
    let mut ob = Orderbook::new();

    let mut cnt = 0;            // used to restrict output for greater readability
    let slow_output_on = true;  // toggle to true to enable slower output

    // Process market data as it streams in
    while let Some(msg) = ws_stream.next().await {
        let msg = msg.expect("Failed to read WebSocket message");
        if let Message::Text(text) = msg {
            // println!("{}", text);
            let parsed: serde_json::Value = serde_json::from_str(&text).expect("Failed to parse JSON");
            if parsed["type"] == "pong" {
                println!("Received Pong.");
            } 
            else if let Some(change) = parsed["data"]["change"].as_str() {
                let parts: Vec<&str> = change.split(',').collect();
                if let [price_s, direction_s, quantity_s] = &parts[..] {
                    let price = ordered_float::OrderedFloat(price_s.parse().unwrap());
                    let quantity = quantity_s.parse().unwrap();
                    let direction = if direction_s.to_lowercase() == "buy" {
                        Direction::Buy
                    } else {
                        Direction::Sell 
                    };
                    let inc = Incremental {
                        price,
                        quantity,
                        direction
                    };
                    ob.process(&inc);
                    cnt += 1;
                    if !slow_output_on || cnt % 1000 == 0 {
                        ob.display();
                    }
                }
            } 
        }
    }
}
