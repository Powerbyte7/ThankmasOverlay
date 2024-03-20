# Thankmas Donation Overlay

A Tiltify donation overlay service written in Rust for the anual Hytale Thankmas charity event. It uses Tiltify's Webhooks to get the latest campaign data and sends it to clients over Websocket.

## Project Structure

- The backend can be found in rest/, it handles Tiltify's webhooks and websocket communication
- Types from the Tiltify API can be found in tiltify/src/lib.rs
- The Godot overlay can be found in overlay/
- You can find test data for the API in test_data/

## Testing the API

```shell
curl --header "Content-Type: application/json" --request POST --data "@test_data.json" http://localhost:80/webhook
```