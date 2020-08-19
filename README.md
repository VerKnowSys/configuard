# WireGuard Service


## What Am I Looking At Here

This is HTTP service that manages WireGuard-server configuration via API.


## Assumptions:

- small/ medium private network is fine for me

- production system is svdOS (based on HardnedBSD/FreeBSD 12.x) x86_64 with ZFS-on-root.

- by default `/10` netmask is used for access to whole default private network: `100.64.0.0`

- by default max defined workstations: `253` (.1.2 => .1.254)

- by default max defined instances: `64009` (.2.2 => .254.254)



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


## Usage

### Workstation side, Darwin only:

- Install latest version of Wireguard tools:
  `brew install wireguard-tools`

- Fetch your configuration from your configuard server and store it in local configuration file:
  `curl -XPOST … > /usr/local/etc/wireguard/wg0.conf`

- Use system-specific configuration:
  `cp config/config.toml.$(uname) config/config.toml`

- Run client script on your workstation:
  Install Wireguard GUI from AppStore or `bin/wg-workstation` or `wg-quick up wg0` (background)

### Remote-host-instance side, Linux or FreeBSD only:

- Install latest version of Wireguard tools:
  `apt install -y wireguard-tools wireguard-dkms` (Ubuntu 18.x) or `s i Wireguard-tools` (FreeBSD 12.x + svdOS)

- Fetch your configuration from your configuard server and store it in local configuration file:
  `curl -XPOST … > /etc/wireguard/wg0.conf` (Ubuntu 18.x) or `/Services/Wireguard-tools/wg0.conf` (FreeBSD 12.x + svdOS)

- Use system-specific configuration:
  `cp config/config.toml.$(uname) config/config.toml`

- Run client script on your instance:
  `bin/wg-instance` (foreground) or `wg-quick up wg0` (background)

### Configuard server, FreeBSD only:

- Start service locally on `127.1:8000`:
  `ROCKET_ENV=production ROCKET_address="localhost" ROCKET_port=8000 cargo run`

- Generate workstation configuration (with address re-use - both auth keys are always generated, but existing address will not change):
  `curl -X POST http://localhost:8000/your-configured-uuid/wireguard/workstation/dmilith`

- Generate server-instance configuration (also with address re-use):
  `curl -X POST http://localhost:8000/your-configured-uuid/wireguard/instance/my-server`


## License:

- BSD

- MIT

- Apache2
