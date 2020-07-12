#!/bin/sh

echo "Starting OpenVPN client..."
openvpn --config /config.ovpn \
  --auth-nocache \
  --daemon

echo "Starting Tunelo..."
exec tunelo socks-server --ip 0.0.0.0 --port 8118
