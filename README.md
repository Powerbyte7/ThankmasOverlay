# Thankmas Donation Overlay

![](/rest/overlay/icon.png)

A Tiltify donation overlay service written in Rust for the anual Hytale Thankmas charity event. It uses Tiltify's Webhooks to get the latest campaign data and sends it to clients over Websocket.

## Project Structure

- The backend can be found in rest/, it handles Tiltify's webhooks and websocket communication
- Types from the Tiltify API can be found in tiltify/src/lib.rs
- The overlays can be found in overlay/
- You can find test data for the API in test_data/

## Building the project

When running locally, you can use cargo

```bash
cd rest
cargo run
```

For deployments I've set up a barbones Dockerfile. The resulting image is really small because it uses `FROM scratch`. 

```bash
docker build .
```

## Testing the API

On Linux in Bash:

```shell
curl --header "Content-Type: application/json" --request POST --data "tiltify/testdata/campaign2.json" http://localhost:8080/campaign
curl --header "Content-Type: application/json" --request POST --data "tiltify/testdata/donation.json" http://localhost:8080/donation
```

On Windows in Powershell:

```powershell
curl -Uri 'http://localhost:8080/campaign' -Method Post -ContentType 'application/json' -InFile 'tiltify/testdata/campaign2.json'
curl -Uri 'http://localhost:8080/donation' -Method Post -ContentType 'application/json' -InFile 'tiltify/testdata/donation.json'
```