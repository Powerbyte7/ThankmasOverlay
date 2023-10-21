# Thankmas Donation Overlay

A Tiltify donation overlay service written in Rust.

## Project Structure

- The backend can be found in rest/, it handles Tiltify's webhooks and websocket communication
- Types from the Tiltify API can be found in tiltify/src/lib.rs
- The Godot overlay can be found in overlay/


## Testing the API

```shell
curl --header "Content-Type: application/json" --request POST --data "@test_data.json" http://localhost:1338/webhook
```
