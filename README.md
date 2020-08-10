# WireGuard Service


## Configuration:


`config/config.toml` contains:

```toml
uuid = "791A455F-DD50-4A61-B535-92920EA34848" # uuid is unique part of a request path
main_net = "100.64" # main network used on wireguard server
main_net_mask = "/10" # default mask for main_net
server_public_ip = "1.2.3.4" # your external wireguard server ipv4 address
server_port = 51194 # some open port on server endpoint
wireguard_bin = "/Software/Wireguard-tools/exports/wg" # full path to wg utility
wireguard_conf = "/Services/Wireguard-tools/wg0.conf" # full path to wg0.conf
```


## Usage:

- Start service locally on `127.1:8000`:
  `ROCKET_address="127.0.0.1" ROCKET_port=8000 cargo run`

- Generate new workstation configuration:
  `curl -X POST http://localhost:8000/your-configured-uuid/wireguard/new/workstation/dmilith`

- Generate new server-instance configuration:
  `curl -X POST http://localhost:8000/your-configured-uuid/wireguard/new/instance/my-server`
