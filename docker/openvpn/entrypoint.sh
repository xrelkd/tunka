#!/bin/sh

echo "Starting OpenVPN client as a daemon"
openvpn --config /config.ovpn \
  --auth-nocache \
  --daemon

echo "Starting Tunelo $(tunelo version)"
exec tunelo socks-server --ip 0.0.0.0 --port 8118
