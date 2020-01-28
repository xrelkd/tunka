#!/bin/sh

echo "Starting OpenVPN client..."
openvpn --config /config.ovpn \
    --auth-nocache \
    --daemon

echo "Starting Brook..."
brook --version
exec brook socks5 -l :8118 -i 0.0.0.0
